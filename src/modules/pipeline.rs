//! Fixed pipeline execution with proper handle management
//! 
//! Key fixes:
//! 1. Pipe handles are transferred by ownership, not cloned
//! 2. Simplified Windows command adaptation - no automatic cmd.exe wrapping
//! 3. Proper cleanup to avoid hanging processes

use std::process::{Command, Stdio, Child};
use std::io::{self, Write};
use std::rc::Rc;

// Import os_pipe for creating real OS pipes
use os_pipe::{pipe, PipeReader, PipeWriter};

use crate::modules::shell::Shell;
use crate::modules::expansion;
use crate::modules::parser;

/// Check if a line contains a pipeline
pub fn has_pipeline(line: &str) -> bool {
    // We need to check if there's a pipe that's not part of ||
    let chars = line.chars().collect::<Vec<char>>();
    let mut i = 0;
    
    while i < chars.len() {
        if chars[i] == '|' {
            if i + 1 < chars.len() && chars[i + 1] == '|' {
                // Skip over ||
                i += 2;
            } else {
                // Found a single pipe
                return true;
            }
        } else {
            i += 1;
        }
    }
    
    false
}

/// Execute a single pipeline command (serial mode - fallback)
fn execute_single_pipeline_command(shell: &mut Shell, cmd_str: &str, stdin_data: Option<Vec<u8>>, is_last: bool) -> (i32, Option<Vec<u8>>) {
    // Parse the command
    let (cmd, args) = parser::parse_command(cmd_str);
    if cmd.is_empty() {
        return (0, None);
    }
    
    // Expand variables in arguments
    let args = args.iter()
        .map(|arg| expansion::expand_variables_simple(shell, arg))
        .collect::<Vec<String>>();
    
    if shell.builtin_registry.has_builtin(&cmd) {
        // For builtins in pipeline, we need to handle input/output
        let registry = Rc::clone(&shell.builtin_registry);
        let status = registry.execute_builtin_in_pipeline(shell, &cmd, &args, is_last);
        
        // Builtins in pipeline context - for simplicity, we'll return empty output
        // In a real implementation, builtins should capture their output
        (status, None)
    } else {
        // External command
        let path = match shell.find_in_path(&cmd) {
            Some(path) => path,
            None => {
                eprintln!("{}: command not found", cmd);
                return (127, None);
            }
        };
        
        let mut command = Command::new(path);
        for arg in &args {
            command.arg(arg);
        }
        command.current_dir(&shell.current_dir);
        command.envs(&shell.env_vars);
        
        // Set up stdin
        if stdin_data.is_some() {
            command.stdin(Stdio::piped());
        }
        
        // Set up stdout
        if !is_last {
            command.stdout(Stdio::piped());
        }
        
        match command.spawn() {
            Ok(mut child) => {
                // Write stdin data if provided
                if let Some(data) = stdin_data {
                    if let Some(mut stdin) = child.stdin.take() {
                        stdin.write_all(&data).ok();
                    }
                }
                
                match child.wait_with_output() {
                    Ok(output) => {
                        let status = output.status.code().unwrap_or(128);
                        let output_data = if !is_last && !output.stdout.is_empty() {
                            Some(output.stdout)
                        } else if is_last && output.status.success() && !output.stdout.is_empty() {
                            // Print output of last command
                            io::stdout().write_all(&output.stdout).ok();
                            None
                        } else {
                            None
                        };
                        (status, output_data)
                    }
                    Err(e) => {
                        eprintln!("{}: {}", cmd, e);
                        (1, None)
                    }
                }
            }
            Err(e) => {
                eprintln!("{}: {}", cmd, e);
                (1, None)
            }
        }
    }
}

/// Execute a pipeline (commands connected with |) - SERIAL implementation (fallback)
pub fn execute_pipeline_serial(shell: &mut Shell, line: &str) -> i32 {
    let commands: Vec<&str> = line.split('|')
        .map(|s| s.trim())
        .collect();
    
    if commands.is_empty() {
        return 0;
    }
    
    let mut last_status = 0;
    let mut last_output: Option<Vec<u8>> = None;
    
    // Execute commands sequentially, passing output between them
    for (i, cmd_str) in commands.iter().enumerate() {
        let is_last = i == commands.len() - 1;
        
        let (status, output) = execute_single_pipeline_command(
            shell, 
            cmd_str, 
            last_output.take(), 
            is_last
        );
        
        last_status = status;
        last_output = output;
    }
    
    last_status
}

/// Pipe pair for connection between two commands
struct PipePair {
    reader: PipeReader,
    writer: PipeWriter,
}

impl PipePair {
    fn new() -> Result<Self, String> {
        match pipe() {
            Ok((reader, writer)) => Ok(Self { reader, writer }),
            Err(e) => Err(format!("Failed to create pipe: {}", e)),
        }
    }
    
