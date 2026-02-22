//! cd builtin command

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use crate::modules::shell::Shell;
use super::Builtin;

/// cd builtin command
pub struct CdBuiltin;

impl CdBuiltin {
    /// Parse cd options (-L and -P)
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
                // Not a valid option, treat as directory argument
                break;
            }
            
            i += 1;
        }
        
        (i, use_physical)
    }
    
    /// Get target directory from arguments
    fn get_target_dir(&self, shell: &Shell, args: &[String], start_idx: usize) -> Result<String, String> {
        let target = if start_idx >= args.len() {
            // Go to home directory
            match shell.env_vars.get("HOME") {
                Some(home) => home.clone(),
                None => return Err("HOME not set".to_string()),
            }
        } else {
            let arg = &args[start_idx];
            if arg == "-" {
                // Go to previous directory
                match shell.env_vars.get("OLDPWD") {
                    Some(oldpwd) => oldpwd.clone(),
                    None => return Err("OLDPWD not set".to_string()),
                }
            } else {
                arg.clone()
            }
        };
        
        Ok(target)
    }
    
    /// Change directory with specified mode
    fn change_directory(&self, shell: &mut Shell, target: &str, use_physical: bool) -> Result<(), String> {
        // Save current directory as OLDPWD
        let oldpwd = shell.current_dir.clone();
        shell.env_vars.insert("OLDPWD".to_string(), oldpwd.clone());
        env::set_var("OLDPWD", &oldpwd);
        
        // Change directory based on mode
        if use_physical {
            self.change_directory_physical(target)
        } else {
            self.change_directory_logical(target)
        }?;
        
        // Update current directory
        let new_dir = if use_physical {
            // For physical mode, we need to get the canonical path
            match fs::canonicalize(".") {
                Ok(path) => {
                    // On Windows, canonicalize returns paths with \\?\ prefix
                    // We should remove it for user-friendly display
                    self.normalize_canonical_path(&path)
                }
                Err(_) => {
                    // Fallback to current_dir from env
                    env::current_dir()
                        .unwrap_or_else(|_| PathBuf::from(target))
                        .to_string_lossy()
                        .to_string()
                }
            }
        } else {
            // For logical mode, use the path as is
            env::current_dir()
                .unwrap_or_else(|_| PathBuf::from(target))
                .to_string_lossy()
                .to_string()
        };
        
        shell.current_dir = new_dir.clone();
        
        // Set PWD environment variable
        shell.env_vars.insert("PWD".to_string(), new_dir.clone());
        env::set_var("PWD", &new_dir);
        
        Ok(())
    }
    
    /// Change directory in logical mode (follow symbolic links)
    fn change_directory_logical(&self, target: &str) -> Result<(), String> {
        env::set_current_dir(target)
            .map_err(|e| format!("{}: {}", target, e))
    }
    
    /// Change directory in physical mode (resolve symbolic links)
    fn change_directory_physical(&self, target: &str) -> Result<(), String> {
        // First try to get canonical path
        let canonical_path = fs::canonicalize(target)
            .map_err(|e| format!("Failed to resolve path {}: {}", target, e))?;
        
        // Then change to the canonical path
        env::set_current_dir(&canonical_path)
            .map_err(|e| format!("{}: {}", canonical_path.display(), e))
    }
    
    /// Normalize canonical path (remove Windows \\?\ prefix if present)
    fn normalize_canonical_path(&self, path: &Path) -> String {
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

impl Builtin for CdBuiltin {
    fn name(&self) -> &'static str {
        "cd"
    }
    
    fn execute(&self, shell: &mut Shell, args: &[String]) -> i32 {
        // Parse options
        let (args_start, use_physical) = self.parse_options(args);
        
        // Get target directory
        let target = match self.get_target_dir(shell, args, args_start) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("cd: {}", e);
                return 1;
            }
        };
        
        // Check for extra arguments
        if args_start + 1 < args.len() {
            eprintln!("cd: too many arguments");
            return 1;
        }
        
        // Change directory
        match self.change_directory(shell, &target, use_physical) {
            Ok(()) => 0,
            Err(e) => {
                eprintln!("cd: {}", e);
                1
            }
        }
    }
}