//! false builtin command

use crate::modules::shell::Shell;
use super::Builtin;

/// false builtin command
pub struct FalseBuiltin;

impl Builtin for FalseBuiltin {
    fn name(&self) -> &'static str {
        "false"
    }
    
    fn execute(&self, _shell: &mut Shell, _args: &[String]) -> i32 {
        1
    }
}