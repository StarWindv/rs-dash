//! Subshell support for rs-dash
//! Supports (command) syntax for executing commands in subshell

use crate::modules::shell::Shell;

/// Check if a command contains subshell syntax
pub fn has_subshell(cmd: &str) -> bool {
    let trimmed = cmd.trim();
    
    // Check for (command) pattern
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        // Make sure it's not just parentheses around a single word
        let inner = &trimmed[1..trimmed.len()-1].trim();
        return !inner.is_empty() && inner.chars().any(|c| c.is_whitespace() || c == '|' || c == ';' || c == '&');
    }
    
    false
}

/// Parse and execute subshell command
pub fn execute_subshell(shell: &mut Shell, cmd: &str) -> i32 {
    // Extract the inner command
    let trimmed = cmd.trim();
    if !trimmed.starts_with('(') || !trimmed.ends_with(')') {
        return 1;
    }
    
    let inner_cmd = &trimmed[1..trimmed.len()-1].trim();
    
    // Save current shell state
    let saved_env_vars = shell.env_vars.clone();
    let saved_current_dir = shell.current_dir.clone();
    let saved_positional_params = shell.positional_params.clone();
    let saved_last_exit_status = shell.last_exit_status;
    
    // Execute the command in subshell
    let exit_status = shell.execute_command_line(inner_cmd);
    
    // Restore shell state (subshell changes don't affect parent)
    shell.env_vars = saved_env_vars;
    shell.current_dir = saved_current_dir;
    shell.positional_params = saved_positional_params;
    shell.last_exit_status = saved_last_exit_status;
    
    exit_status
}

/// Parse command line, handling subshells
pub fn parse_command_with_subshells(shell: &mut Shell, line: &str) -> i32 {
    // Simple implementation: just check for subshell at the start
    if has_subshell(line) {
        return execute_subshell(shell, line);
    }
    
    // Otherwise, execute normally
    shell.execute_command_line(line)
}