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

/// Pipeline execution context with proper handle management
/// FIXED VERSION: Simplified approach with separate pipe arrays
struct PipelineContext {
    /// Child processes
    children: Vec<Child>,
    /// Pipe readers for each command (except first)
    /// command i reads from pipe_reader[i-1]
    pipe_readers: Vec<Option<PipeReader>>,
    /// Pipe writers for each command (except last)
    /// command i writes to pipe_writer[i]
    pipe_writers: Vec<Option<PipeWriter>>,
}

impl PipelineContext {
    fn new(num_commands: usize) -> Self {
        let num_pipes = if num_commands > 1 { num_commands - 1 } else { 0 };
        Self {
            children: Vec::new(),
            pipe_readers: Vec::with_capacity(num_pipes),
            pipe_writers: Vec::with_capacity(num_pipes),
        }
    }
    
    /// Create pipes for N commands (N-1 pipes needed)
    fn create_pipes(&mut self) -> Result<(), String> {
        let num_pipes = self.pipe_readers.capacity();
        for i in 0..num_pipes {
            match pipe() {
                Ok((reader, writer)) => {
                    self.pipe_readers.push(Some(reader));
                    self.pipe_writers.push(Some(writer));
                }
                Err(e) => {
                    return Err(format!("Failed to create pipe {}: {}", i, e));
                }
            }
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
        // Note: cmd here is already the adapted command (e.g., "cmd.exe" for echo)
        // We need to check if it exists
        let path = if cmd.contains('/') || cmd.contains('\\') || cmd.contains('.') {
            // Looks like a path, use as is
            cmd.to_string()
        } else {
            // Try to find in PATH
            match shell.find_in_path(cmd) {
                Some(path) => path,
                None => {
                    // For cmd.exe builtins, we don't need to find them in PATH
                    if cmd == "cmd.exe" {
                        "cmd.exe".to_string()
                    } else {
                        return Err(format!("{}: command not found", cmd));
                    }
                }
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
            // Take the reader from the previous pipe
            if let Some(reader) = self.pipe_readers[command_index - 1].take() {
                command.stdin(Stdio::from(reader));
            } else {
                return Err(format!("No pipe reader available for command {}", command_index));
            }
        }
        
        // Set up stdout (to next pipe if not last command)
        if command_index < total_commands - 1 {
            // Take the writer for this command
            if let Some(writer) = self.pipe_writers[command_index].take() {
                command.stdout(Stdio::from(writer));
            } else {
                return Err(format!("No pipe writer available for command {}", command_index));
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
        // Drop all pipe ends
        self.pipe_readers.clear();
        self.pipe_writers.clear();
    }
    
    /// Wait for all children to complete and get exit status of last command
    fn wait_for_completion(&mut self) -> i32 {
        // Drop all remaining pipe ends
        // This closes any pipe ends that are still open in parent
        self.pipe_readers.clear();
        self.pipe_writers.clear();
        
        let mut last_status = 0;
        let total_children = self.children.len();
        
        for (i, mut child) in self.children.drain(..).enumerate() {
            match child.wait() {
                Ok(status) => {
                    let exit_code = status.code().unwrap_or(128);
                    if i == total_children - 1 {
                        last_status = exit_code;
                    }
                }
                Err(e) => {
                    eprintln!("Error waiting for command {}: {}", i + 1, e);
                    if i == total_children - 1 {
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
    
    /// Check if command exists on this platform
    fn command_exists(&self, shell: &Shell, cmd: &str) -> bool;
}

struct UnixCommandAdapter;

impl CommandAdapter for UnixCommandAdapter {
    fn adapt_command(&self, cmd: &str, args: &[String]) -> (String, Vec<String>) {
        (cmd.to_string(), args.to_vec())
    }
    
    fn command_exists(&self, shell: &Shell, cmd: &str) -> bool {
        shell.find_in_path(cmd).is_some()
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
    // But for simple builtins like echo, we can still use parallel execution
    let has_complex_builtin = parsed_commands.iter().any(|(cmd, _)| {
        if shell.builtin_registry.has_builtin(cmd) {
            // Check if it's a simple builtin that can work in pipeline
            // Simple builtins: echo, true, false, pwd
            // Complex builtins: cd, exit, help, etc.
            match cmd.as_str() {
                "echo" | "true" | "false" | "pwd" => false, // These can work in pipeline
                _ => true, // Others need serial execution
            }
        } else {
            false
        }
    });
    
    if has_complex_builtin {
        return execute_pipeline_serial(shell, line);
    }
    
    // Check if all commands exist on this platform
    let all_commands_exist = parsed_commands.iter().all(|(cmd, _)| {
        let exists = adapter.command_exists(shell, cmd);
        exists
    });
    
    if !all_commands_exist {

        return execute_pipeline_serial(shell, line);
    }
    
    // Create pipeline context
    let mut context = PipelineContext::new(parsed_commands.len());
    
    // Create pipes
    if let Err(e) = context.create_pipes() {
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
    context.wait_for_completion()
}


fn execute_pipeline_unix(shell: &mut Shell, line: &str) -> i32 {
    execute_pipeline_generic(shell, line, UnixCommandAdapter)
}

/// Cross-platform pipeline execution
/// Uses platform-specific implementations
pub fn execute_pipeline_cross_platform(shell: &mut Shell, line: &str) -> i32 {

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

