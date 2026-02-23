//! test builtin command for rs-dash

use crate::modules::shell::Shell;
use super::Builtin;
use std::fs;
use std::path::Path;

/// test builtin command
pub struct TestBuiltin;

impl Builtin for TestBuiltin {
    fn name(&self) -> &'static str {
        "test"
    }
    
    fn execute(&self, _shell: &mut Shell, args: &[String]) -> i32 {
        execute_test(args)
    }
}

/// Execute the test command (also used by [ builtin)
pub fn execute_test(args: &[String]) -> i32 {
    // Convert args to Vec<&str> for easier processing
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    
    // Handle the [ command special case
    let is_bracket = !args.is_empty() && args[0] == "[";
    let args = if is_bracket {
        // For [ command, we need to check that the last argument is "]"
        if args.len() < 2 || args[args.len() - 1] != "]" {
            eprintln!("test: missing ]");
            return 2; // Exit code 2 for syntax error in test
        }
        &args[1..args.len() - 1] // Remove "[" and "]"
    } else {
        &args[..]
    };
    
    if args.is_empty() {
        // test with no arguments returns false (exit code 1)
        return 1;
    }
    
    // Simple implementation for now - just handle basic cases
    match evaluate_expression(args) {
        Ok(result) => if result { 0 } else { 1 },
        Err(e) => {
            eprintln!("test: {}", e);
            2 // Exit code 2 for syntax error
        }
    }
}

/// Evaluate a test expression
fn evaluate_expression(args: &[&str]) -> Result<bool, String> {
    if args.is_empty() {
        return Ok(false);
    }
    
    // Handle negation
    if args[0] == "!" {
        if args.len() == 1 {
            return Err("!: argument expected".to_string());
        }
        let result = evaluate_expression(&args[1..])?;
        return Ok(!result);
    }
    
    // Handle parentheses
    if args[0] == "(" && args.len() > 1 && args[args.len() - 1] == ")" {
        return evaluate_expression(&args[1..args.len() - 1]);
    }
    
    // Handle unary operators
    if args.len() == 2 {
        let operator = args[0];
        let operand = args[1];
        
        match operator {
            "-n" => return Ok(!operand.is_empty()),
            "-z" => return Ok(operand.is_empty()),
            _ => {
                // Check if it's a file test operator
                if let Some(result) = evaluate_file_test(operator, operand) {
                    return Ok(result);
                }
            }
        }
    }
    
    // Handle binary operators
    if args.len() == 3 {
        let left = args[0];
        let operator = args[1];
        let right = args[2];
        
        match operator {
            "=" => return Ok(left == right),
            "!=" => return Ok(left != right),
            "-eq" => {
                let left_num = parse_number(left)?;
                let right_num = parse_number(right)?;
                return Ok(left_num == right_num);
            }
            "-ne" => {
                let left_num = parse_number(left)?;
                let right_num = parse_number(right)?;
                return Ok(left_num != right_num);
            }
            "-gt" => {
                let left_num = parse_number(left)?;
                let right_num = parse_number(right)?;
                return Ok(left_num > right_num);
            }
            "-lt" => {
                let left_num = parse_number(left)?;
                let right_num = parse_number(right)?;
                return Ok(left_num < right_num);
            }
            "-ge" => {
                let left_num = parse_number(left)?;
                let right_num = parse_number(right)?;
                return Ok(left_num >= right_num);
            }
            "-le" => {
                let left_num = parse_number(left)?;
                let right_num = parse_number(right)?;
                return Ok(left_num <= right_num);
            }
            _ => {}
        }
    }
    
    // If we get here and have a single argument, test if it's non-empty
    if args.len() == 1 {
        return Ok(!args[0].is_empty());
    }
    
    // Handle -a (AND) and -o (OR) operators
    // This is a simplified implementation
    for (i, &arg) in args.iter().enumerate() {
        if arg == "-a" && i > 0 && i < args.len() - 1 {
            let left = evaluate_expression(&args[..i])?;
            let right = evaluate_expression(&args[i+1..])?;
            return Ok(left && right);
        }
        if arg == "-o" && i > 0 && i < args.len() - 1 {
            let left = evaluate_expression(&args[..i])?;
            let right = evaluate_expression(&args[i+1..])?;
            return Ok(left || right);
        }
    }
    
    Err("expression syntax error".to_string())
}

