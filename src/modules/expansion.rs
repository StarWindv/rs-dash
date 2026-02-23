//! Variable expansion and command substitution

use crate::modules::shell::Shell;
use crate::modules::param_expand;
use crate::modules::arithmetic;

/// Expand variables and command substitutions in a string
pub fn expand_variables(shell: &mut Shell, input: &str) -> String {
    
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '$' {
            // Check what comes after $
            if let Some(&next_c) = chars.peek() {
                if next_c == '(' {
                    chars.next(); // Skip '('
                    // Check if it's arithmetic expansion: $((...))
                    if let Some(&next_next) = chars.peek() {
                        if next_next == '(' {
                            // Arithmetic expansion: $((expression))
                            chars.next(); // Skip second '('
                            let mut expr = String::new();
                            let mut paren_depth = 2;
                            
                            while let Some(c) = chars.next() {
                                if c == '(' {
                                    paren_depth += 1;
                                    expr.push(c);
                                } else if c == ')' {
                                    paren_depth -= 1;
                                    if paren_depth == 0 {
                                        break;
                                    } else {
                                        expr.push(c);
                                    }
                                } else {
                                    expr.push(c);
                                }
                            }
                            
                            // Evaluate arithmetic expression
                            match arithmetic::expand_arithmetic(shell, &expr) {
                                Ok(value) => result.push_str(&value),
                                Err(e) => {
                                    eprintln!("Arithmetic expansion error: {}", e);
                                    // On error, leave empty like dash
                                }
                            }
                        } else {
                            // Command substitution: $(command)
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
                        }
                    } else {
                        // Just $( at end of string, treat as literal
                        result.push('$');
                        result.push('(');
                    }
                } else if next_c == '{' {
                    // Parameter expansion: ${...}
                    chars.next(); // Skip '{'
                    let mut content = String::new();
                    let mut brace_depth = 1;
                    
                    while let Some(c) = chars.next() {
                        if c == '{' {
                            brace_depth += 1;
                            content.push(c);
                        } else if c == '}' {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                break;
                            } else {
                                content.push(c);
                            }
                        } else {
                            content.push(c);
                        }
                    }
                    
                    // Parse and expand parameter
                    match param_expand::parse_param_expansion(&content) {
                        Ok(expansion) => {
                            match param_expand::expand_param(shell, &expansion) {
                                Ok(value) => result.push_str(&value),
                                Err(e) => {
                                    eprintln!("{}", e);
                                    // On error, leave empty like dash
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Parameter expansion error: {}", e);
                            // On error, leave as is
                            result.push_str(&format!("${{{}}}", content));
                        }
                    }
                } else if next_c == '#' {
                    // Special variable: $# (number of positional parameters)
                    chars.next(); // Skip '#'
                    result.push_str(&shell.positional_param_count().to_string());
                } else if next_c == '@' || next_c == '*' {
                    // Special variables: $@ or $* (all positional parameters)
                    chars.next(); // Skip '@' or '*'
                    result.push_str(&shell.positional_params.join(" "));
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
                    
                    // Try to parse as positional parameter
                    match num_str.parse::<usize>() {
                        Ok(n) => {
                            if let Some(value) = shell.get_positional_param(n) {
                                result.push_str(value);
                            }
                            // If positional parameter not set, leave empty (like dash)
                        }
                        Err(_) => {
                            // Not a valid number, treat as regular variable
                            if let Some(value) = shell.env_vars.get(&num_str) {
                                result.push_str(value);
                            }
                        }
                    }
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
                    // Positional parameters not yet implemented
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