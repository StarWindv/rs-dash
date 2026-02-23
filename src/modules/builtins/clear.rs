//! true builtin command

use crate::modules::shell::Shell;
use super::Builtin;

/// true builtin command
pub struct ClearBuiltin;

impl Builtin for ClearBuiltin {
    fn name(&self) -> &'static str {
        "clear"
    }

    fn execute(&self, _shell: &mut Shell, _args: &[String]) -> i32 {
        "\033[H\033[2J\033[3J"
    }
}