/// Evaluate a file test operator
fn evaluate_file_test(operator: &str, operand: &str) -> Option<bool> {
    let path = Path::new(operand);
    
    match operator {
        "-e" => Some(path.exists()),
        "-f" => Some(path.is_file()),
        "-d" => Some(path.is_dir()),
        "-r" => {
            // Check if file is readable
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = fs::metadata(path) {
                    let permissions = metadata.permissions();
                    // Check if readable by user, group, or others
                    Some(permissions.mode() & 0o444 != 0)
                } else {
                    Some(false)
                }
            }
            #[cfg(windows)]
            {
                // On Windows, we'll just check if the file exists and can be opened for reading
                match fs::File::open(path) {
                    Ok(_) => Some(true),
                    Err(_) => Some(false),
                }
            }
        }
        "-w" => {
            // Check if file is writable
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = fs::metadata(path) {
                    let permissions = metadata.permissions();
                    // Check if writable by user, group, or others
                    Some(permissions.mode() & 0o222 != 0)
                } else {
                    Some(false)
                }
            }
            #[cfg(windows)]
            {
                // On Windows, check if we can open the file for writing
                match fs::OpenOptions::new().write(true).open(path) {
                    Ok(_) => Some(true),
                    Err(_) => Some(false),
                }
            }
        }
        "-x" => {
            // Check if file is executable
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = fs::metadata(path) {
                    let permissions = metadata.permissions();
                    // Check if executable by user, group, or others
                    Some(permissions.mode() & 0o111 != 0)
                } else {
                    Some(false)
                }
            }
            #[cfg(windows)]
            {
                // On Windows, check file extension for executables
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    Some(ext_str == "exe" || ext_str == "bat" || ext_str == "cmd" || ext_str == "com")
                } else {
                    Some(false)
                }
            }
        }
        "-s" => {
            // Check if file exists and has size greater than 0
            match fs::metadata(path) {
                Ok(metadata) => Some(metadata.len() > 0),
                Err(_) => Some(false),
            }
        }
        "-L" => {
            // Check if file is a symbolic link
            #[cfg(unix)]
            {
                match fs::symlink_metadata(path) {
                    Ok(metadata) => Some(metadata.file_type().is_symlink()),
                    Err(_) => Some(false),
                }
            }
            #[cfg(windows)]
            {
                // On Windows, check for reparse points (which include symlinks)
                match fs::metadata(path) {
                    Ok(metadata) => {
                        let file_type = metadata.file_type();
                        // Check if it's a symlink or reparse point
                        // Note: This is a simplification - Windows symlink detection is complex
                        // For now, we'll return false as a placeholder
                        Some(false)
                    }
                    Err(_) => Some(false),
                }
            }
        }
        _ => None,
    }
}

/// Parse a string as a number
fn parse_number(s: &str) -> Result<i64, String> {
    s.parse::<i64>()
        .map_err(|_| format!("integer expression expected: {}", s))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_expressions() {
        // Test empty string
        assert_eq!(evaluate_expression(&[""]).unwrap(), false);
        
        // Test non-empty string
        assert_eq!(evaluate_expression(&["hello"]).unwrap(), true);
        
        // Test -n operator
        assert_eq!(evaluate_expression(&["-n", "hello"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["-n", ""]).unwrap(), false);
        
        // Test -z operator
        assert_eq!(evaluate_expression(&["-z", ""]).unwrap(), true);
        assert_eq!(evaluate_expression(&["-z", "hello"]).unwrap(), false);
        
        // Test = operator
        assert_eq!(evaluate_expression(&["hello", "=", "hello"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["hello", "=", "world"]).unwrap(), false);
        
        // Test != operator
        assert_eq!(evaluate_expression(&["hello", "!=", "world"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["hello", "!=", "hello"]).unwrap(), false);
    }
    
    #[test]
    fn test_numeric_comparisons() {
        // Test -eq operator
        assert_eq!(evaluate_expression(&["5", "-eq", "5"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["5", "-eq", "3"]).unwrap(), false);
        
        // Test -ne operator
        assert_eq!(evaluate_expression(&["5", "-ne", "3"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["5", "-ne", "5"]).unwrap(), false);
        
        // Test -gt operator
        assert_eq!(evaluate_expression(&["5", "-gt", "3"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["3", "-gt", "5"]).unwrap(), false);
        
        // Test -lt operator
        assert_eq!(evaluate_expression(&["3", "-lt", "5"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["5", "-lt", "3"]).unwrap(), false);
        
        // Test -ge operator
        assert_eq!(evaluate_expression(&["5", "-ge", "5"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["5", "-ge", "3"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["3", "-ge", "5"]).unwrap(), false);
        
        // Test -le operator
        assert_eq!(evaluate_expression(&["3", "-le", "3"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["3", "-le", "5"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["5", "-le", "3"]).unwrap(), false);
    }
    
    #[test]
    fn test_negation() {
        // Test ! operator
        assert_eq!(evaluate_expression(&["!", "true"]).unwrap(), false);
        assert_eq!(evaluate_expression(&["!", ""]).unwrap(), true);
        assert_eq!(evaluate_expression(&["!", "-z", "hello"]).unwrap(), true);
    }
    
    #[test]
    fn test_parentheses() {
        // Test parentheses
        assert_eq!(evaluate_expression(&["(", "hello", ")"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["(", "", ")"]).unwrap(), false);
        assert_eq!(evaluate_expression(&["(", "!", "hello", ")"]).unwrap(), false);
    }
    
    #[test]
    fn test_logical_operators() {
        // Test -a (AND) operator
        assert_eq!(evaluate_expression(&["true", "-a", "true"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["true", "-a", ""]).unwrap(), false);
        assert_eq!(evaluate_expression(&["", "-a", "true"]).unwrap(), false);
        
        // Test -o (OR) operator
        assert_eq!(evaluate_expression(&["true", "-o", "true"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["true", "-o", ""]).unwrap(), true);
        assert_eq!(evaluate_expression(&["", "-o", "true"]).unwrap(), true);
        assert_eq!(evaluate_expression(&["", "-o", ""]).unwrap(), false);
    }
}

