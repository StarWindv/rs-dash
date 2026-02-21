//! Process substitution support for rs-dash
//! Supports <(command) and >(command) syntax

use crate::modules::shell::Shell;
use std::fs;
use std::io::Write;
use std::process::Command;

/// Check if a command contains process substitution
pub fn has_process_substitution(cmd: &str) -> bool {
    cmd.contains("<(") || cmd.contains(">(")
}

/// Execute process substitution
pub fn execute_process_substitution(shell: &mut Shell, cmd: &str) -> Result<String, String> {
    // This is a simplified implementation
    // In a real shell, <(command) creates a named pipe or /dev/fd file descriptor
    // For now, we'll execute the command and capture output to a temp file
    
    let mut result = cmd.to_string();
    
    // Find and replace <(command) patterns
    while let Some(start) = result.find("<(") {
        if let Some(end) = find_matching_paren(&result[start..]) {
            let full_match = &result[start..start+end];
            let inner_cmd = &full_match[2..full_match.len()-1]; // Remove <( and )
            
            // Execute the command and capture output
            let output = shell.execute_command_and_capture(inner_cmd);
            
            // On Unix, we could use /dev/fd or named pipes
            // On Windows, we need a different approach
            // For now, we'll just use the output directly for simple cases
            // This is a simplification and won't work for all cases
            
            // Replace <(command) with a placeholder
            // In a real implementation, this would be a file descriptor
            result = result.replacen(full_match, &format!("'{}'", output), 1);
        } else {
            break;
        }
    }
    
    // Find and replace >(command) patterns
    while let Some(start) = result.find(">(") {
        if let Some(end) = find_matching_paren(&result[start..]) {
            let full_match = &result[start..start+end];
            let inner_cmd = &full_match[2..full_match.len()-1]; // Remove >( and )
            
            // For >(command), we need to create a pipe for writing
            // This is more complex and requires async handling
            // For now, we'll just return an error
            return Err(">(command) process substitution not yet implemented".to_string());
        } else {
            break;
        }
    }
    
    Ok(result)
}

/// Find matching parenthesis
fn find_matching_paren(s: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i + 1); // +1 to include the closing paren
                }
            }
            _ => {}
        }
    }
    None
}

/// Handle command line with process substitution
pub fn handle_process_substitution(shell: &mut Shell, line: &str) -> Result<String, i32> {
    if has_process_substitution(line) {
        match execute_process_substitution(shell, line) {
            Ok(processed_line) => Ok(processed_line),
            Err(e) => {
                eprintln!("{}", e);
                Err(1)
            }
        }
    } else {
        Ok(line.to_string())
    }
}