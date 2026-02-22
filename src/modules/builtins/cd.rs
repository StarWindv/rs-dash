//! cd builtin command

use std::env;
use std::fs;
use std::path::Path;
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
    
    
    
    /// Update shell's directory state
    fn update_shell_directory(&self, shell: &mut Shell, logical_path: &str, physical_path: &str, use_physical: bool) {
        // Save current directory as OLDPWD
        let oldpwd = if shell.physical_mode {
            shell.physical_dir.clone()
        } else {
            shell.current_dir.clone()
        };
        shell.env_vars.insert("OLDPWD".to_string(), oldpwd.clone());
        env::set_var("OLDPWD", &oldpwd);
        
        // Update shell state
        shell.current_dir = logical_path.to_string();
        shell.physical_dir = physical_path.to_string();
        shell.physical_mode = use_physical;
        
        // Set PWD environment variable based on mode
        let pwd_value = if use_physical {
            physical_path.to_string()
        } else {
            logical_path.to_string()
        };
        
        shell.env_vars.insert("PWD".to_string(), pwd_value.clone());
        env::set_var("PWD", &pwd_value);
    }
    
    /// Change directory with specified mode
    fn change_directory(&self, shell: &mut Shell, target: &str, use_physical: bool) -> Result<(), String> {
        // First, change to the directory
        env::set_current_dir(target)
            .map_err(|e| format!("{}: {}", target, e))?;
        
        // Get the physical (canonical) path
        let physical_path = match fs::canonicalize(".") {
            Ok(path) => self.normalize_canonical_path(&path),
            Err(e) => {
                // Fallback to current directory
                env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .map_err(|_| format!("Failed to get canonical path: {}", e))?
            }
        };
        
        // For logical mode, we need to construct the logical path
        // This is simplified - in a real implementation, we'd need to track
        // the logical path separately
        let logical_path = if use_physical {
            // For physical mode, logical path is same as physical
            physical_path.clone()
        } else {
            // For logical mode, we construct from current_dir and target
            self.construct_logical_path(shell, target, &physical_path)
        };
        
        // Update shell state
        self.update_shell_directory(shell, &logical_path, &physical_path, use_physical);
        
        Ok(())
    }
    
    /// Get logical path from current directory and target
    fn construct_logical_path(&self, shell: &Shell, target: &str, _physical_path: &str) -> String {
        use std::path::PathBuf;
        
        let path = Path::new(target);
        
        // If target is absolute, use it as is
        if path.is_absolute() {
            return target.to_string();
        }
        
        // Start from current logical directory
        let base_dir = if shell.physical_mode {
            &shell.physical_dir
        } else {
            &shell.current_dir
        };
        
        let mut result = PathBuf::from(base_dir);
        
        // Handle special cases
        if target == "." {
            return base_dir.to_string();
        } else if target == ".." {
            if let Some(parent) = result.parent() {
                return parent.to_string_lossy().to_string();
            }
            return base_dir.to_string();
        }
        
        // For relative paths, push the target
        result.push(target);
        
        // Clean up the path
        self.cleanup_path(&result)
    }
    
    /// Clean up a path by removing . and .. components
    fn cleanup_path(&self, path: &Path) -> String {
        let mut components = Vec::new();
        
        for component in path.components() {
            match component {
                std::path::Component::CurDir => {
                    // Skip .
                }
                std::path::Component::ParentDir => {
                    // Handle ..
                    if !components.is_empty() {
                        components.pop();
                    }
                }
                std::path::Component::Normal(name) => {
                    components.push(name.to_string_lossy().to_string());
                }
                std::path::Component::RootDir => {
                    components.clear();
                    components.push("".to_string()); // Represent root
                }
                std::path::Component::Prefix(_) => {
                    // On Windows, keep the prefix
                    components.push(component.as_os_str().to_string_lossy().to_string());
                }
            }
        }
        
        if components.is_empty() {
            ".".to_string()
        } else if components[0].is_empty() {
            // Absolute path starting with /
            format!("/{}", components[1..].join("/"))
        } else {
            components.join("/")
        }
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
        // After parsing options, we should have at most one directory argument
        // args_start points to the first non-option argument
        // If args_start >= args.len(), no directory argument (use HOME)
        // If args_start + 1 < args.len(), there are extra arguments after the directory
        if args_start < args.len() && args_start + 1 < args.len() {
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