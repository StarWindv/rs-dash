//! `return` builtin command
//! Returns from a function with an optional exit status

use crate::modules::shell::Shell;
use super::Builtin;

/// Return builtin command
pub struct ReturnBuiltin;

impl Builtin for ReturnBuiltin {
    /// Get the name of the builtin command
    fn name(&self) -> &'static str {
        "return"
    }
    
    /// Execute the `return` builtin
    fn execute(&self, shell: &mut Shell, args: &[String]) -> i32 {
        // Parse exit status if provided
        let exit_status = if args.is_empty() {
            shell.last_exit_status
        } else {
            match args[0].parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("return: numeric argument required");
                    255
                }
            }
        };
        
        // Set a special flag or use a different mechanism to signal return
        // For now, we'll just return the status and let the caller handle it
        exit_status
    }
    
    /// Execute in pipeline context
    fn execute_in_pipeline(&self, shell: &mut Shell, args: &[String], _is_last: bool) -> i32 {
        // Same as regular execution for return
        self.execute(shell, args)
    }
}