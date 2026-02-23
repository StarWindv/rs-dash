//! true builtin command

use crate::modules::shell::Shell;
use super::Builtin;

/// true builtin command
pub struct TrueBuiltin;

impl Builtin for TrueBuiltin {
    fn name(&self) -> &'static str {
        "true"
    }

    fn execute(&self, _shell: &mut Shell, _args: &[String]) -> i32 {
        0
    }
    
    fn execute_and_capture(&self, _shell: &mut Shell, _args: &[String]) -> (i32, String) {
        (0, String::new())
    }
}

