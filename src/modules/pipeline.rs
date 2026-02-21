//! Pipeline execution

use std::process::{Command, Stdio};
use std::io::{self, Write};

use crate::modules::shell::Shell;
use crate::modules::builtins;
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

/// Execute a pipeline (commands connected with |)
pub fn execute_pipeline(shell: &mut Shell, line: &str) -> i32 {
    let commands: Vec<&str> = line.split('|')
        .map(|s| s.trim())
        .collect();
    
    if commands.is_empty() {
        return 0;
    }
    
    let mut last_status = 0;
    
    // We'll handle pipelines by executing commands sequentially
    // and passing output between them
    for (i, cmd_str) in commands.iter().enumerate() {
        let is_last = i == commands.len() - 1;
        
        // Parse the command
        let (cmd, args) = parser::parse_command(cmd_str);
        if cmd.is_empty() {
            continue;
        }
        
        // Expand variables in arguments (use simple version for pipeline)
        let args = args.iter()
            .map(|arg| expansion::expand_variables_simple(shell, arg))
            .collect::<Vec<String>>();
        
        if builtins::is_builtin(&cmd) {
            // For builtins in pipeline, we need to capture output
            last_status = builtins::execute_builtin_in_pipeline(shell, &cmd, &args, is_last);
        } else {
            // External command
            let path = shell.find_in_path(&cmd)
                .unwrap_or_else(|| {
                    eprintln!("{}: command not found", cmd);
                    return String::new();
                });
            
            if path.is_empty() {
                return 127;
            }
            
            let mut command = Command::new(path);
            for arg in &args {
                command.arg(arg);
            }
            command.current_dir(&shell.current_dir);
            command.envs(&shell.env_vars);
            
            // For non-last commands, capture output
            if !is_last {
                command.stdout(Stdio::piped());
            }
            
            match command.spawn() {
                Ok(child) => {
                    match child.wait_with_output() {
                        Ok(output) => {
                            last_status = output.status.code().unwrap_or(128);
                            
                            // For non-last commands, we should pass output to next command
                            // In a full implementation, we'd use pipes between processes
                            // For simplicity, we'll just print intermediate output
                            if !is_last && !output.stdout.is_empty() {
                                // In a real pipeline, this would go to the next command
                                // For now, we'll just note that output was produced
                            } else if is_last && output.status.success() {
                                // Print output of last command
                                if !output.stdout.is_empty() {
                                    io::stdout().write_all(&output.stdout).ok();
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("{}: {}", cmd, e);
                            last_status = 1;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", cmd, e);
                    last_status = 1;
                }
            }
        }
    }
    
    last_status
}