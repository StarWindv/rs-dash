//! test builtin command for rs-dash
//! Implements the test command and [ alias

use std::path::Path;
use crate::modules::shell::Shell;
use super::Builtin;

/// Test builtin command
pub struct TestBuiltin;

impl Builtin for TestBuiltin {
    fn name(&self) -> &'static str {
        "test"
    }
    
    fn execute(&self, _shell: &mut Shell, args: &[String]) -> i32 {
        execute_test(args)
    }
}

/// Execute the test builtin command
pub fn execute_test(args: &[String]) -> i32 {
    // Simple implementation for now
    // Supports: test expression or [ expression ]
    
    let mut arg_iter = args.iter();
    let first_arg = arg_iter.next();
    
    // Check if called as [ (first arg is "[" )
    let is_bracket = first_arg == Some(&"[".to_string());
    
    // Skip "[" if present
    let test_args: Vec<&String> = if is_bracket {
        arg_iter.collect()
    } else {
        args.iter().collect()
    };
    
    // If called as [, last argument should be "]"
    if is_bracket {
        if let Some(last) = test_args.last() {
            if *last == "]" {
                // OK, we'll handle it in parse_test_expression
            } else {
                eprintln!("test: missing ]");
                return 2;
            }
        } else {
            eprintln!("test: missing ]");
            return 2;
        }
    }
    
    // Empty test returns false
    if test_args.is_empty() {
        return 1;
    }
    
    // Parse test expression
    match parse_test_expression(&test_args, is_bracket) {
        Ok(result) => {
            if result { 0 } else { 1 }
        }
        Err(err_code) => {
            eprintln!("test: {}", err_code);
            err_code
        }
    }
}

/// Parse test expression and evaluate it
fn parse_test_expression(args: &[&String], is_bracket: bool) -> Result<bool, i32> {
    let mut args = args.to_vec();
    
    // Remove trailing "]" if present (for bracket syntax)
    if is_bracket {
        if let Some(last) = args.last() {
            if *last == "]" {
                args.pop();
            }
        }
    }
    
    if args.is_empty() {
        return Ok(false);
    }
    
    let mut index = 0;
    
    // Handle negation
    let negate = if args[0] == "!" {
        index += 1;
        true
    } else {
        false
    };
    
    if index >= args.len() {
        return Err(2); // Syntax error
    }
    
    // Check for unary operators
    if args[index].starts_with('-') && args[index].len() == 2 {
        let op = &args[index][1..2];
        index += 1;
        
        if index >= args.len() {
            return Err(2); // Missing argument
        }
        
        let arg = args[index].as_str();
        let result = match op {
            "n" => !arg.is_empty(),          // -n: string is not empty
            "z" => arg.is_empty(),           // -z: string is empty
            "f" => Path::new(arg).is_file(), // -f: file exists and is regular file
            "d" => Path::new(arg).is_dir(),  // -d: directory exists
            "e" => Path::new(arg).exists(),  // -e: file exists
            "r" => {                         // -r: file exists and is readable
                let path = Path::new(arg);
                path.exists() && std::fs::metadata(path).map(|m| !m.permissions().readonly()).unwrap_or(false)
            }
            "w" => {                         // -w: file exists and is writable
                let path = Path::new(arg);
                path.exists() && std::fs::metadata(path).map(|m| !m.permissions().readonly()).unwrap_or(false)
            }
            "x" => {                         // -x: file exists and is executable
                let path = Path::new(arg);
                path.exists()
            }
            _ => {
                // Unknown operator
                return Err(2);
            }
        };
        
        // Apply negation if needed
        let final_result = if negate { !result } else { result };
        
        // Check for extra arguments
        if index + 1 < args.len() {
            return Err(2); // Too many arguments
        }
        
        return Ok(final_result);
    }
    
    // Handle binary operators
    if args.len() - index == 3 {
        let left = args[index].as_str();
        let op = args[index + 1].as_str();
        let right = args[index + 2].as_str();
        
        let result = match op {
            "=" => left == right,        // string equality
            "!=" => left != right,       // string inequality
            "-eq" => parse_int(left) == parse_int(right),  // integer equality
            "-ne" => parse_int(left) != parse_int(right),  // integer inequality
            "-lt" => parse_int(left) < parse_int(right),   // less than
            "-le" => parse_int(left) <= parse_int(right),  // less than or equal
            "-gt" => parse_int(left) > parse_int(right),   // greater than
            "-ge" => parse_int(left) >= parse_int(right),  // greater than or equal
            _ => {
                // Try as file comparison
                if op == "-nt" {
                    // newer than
                    let left_path = Path::new(left);
                    let right_path = Path::new(right);
                    if left_path.exists() && right_path.exists() {
                        let left_modified = left_path.metadata().ok().and_then(|m| m.modified().ok());
                        let right_modified = right_path.metadata().ok().and_then(|m| m.modified().ok());
                        match (left_modified, right_modified) {
                            (Some(l), Some(r)) => l > r,
                            _ => false
                        }
                    } else {
                        false
                    }
                } else if op == "-ot" {
                    // older than
                    let left_path = Path::new(left);
                    let right_path = Path::new(right);
                    if left_path.exists() && right_path.exists() {
                        let left_modified = left_path.metadata().ok().and_then(|m| m.modified().ok());
                        let right_modified = right_path.metadata().ok().and_then(|m| m.modified().ok());
                        match (left_modified, right_modified) {
                            (Some(l), Some(r)) => l < r,
                            _ => false
                        }
                    } else {
                        false
                    }
                } else if op == "-ef" {
                    // same device and inode numbers
                    let left_path = Path::new(left);
                    let right_path = Path::new(right);
                    if left_path.exists() && right_path.exists() {
                        // Simplified: just compare canonical paths
                        left_path.canonicalize().ok() == right_path.canonicalize().ok()
                    } else {
                        false
                    }
                } else {
                    return Err(2); // Unknown operator
                }
            }
        };
        
        let final_result = if negate { !result } else { result };
        return Ok(final_result);
    }
    
    // Single argument: true if not empty
    if args.len() - index == 1 {
        let result = !args[index].is_empty();
        let final_result = if negate { !result } else { result };
        return Ok(final_result);
    }
    
    // Syntax error
    Err(2)
}

/// Parse string as integer for test comparisons
fn parse_int(s: &str) -> i64 {
    s.parse().unwrap_or(0)
}

/// Check if command is test or [
pub fn is_test_command(cmd: &str) -> bool {
    cmd == "test" || cmd == "["
}