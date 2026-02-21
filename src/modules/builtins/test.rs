//! test builtin command for rs-dash
//! Implements the test command and [ alias

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
    let mut test_args: Vec<&String> = if is_bracket {
        arg_iter.collect()
    } else {
        args.iter().collect()
    };
    
    // If called as [, last argument should be "]"
    if is_bracket {
        if let Some(last) = test_args.last() {
            if *last == "]" {
                test_args.pop();
            } else {
                eprintln!("test: missing ]");
                return 2;
            }
        } else {
            eprintln!("test: missing ]");
            return 2;
        }
    }
    
    // Simple test implementation
    // For now, just handle -n (string is not empty) and -z (string is empty)
    if test_args.is_empty() {
        // test with no arguments returns false (exit code 1)
        return 1;
    }
    
    // Handle common test expressions
    match test_args[0].as_str() {
        "-n" => {
            if test_args.len() > 1 {
                // -n string: true if string is not empty
                if test_args[1].is_empty() {
                    1
                } else {
                    0
                }
            } else {
                eprintln!("test: -n requires an argument");
                2
            }
        }
        "-z" => {
            if test_args.len() > 1 {
                // -z string: true if string is empty
                if test_args[1].is_empty() {
                    0
                } else {
                    1
                }
            } else {
                eprintln!("test: -z requires an argument");
                2
            }
        }
        "=" => {
            if test_args.len() > 2 {
                // string1 = string2: true if strings are equal
                if test_args[1] == test_args[2] {
                    0
                } else {
                    1
                }
            } else {
                eprintln!("test: = requires two arguments");
                2
            }
        }
        "!=" => {
            if test_args.len() > 2 {
                // string1 != string2: true if strings are not equal
                if test_args[1] != test_args[2] {
                    0
                } else {
                    1
                }
            } else {
                eprintln!("test: != requires two arguments");
                2
            }
        }
        _ => {
            // Single argument: true if argument is not empty
            if test_args[0].is_empty() {
                1
            } else {
                0
            }
        }
    }
}

/// Check if command is test or [
pub fn is_test_command(cmd: &str) -> bool {
    cmd == "test" || cmd == "["
}