//! Redirection handling

use std::fs::{File, OpenOptions};
use std::process::{Command, Stdio};
use std::collections::HashMap;

/// Redirection types
pub enum Redirection {
    Output(String),   // > filename
    Append(String),   // >> filename
    Input(String),    // < filename
}

/// Parse redirections from command arguments
pub fn parse_redirections(cmd: &str, args: &[String]) -> (String, Vec<String>, Vec<Redirection>) {
    let mut new_args = Vec::new();
    let mut redirects = Vec::new();
    
    // The command name should stay as is
    let current_cmd = cmd.to_string();
    
    for arg in args {
        if arg.starts_with(">>") {
            // Append redirection
            let filename = if arg.len() > 2 { &arg[2..] } else { "" };
            if !filename.is_empty() {
                redirects.push(Redirection::Append(filename.to_string()));
            }
        } else if arg.starts_with('>') {
            // Output redirection
            let filename = if arg.len() > 1 { &arg[1..] } else { "" };
            if !filename.is_empty() {
                redirects.push(Redirection::Output(filename.to_string()));
            }
        } else if arg.starts_with('<') {
            // Input redirection
            let filename = if arg.len() > 1 { &arg[1..] } else { "" };
            if !filename.is_empty() {
                redirects.push(Redirection::Input(filename.to_string()));
            }
        } else {
            new_args.push(arg.clone());
        }
    }
    
    (current_cmd, new_args, redirects)
}

/// Execute command with redirections
pub fn execute_with_redirections(
    path: &str,
    args: &[String],
    current_dir: &str,
    env_vars: &HashMap<String, String>,
    redirects: Vec<Redirection>,
    stdin_data: Option<&[u8]>
) -> i32 {
    // Prepare command
    let mut command = Command::new(path);
    
    // Add arguments
    for arg in args {
        command.arg(arg);
    }
    
    // Set current directory
    command.current_dir(current_dir);
    
    // Apply redirections
    for redirect in redirects {
        match redirect {
            Redirection::Output(filename) => {
                match File::create(&filename) {
                    Ok(file) => {
                        command.stdout(file);
                    }
                    Err(e) => {
                        eprintln!("{}: {}", filename, e);
                        return 1;
                    }
                }
            }
            Redirection::Append(filename) => {
                match OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&filename) {
                    Ok(file) => {
                        command.stdout(file);
                    }
                    Err(e) => {
                        eprintln!("{}: {}", filename, e);
                        return 1;
                    }
                }
            }
            Redirection::Input(filename) => {
                match File::open(&filename) {
                    Ok(file) => {
                        command.stdin(file);
                    }
                    Err(e) => {
                        eprintln!("{}: {}", filename, e);
                        return 1;
                    }
                }
            }
        }
    }
    
    // Set environment variables
    command.envs(env_vars);
    
    // Handle pipeline stdin
    if let Some(data) = stdin_data {
        if !data.is_empty() {
            command.stdin(Stdio::piped());
        }
    }
    
    // Execute command
    match command.spawn() {
        Ok(mut child) => {
            // Write stdin data if provided
            if let Some(_data) = stdin_data {
                if let Some(_stdin) = child.stdin.take() {
                    // In a real implementation, we'd write the data
                    // For now, this is simplified
                }
            }
            
            match child.wait() {
                Ok(status) => {
                    if let Some(code) = status.code() {
                        code
                    } else {
                        // Process terminated by signal
                        128
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", path, e);
                    1
                }
            }
        }
        Err(e) => {
            eprintln!("{}: {}", path, e);
            1
        }
    }
}