    /// Split into reader and writer (consumes self)
    fn split(self) -> (PipeReader, PipeWriter) {
        (self.reader, self.writer)
    }
}

/// Pipeline execution context with proper handle management
struct PipelineContext {
    /// Child processes
    children: Vec<Child>,
    /// Pipe pairs for connections between commands
    /// These are stored until needed, then ownership is transferred to commands
    pipe_pairs: Vec<PipePair>,
}

impl PipelineContext {
    fn new() -> Self {
        Self {
            children: Vec::new(),
            pipe_pairs: Vec::new(),
        }
    }
    
    /// Create pipes for N commands (N-1 pipes needed)
    fn create_pipes(&mut self, num_commands: usize) -> Result<(), String> {
        if num_commands <= 1 {
            return Ok(());
        }
        
        self.pipe_pairs.clear();
        for _ in 0..(num_commands - 1) {
            self.pipe_pairs.push(PipePair::new()?);
        }
        
        Ok(())
    }
    
    /// Spawn a command in the pipeline with proper pipe ownership transfer
    fn spawn_command(
        &mut self,
        shell: &Shell,
        cmd: &str,
        args: &[String],
        command_index: usize,
        total_commands: usize,
    ) -> Result<(), String> {
        let path = match shell.find_in_path(cmd) {
            Some(path) => path,
            None => {
                return Err(format!("{}: command not found", cmd));
            }
        };
        
        let mut command = Command::new(path);
        for arg in args {
            command.arg(arg);
        }
        command.current_dir(&shell.current_dir);
        command.envs(&shell.env_vars);
        
        // Set up stdin (from previous pipe if not first command)
        if command_index > 0 {
            // Take ownership of the reader from the previous pipe pair
            let pipe_pair = self.pipe_pairs.remove(command_index - 1);
            let (reader, writer) = pipe_pair.split();
            
            // Give reader to this command as stdin
            command.stdin(Stdio::from(reader));
            
            // We no longer need the writer (it was used by previous command)
            drop(writer);
            
            // Adjust indices since we removed an element
            // The remaining pipe pairs shift left
        }
        
        // Set up stdout (to next pipe if not last command)
        if command_index < total_commands - 1 {
            // Calculate the new index after potential removal above
            let pipe_index = if command_index > 0 {
                command_index - 1
            } else {
                command_index
            };
            
            if pipe_index < self.pipe_pairs.len() {
                // Take ownership of the writer from the current pipe pair
                let pipe_pair = self.pipe_pairs.remove(pipe_index);
                let (reader, writer) = pipe_pair.split();
                
                // Give writer to this command as stdout
                command.stdout(Stdio::from(writer));
                
                // Store reader for the next command
                // We need to create a new PipePair with just the reader
                // The writer will be dropped when the command finishes
                self.pipe_pairs.insert(pipe_index, PipePair {
                    reader,
                    writer: pipe()?.1, // Dummy writer that will be dropped
                });
            } else {
                return Err("Pipe index out of bounds".to_string());
            }
        }
        // Last command's stdout goes to terminal (default)
        
        match command.spawn() {
            Ok(child) => {
                self.children.push(child);
                Ok(())
            }
            Err(e) => {
                Err(format!("{}: {}", cmd, e))
            }
        }
    }
    
    /// Clean up after spawn failure - kill all children and wait for them
    fn cleanup_on_error(&mut self) {
        for mut child in self.children.drain(..) {
            // Try to kill if still running
            let _ = child.kill();
            // Wait to avoid zombie processes
            let _ = child.wait();
        }
        // Drop all pipe pairs
        self.pipe_pairs.clear();
    }
    
    /// Wait for all children to complete and get exit status of last command
    fn wait_for_completion(&mut self, total_commands: usize) -> i32 {
        // Drop all remaining pipe pairs
        // This closes any pipe ends that are still open in parent
        self.pipe_pairs.clear();
        
        let mut last_status = 0;
        
        for (i, mut child) in self.children.drain(..).enumerate() {
            match child.wait() {
                Ok(status) => {
                    if i == total_commands - 1 {
                        last_status = status.code().unwrap_or(128);
                    }
                }
                Err(e) => {
                    eprintln!("Error waiting for command {}: {}", i + 1, e);
                    if i == total_commands - 1 {
                        last_status = 1;
                    }
                }
            }
        }
        
        last_status
    }
}

/// Platform-specific command adaptation
trait CommandAdapter {
    /// Adapt command and arguments for the platform
    fn adapt_command(&self, cmd: &str, args: &[String]) -> (String, Vec<String>);
}

/// Windows command adapter - SIMPLIFIED
/// We don't automatically wrap commands in cmd.exe
/// Only handle .bat/.cmd files explicitly
#[cfg(windows)]
struct WindowsCommandAdapter;

