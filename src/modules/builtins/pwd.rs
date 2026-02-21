//! pwd builtin command

use crate::modules::shell::Shell;
use super::Builtin;

/// pwd builtin command
pub struct PwdBuiltin;

impl Builtin for PwdBuiltin {
    fn name(&self) -> &'static str {
        "pwd"
    }
    
    fn execute(&self, shell: &mut Shell, _args: &[String]) -> i32 {
        println!("{}", shell.current_dir);
        0
    }
    
    fn execute_in_pipeline(&self, shell: &mut Shell, _args: &[String], is_last: bool) -> i32 {
        if is_last {
            println!("{}", shell.current_dir);
        }
        // In a real pipeline, output would be passed to next command
        0
    }
}