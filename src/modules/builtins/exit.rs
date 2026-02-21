//! exit builtin command

use std::process;
use crate::modules::shell::Shell;
use super::Builtin;

/// exit builtin command
pub struct ExitBuiltin;

impl Builtin for ExitBuiltin {
    fn name(&self) -> &'static str {
        "exit"
    }
    
    fn execute(&self, shell: &mut Shell, args: &[String]) -> i32 {
        let exit_code = if args.is_empty() {
            shell.last_exit_status
        } else {
            match args[0].parse::<i32>() {
                Ok(code) => code,
                Err(_) => {
                    eprintln!("exit: {}: numeric argument required", args[0]);
                    2
                }
            }
        };
        
        // Exit process
        process::exit(exit_code);
    }
}