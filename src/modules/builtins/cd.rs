//! cd builtin command

use std::env;
use crate::modules::shell::Shell;
use super::Builtin;

/// cd builtin command
pub struct CdBuiltin;

impl Builtin for CdBuiltin {
    fn name(&self) -> &'static str {
        "cd"
    }
    
    fn execute(&self, shell: &mut Shell, args: &[String]) -> i32 {
        let path = if args.is_empty() {
            // Go to home directory
            match shell.env_vars.get("HOME") {
                Some(home) => home.clone(),
                None => {
                    eprintln!("cd: HOME not set");
                    return 1;
                }
            }
        } else {
            args[0].clone()
        };
        
        let path = if path == "-" {
            // Go to previous directory
            match shell.env_vars.get("OLDPWD") {
                Some(oldpwd) => oldpwd.clone(),
                None => {
                    eprintln!("cd: OLDPWD not set");
                    return 1;
                }
            }
        } else {
            path
        };
        
        // Save current directory as OLDPWD
        shell.env_vars.insert("OLDPWD".to_string(), shell.current_dir.clone());
        env::set_var("OLDPWD", &shell.current_dir);
        
        // Change directory
        match env::set_current_dir(&path) {
            Ok(_) => {
                // Update current directory
                shell.current_dir = env::current_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                // Set PWD environment variable
                shell.env_vars.insert("PWD".to_string(), shell.current_dir.clone());
                env::set_var("PWD", &shell.current_dir);
                0
            }
            Err(e) => {
                eprintln!("cd: {}: {}", path, e);
                1
            }
        }
    }
}