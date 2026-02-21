//! echo builtin command

use crate::modules::shell::Shell;
use super::Builtin;

/// echo builtin command
pub struct EchoBuiltin;

impl Builtin for EchoBuiltin {
    fn name(&self) -> &'static str {
        "echo"
    }
    
    fn execute(&self, _shell: &mut Shell, args: &[String]) -> i32 {
        let mut first = true;
        for arg in args {
            if !first {
                print!(" ");
            }
            print!("{}", arg);
            first = false;
        }
        println!();
        0
    }
    
    fn execute_in_pipeline(&self, _shell: &mut Shell, args: &[String], is_last: bool) -> i32 {
        let output = args.join(" ");
        if is_last {
            println!("{}", output);
        }
        // In a real pipeline, output would be passed to next command
        0
    }
}