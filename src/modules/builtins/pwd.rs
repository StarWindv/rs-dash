//! pwd builtin command

use std::env;
use std::fs;
use std::path::PathBuf;
use crate::modules::shell::Shell;
use super::Builtin;

/// pwd builtin command
pub struct PwdBuiltin;

impl PwdBuiltin {
    /// Parse pwd options (-L and -P)
    /// Returns (remaining_args_start_index, use_physical_path)
    fn parse_options(&self, args: &[String]) -> (usize, bool) {
        let mut use_physical = false; // -L is default (logical)
        let mut i = 0;
        
        while i < args.len() && args[i].starts_with('-') && args[i] != "--" {
            let arg = &args[i];
            
            if arg == "-L" {
                use_physical = false;
            } else if arg == "-P" {
                use_physical = true;
            } else if arg == "--" {
                i += 1; // Skip --
                break;
            } else {
                // Not a valid option, treat as end of options
                break;
            }
            
            i += 1;
        }
        
        (i, use_physical)
    }
    
    /// Get current directory based on mode
    fn get_current_dir(&self, shell: &Shell, use_physical: bool) -> String {
        if use_physical {
            // Physical mode: get canonical path
            match fs::canonicalize(&shell.current_dir) {
                Ok(path) => {
                    // On Windows, canonicalize returns paths with \\?\ prefix
                    // Remove it for user-friendly display
                    self.normalize_canonical_path(&path)
                }
                Err(_) => {
                    // Fallback to trying to get canonical path from current directory
                    match env::current_dir() {
                        Ok(path) => match fs::canonicalize(path) {
                            Ok(canonical) => self.normalize_canonical_path(&canonical),
                            Err(_) => shell.current_dir.clone(),
                        },
                        Err(_) => shell.current_dir.clone(),
                    }
                }
            }
        } else {
            // Logical mode: use stored current directory
            shell.current_dir.clone()
        }
    }
    
    /// Normalize canonical path (remove Windows \\?\ prefix if present)
    fn normalize_canonical_path(&self, path: &PathBuf) -> String {
        let path_str = path.to_string_lossy().to_string();
        
        // On Windows, canonicalize returns paths with \\?\ prefix
        // Remove it for user-friendly display
        #[cfg(windows)]
        {
            if path_str.starts_with(r"\\?\") {
                // Remove the \\?\ prefix
                return path_str[4..].to_string();
            }
        }
        
        path_str
    }
}

impl Builtin for PwdBuiltin {
    fn name(&self) -> &'static str {
        "pwd"
    }
    
    fn execute(&self, shell: &mut Shell, args: &[String]) -> i32 {
        // Parse options
        let (args_start, use_physical) = self.parse_options(args);
        
        // Check for extra arguments
        if args_start < args.len() {
            eprintln!("pwd: too many arguments");
            return 1;
        }
        
        // Get and print current directory
        let current_dir = self.get_current_dir(shell, use_physical);
        println!("{}", current_dir);
        0
    }
    
    fn execute_in_pipeline(&self, shell: &mut Shell, args: &[String], is_last: bool) -> i32 {
        if is_last {
            // Parse options
            let (args_start, use_physical) = self.parse_options(args);
            
            // Check for extra arguments
            if args_start < args.len() {
                eprintln!("pwd: too many arguments");
                return 1;
            }
            
            // Get and print current directory
            let current_dir = self.get_current_dir(shell, use_physical);
            println!("{}", current_dir);
        }
        // In a pipeline, output would be passed to next command
        0
    }
}