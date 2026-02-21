//! Variable expansion and command substitution

use crate::modules::shell::Shell;

/// Expand variables and command substitutions in a string
pub fn expand_variables(shell: &mut Shell, input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '$' {
            // Check what comes after $
            if let Some(&next_c) = chars.peek() {
                if next_c == '(' {
                    // Command substitution: $(command)
                    chars.next(); // Skip '('
                    let mut cmd_str = String::new();
                    let mut paren_depth = 1;
                    
                    while let Some(c) = chars.next() {
                        if c == '(' {
                            paren_depth += 1;
                            cmd_str.push(c);
                        } else if c == ')' {
                            paren_depth -= 1;
                            if paren_depth == 0 {
                                break;
                            } else {
                                cmd_str.push(c);
                            }
                        } else {
                            cmd_str.push(c);
                        }
                    }
                    
                    // Execute the command and capture output
                    let output = shell.execute_command_and_capture(&cmd_str);
                    result.push_str(&output);
                } else if next_c == '?' {
                    // Special variable: $? (exit status)
                    chars.next(); // Skip '?'
                    result.push_str(&shell.last_exit_status.to_string());
                } else if next_c == '$' {
                    // Special variable: $$ (PID)
                    chars.next(); // Skip second '$'
                    result.push_str(&std::process::id().to_string());
                } else if next_c == '0' {
                    // Special variable: $0 (shell name)
                    chars.next(); // Skip '0'
                    result.push_str("rs-dash");
                } else if next_c.is_digit(10) {
                    // Positional parameter: $1, $2, etc.
                    let mut num_str = String::new();
                    while let Some(&next_c) = chars.peek() {
                        if next_c.is_digit(10) {
                            num_str.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    // For now, positional parameters not implemented
                    // Just leave empty like dash does for undefined parameters
                } else {
                    // Regular variable expansion
                    let mut var_name = String::new();
                    while let Some(&next_c) = chars.peek() {
                        if next_c.is_alphanumeric() || next_c == '_' {
                            var_name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    if var_name.is_empty() {
                        // Just a $, keep it
                        result.push('$');
                    } else {
                        // Look up variable
                        if let Some(value) = shell.env_vars.get(&var_name) {
                            result.push_str(value);
                        }
                        // If variable not found, leave it empty (like dash)
                    }
                }
            } else {
                // Just a $ at end of string
                result.push('$');
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Expand variables in a string (simple version without command substitution)
pub fn expand_variables_simple(shell: &Shell, input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '$' {
            // Check what comes after $
            if let Some(&next_c) = chars.peek() {
                if next_c == '?' {
                    // Special variable: $? (exit status)
                    chars.next(); // Skip '?'
                    result.push_str(&shell.last_exit_status.to_string());
                } else if next_c == '$' {
                    // Special variable: $$ (PID)
                    chars.next(); // Skip second '$'
                    result.push_str(&std::process::id().to_string());
                } else if next_c == '0' {
                    // Special variable: $0 (shell name)
                    chars.next(); // Skip '0'
                    result.push_str("rs-dash");
                } else if next_c.is_digit(10) {
                    // Positional parameter: $1, $2, etc.
                    let mut num_str = String::new();
                    while let Some(&next_c) = chars.peek() {
                        if next_c.is_digit(10) {
                            num_str.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    // For now, positional parameters not implemented
                } else {
                    // Regular variable expansion
                    let mut var_name = String::new();
                    while let Some(&next_c) = chars.peek() {
                        if next_c.is_alphanumeric() || next_c == '_' {
                            var_name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    if var_name.is_empty() {
                        // Just a $, keep it
                        result.push('$');
                    } else {
                        // Look up variable
                        if let Some(value) = shell.env_vars.get(&var_name) {
                            result.push_str(value);
                        }
                    }
                }
            } else {
                // Just a $ at end of string
                result.push('$');
            }
        } else {
            result.push(c);
        }
    }
    
    result
}