//! Pipeline execution

use std::process::{Command, Stdio, Child};
use std::io::{self, Write};
use std::rc::Rc;

// Import os_pipe for creating real OS pipes
use os_pipe::pipe;

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

/// Execute a single pipeline command
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

/// Execute a pipeline (commands connected with |) - SERIAL implementation
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

/// Create and manage a true parallel pipeline using OS pipes
pub fn execute_pipeline_parallel(shell: &mut Shell, line: &str) -> i32 {
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
    let parsed_commands: Vec<(String, Vec<String>)> = commands.iter()
        .map(|cmd_str| {
            let (cmd, args) = parser::parse_command(cmd_str);
            let expanded_args = args.iter()
                .map(|arg| expansion::expand_variables_simple(shell, arg))
                .collect::<Vec<String>>();
            (cmd, expanded_args)
        })
        .collect();
    
    // Check for builtins - if any command is a builtin, fall back to serial execution
    // because builtins may need shell state and are harder to run in parallel
    let has_builtin = parsed_commands.iter().any(|(cmd, _)| {
        shell.builtin_registry.has_builtin(cmd)
    });
    
    if has_builtin {
        // Fall back to serial execution for builtins
        return execute_pipeline_serial(shell, line);
    }
    
    // For external commands, we can create a true parallel pipeline
    // We'll create all processes and connect their pipes
    
    let mut children: Vec<Child> = Vec::new();
    let mut pipes: Vec<(os_pipe::PipeReader, os_pipe::PipeWriter)> = Vec::new();
    
    // Create pipes between commands
    for _ in 0..(parsed_commands.len() - 1) {
        match pipe() {
            Ok((reader, writer)) => {
                pipes.push((reader, writer));
            }
            Err(e) => {
                eprintln!("Failed to create pipe: {}", e);
                return 1;
            }
        }
    }
    
    // Spawn all commands in parallel
    for i in 0..parsed_commands.len() {
        let (cmd, args) = &parsed_commands[i];
        let path = match shell.find_in_path(cmd) {
            Some(path) => path,
            None => {
                eprintln!("{}: command not found", cmd);
                // Kill already spawned children
                for mut child in children {
                    let _ = child.kill();
                }
                return 127;
            }
        };
        
        let mut command = Command::new(path);
        for arg in args {
            command.arg(arg);
        }
        command.current_dir(&shell.current_dir);
        command.envs(&shell.env_vars);
        
        // Set up stdin
        if i > 0 {
            // Not the first command - connect to previous pipe's reader
            let (reader, _) = &pipes[i - 1];
            // Convert os_pipe::PipeReader to std::process::Stdio
            command.stdin(Stdio::from(reader.try_clone().expect("Failed to clone pipe reader")));
        }
        
        // Set up stdout
        if i < parsed_commands.len() - 1 {
            // Not the last command - connect to current pipe's writer
            let (_, writer) = &pipes[i];
            // Convert os_pipe::PipeWriter to std::process::Stdio
            command.stdout(Stdio::from(writer.try_clone().expect("Failed to clone pipe writer")));
        }
        // Last command's stdout goes to terminal (default)
        
        match command.spawn() {
            Ok(child) => {
                children.push(child);
            }
            Err(e) => {
                eprintln!("{}: {}", cmd, e);
                // Kill already spawned children
                for child in &mut children {
                    let _ = child.kill();
                }
                return 1;
            }
        }
    }
    
    // Close pipe writer ends in the parent process
    // This is important: when all writers are closed, readers get EOF
    drop(pipes);
    
    // Wait for all children to complete
    let mut last_status = 0;
    
    for (i, child) in children.into_iter().enumerate() {
        match child.wait_with_output() {
            Ok(output) => {
                if i == parsed_commands.len() - 1 {
                    // Last command's exit status is the pipeline's exit status
                    last_status = output.status.code().unwrap_or(128);
                    if !output.stdout.is_empty() {
                        // Print the output
                        io::stdout().write_all(&output.stdout).ok();
                    }
                }
            }
            Err(e) => {
                eprintln!("Error waiting for command {}: {}", i + 1, e);
                if i == parsed_commands.len() - 1 {
                    last_status = 1;
                }
            }
        }
    }
    
    last_status
}