#[cfg(windows)]
impl CommandAdapter for WindowsCommandAdapter {
    fn adapt_command(&self, cmd: &str, args: &[String]) -> (String, Vec<String>) {
        // On Windows, we handle:
        // 1. .bat and .cmd files - need cmd.exe
        // 2. Everything else - execute directly
        
        // Check if it's a batch file (ends with .bat or .cmd)
        if cmd.ends_with(".bat") || cmd.ends_with(".cmd") {
            // Batch files need cmd.exe /c
            let mut cmd_args = vec!["/c".to_string(), cmd.to_string()];
            cmd_args.extend(args.to_vec());
            ("cmd.exe".to_string(), cmd_args)
        } else {
            // Regular executable - execute directly
            (cmd.to_string(), args.to_vec())
        }
    }
}

/// Unix command adapter
#[cfg(unix)]
struct UnixCommandAdapter;

#[cfg(unix)]
impl CommandAdapter for UnixCommandAdapter {
    fn adapt_command(&self, cmd: &str, args: &[String]) -> (String, Vec<String>) {
        // Unix doesn't need special adaptation
        (cmd.to_string(), args.to_vec())
    }
}

/// Generic pipeline execution with proper handle management
fn execute_pipeline_generic<A: CommandAdapter>(
    shell: &mut Shell,
    line: &str,
    adapter: A,
) -> i32 {
    let commands: Vec<&str> = line.split('|')
        .map(|s| s.trim())
        .collect();
    
    if commands.is_empty() {
        return 0;
    }
    
    // Special case: single command
    if commands.len() == 1 {
        let (cmd, args) = parser::parse_command(commands[0]);
        if cmd.is_empty() {
            return 0;
        }
        
        let args = args.iter()
            .map(|arg| expansion::expand_variables_simple(shell, arg))
            .collect::<Vec<String>>();
        
        if shell.builtin_registry.has_builtin(&cmd) {
            let registry = Rc::clone(&shell.builtin_registry);
            return registry.execute_builtin(shell, &cmd, &args);
        } else {
            return shell.external_command(&cmd, &args, false, None);
        }
    }
    
    // Parse all commands first
    let mut parsed_commands = Vec::new();
    for cmd_str in &commands {
        let (cmd, args) = parser::parse_command(cmd_str);
        let expanded_args = args.iter()
            .map(|arg| expansion::expand_variables_simple(shell, arg))
            .collect::<Vec<String>>();
        parsed_commands.push((cmd, expanded_args));
    }
    
    // Check for builtins - if any command is a builtin, fall back to serial execution
    // because builtins may need shell state and are harder to run in parallel
    let has_builtin = parsed_commands.iter().any(|(cmd, _)| {
        shell.builtin_registry.has_builtin(cmd)
    });
    
    if has_builtin {
        // Fall back to serial execution for builtins
        return execute_pipeline_serial(shell, line);
    }
    
    // Create pipeline context
    let mut context = PipelineContext::new();
    
    // Create pipes
    if let Err(e) = context.create_pipes(parsed_commands.len()) {
        eprintln!("{}", e);
        return 1;
    }
    
    // Spawn all commands
    for (i, (cmd, args)) in parsed_commands.iter().enumerate() {
        // Adapt command for platform
        let (actual_cmd, actual_args) = adapter.adapt_command(cmd, args);
        
        match context.spawn_command(
            shell,
            &actual_cmd,
            &actual_args,
            i,
            parsed_commands.len(),
        ) {
            Ok(()) => continue,
            Err(e) => {
                eprintln!("{}", e);
                context.cleanup_on_error();
                return 1;
            }
        }
    }
    
    // Wait for completion
    context.wait_for_completion(parsed_commands.len())
}

/// Windows-specific pipeline execution
#[cfg(windows)]
fn execute_pipeline_windows(shell: &mut Shell, line: &str) -> i32 {
    execute_pipeline_generic(shell, line, WindowsCommandAdapter)
}

/// Linux/Unix-specific pipeline execution
#[cfg(unix)]
fn execute_pipeline_unix(shell: &mut Shell, line: &str) -> i32 {
    execute_pipeline_generic(shell, line, UnixCommandAdapter)
}

/// Cross-platform pipeline execution
/// Uses platform-specific implementations
pub fn execute_pipeline_cross_platform(shell: &mut Shell, line: &str) -> i32 {
    #[cfg(windows)]
    return execute_pipeline_windows(shell, line);
    
    #[cfg(unix)]
    return execute_pipeline_unix(shell, line);
    
    #[cfg(not(any(windows, unix)))]
    {
        eprintln!("Unsupported platform for parallel pipelines");
        execute_pipeline_serial(shell, line)
    }
}

/// Execute a pipeline (commands connected with |)
/// Main entry point - uses cross-platform parallel execution
pub fn execute_pipeline(shell: &mut Shell, line: &str) -> i32 {
    // Use cross-platform implementation
    execute_pipeline_cross_platform(shell, line)
}