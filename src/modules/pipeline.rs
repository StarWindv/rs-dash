//! Pipeline execution

use std::process::{Command, Stdio, Child};
use std::io::{self, Write, Read};
use std::rc::Rc;

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

/// Create and manage a true parallel pipeline
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
    
    // Create the first command
    let (first_cmd, first_args) = &parsed_commands[0];
    let first_path = match shell.find_in_path(first_cmd) {
        Some(path) => path,
        None => {
            eprintln!("{}: command not found", first_cmd);
            return 127;
        }
    };
    
    let mut first_command = Command::new(first_path);
    for arg in first_args {
        first_command.arg(arg);
    }
    first_command.current_dir(&shell.current_dir);
    first_command.envs(&shell.env_vars);
    first_command.stdout(Stdio::piped()); // First command's output goes to pipe
    
    let first_child = match first_command.spawn() {
        Ok(mut child) => child,
        Err(e) => {
            eprintln!("{}: {}", first_cmd, e);
            return 1;
        }
    };
    
    children.push(first_child);
    let mut prev_stdout = children[0].stdout.take();
    
    // Create intermediate commands
    for i in 1..parsed_commands.len() - 1 {
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
        
        // Connect stdin from previous command
        if let Some(stdout) = prev_stdout.take() {
            command.stdin(Stdio::from(stdout));
        }
        
        // Set up stdout for next command
        command.stdout(Stdio::piped());
        
        match command.spawn() {
            Ok(mut child) => {
                prev_stdout = child.stdout.take();
                children.push(child);
            }
            Err(e) => {
                eprintln!("{}: {}", cmd, e);
                // Kill already spawned children
                for mut child in children {
                    let _ = child.kill();
                }
                return 1;
            }
        }
    }
    
    // Create the last command
    let last_idx = parsed_commands.len() - 1;
    let (last_cmd, last_args) = &parsed_commands[last_idx];
    let last_path = match shell.find_in_path(last_cmd) {
        Some(path) => path,
        None => {
            eprintln!("{}: command not found", last_cmd);
            // Kill already spawned children
            for mut child in children {
                let _ = child.kill();
            }
            return 127;
        }
    };
    
    let mut last_command = Command::new(last_path);
    for arg in last_args {
        last_command.arg(arg);
    }
    last_command.current_dir(&shell.current_dir);
    last_command.envs(&shell.env_vars);
    
    // Connect stdin from previous command
    if let Some(stdout) = prev_stdout.take() {
        last_command.stdin(Stdio::from(stdout));
    }
    
    // Last command's output goes to terminal
    // We'll capture it and print it
    last_command.stdout(Stdio::piped());
    
    let last_child = match last_command.spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("{}: {}", last_cmd, e);
            // Kill already spawned children
            for mut child in children {
                let _ = child.kill();
            }
            return 1;
        }
    };
    
    children.push(last_child);
    
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

/// Execute a pipeline (commands connected with |)
/// Main entry point - uses parallel execution when possible
pub fn execute_pipeline(shell: &mut Shell, line: &str) -> i32 {
    // For now, use parallel implementation for external commands
    // Serial implementation for builtins
    execute_pipeline_parallel(shell, line)
}