/// Windows-specific pipeline execution
/// Windows has different pipe semantics and command execution
#[cfg(windows)]
fn execute_pipeline_windows(shell: &mut Shell, line: &str) -> i32 {
    let commands: Vec<&str> = line.split('|')
        .map(|s| s.trim())
        .collect();
    
    if commands.is_empty() {
        return 0;
    }
    
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
    
    // Parse commands
    let parsed_commands: Vec<(String, Vec<String>)> = commands.iter()
        .map(|cmd_str| {
            let (cmd, args) = parser::parse_command(cmd_str);
            let expanded_args = args.iter()
                .map(|arg| expansion::expand_variables_simple(shell, arg))
                .collect::<Vec<String>>();
            (cmd, expanded_args)
        })
        .collect();
    
    // Check for builtins
    let has_builtin = parsed_commands.iter().any(|(cmd, _)| {
        shell.builtin_registry.has_builtin(cmd)
    });
    
    if has_builtin {
        return execute_pipeline_serial(shell, line);
    }
    
    // On Windows, we need to handle cmd.exe specially
    // Many commands are actually cmd.exe builtins or batch files
    
    let mut children: Vec<Child> = Vec::new();
    let mut pipes = Vec::new();
    
    // Create pipes
    for _ in 0..(parsed_commands.len() - 1) {
        match pipe() {
            Ok(pipe_pair) => pipes.push(pipe_pair),
            Err(e) => {
                eprintln!("Failed to create pipe: {}", e);
                return 1;
            }
        }
    }
    
    // Spawn commands
    for i in 0..parsed_commands.len() {
        let (cmd, args) = &parsed_commands[i];
        
        // On Windows, check if we need to use cmd.exe
        let (actual_cmd, actual_args) = if cmd.ends_with(".bat") || cmd.ends_with(".cmd") || 
                                          cmd == "dir" || cmd == "copy" || cmd == "del" || 
                                          cmd == "echo" || cmd == "type" || cmd == "findstr" {
            // These are cmd.exe builtins or batch files
            let mut cmd_args = vec!["/c".to_string(), cmd.clone()];
            cmd_args.extend(args.clone());
            ("cmd.exe".to_string(), cmd_args)
        } else {
            (cmd.clone(), args.clone())
        };
        
        let path = match shell.find_in_path(&actual_cmd) {
            Some(path) => path,
            None => {
                eprintln!("{}: command not found", cmd);
                for mut child in children {
                    let _ = child.kill();
                }
                return 127;
            }
        };
        
        let mut command = Command::new(path);
        for arg in &actual_args {
            command.arg(arg);
        }
        command.current_dir(&shell.current_dir);
        command.envs(&shell.env_vars);
        
        // Set up stdin
        if i > 0 {
            let (reader, _) = &pipes[i - 1];
            command.stdin(Stdio::from(reader.try_clone().expect("Failed to clone pipe reader")));
        }
        
        // Set up stdout
        if i < parsed_commands.len() - 1 {
            let (_, writer) = &pipes[i];
            command.stdout(Stdio::from(writer.try_clone().expect("Failed to clone pipe writer")));
        }
        
        match command.spawn() {
            Ok(child) => children.push(child),
            Err(e) => {
                eprintln!("{}: {}", cmd, e);
                for mut child in children {
                    let _ = child.kill();
                }
                return 1;
            }
        }
    }
    
    // Close pipes in parent
    drop(pipes);
    
    // Wait for completion
    let mut last_status = 0;
    for (i, mut child) in children.into_iter().enumerate() {
        match child.wait() {
            Ok(status) => {
                if i == parsed_commands.len() - 1 {
                    last_status = status.code().unwrap_or(128);
                }
            }
            Err(e) => {
                eprintln!("Error waiting for command {}: {}", i + 1, e);
                if i == parsed_commands.len() - 1 {
                    last_status = 1;
                }
            }
        }
    }
    
    last_status
}

/// Linux/Unix-specific pipeline execution
#[cfg(unix)]
fn execute_pipeline_unix(shell: &mut Shell, line: &str) -> i32 {
    let commands: Vec<&str> = line.split('|')
        .map(|s| s.trim())
        .collect();
    
    if commands.is_empty() {
        return 0;
    }
    
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
    
    // Parse commands
    let parsed_commands: Vec<(String, Vec<String>)> = commands.iter()
        .map(|cmd_str| {
            let (cmd, args) = parser::parse_command(cmd_str);
            let expanded_args = args.iter()
                .map(|arg| expansion::expand_variables_simple(shell, arg))
                .collect::<Vec<String>>();
            (cmd, expanded_args)
        })
        .collect();
    
    // Check for builtins
    let has_builtin = parsed_commands.iter().any(|(cmd, _)| {
        shell.builtin_registry.has_builtin(cmd)
    });
    
    if has_builtin {
        return execute_pipeline_serial(shell, line);
    }
    
    // Create pipes
    let mut pipes = Vec::new();
    for _ in 0..(parsed_commands.len() - 1) {
        match pipe() {
            Ok(pipe_pair) => pipes.push(pipe_pair),
            Err(e) => {
                eprintln!("Failed to create pipe: {}", e);
                return 1;
            }
        }
    }
    
    // Spawn commands
    let mut children: Vec<Child> = Vec::new();
    
    for i in 0..parsed_commands.len() {
        let (cmd, args) = &parsed_commands[i];
        let path = match shell.find_in_path(cmd) {
            Some(path) => path,
            None => {
                eprintln!("{}: command not found", cmd);
                for mut child in children {
                    let _ = child.kill();
                }
                return 127;
            }
        };
        
        let mut command = Command::new(path);
        for arg in args {
            command.arg(arg);
        }
        command.current_dir(&shell.current_dir);
        command.envs(&shell.env_vars);
        
        // Configure stdin
        if i > 0 {
            let (reader, _) = &pipes[i - 1];
            command.stdin(Stdio::from(reader.try_clone().expect("Failed to clone pipe reader")));
        }
        
        // Configure stdout
        if i < parsed_commands.len() - 1 {
            let (_, writer) = &pipes[i];
            command.stdout(Stdio::from(writer.try_clone().expect("Failed to clone pipe writer")));
        }
        
        match command.spawn() {
            Ok(child) => children.push(child),
            Err(e) => {
                eprintln!("{}: {}", cmd, e);
                for mut child in children {
                    let _ = child.kill();
                }
                return 1;
            }
        }
    }
    
    // Close pipes in parent
    drop(pipes);
    
    // Wait for completion
    let mut last_status = 0;
    for (i, mut child) in children.into_iter().enumerate() {
        match child.wait() {
            Ok(status) => {
                if i == parsed_commands.len() - 1 {
                    last_status = status.code().unwrap_or(128);
                }
            }
            Err(e) => {
                eprintln!("Error waiting for command {}: {}", i + 1, e);
                if i == parsed_commands.len() - 1 {
                    last_status = 1;
                }
            }
        }
    }
    
    last_status
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