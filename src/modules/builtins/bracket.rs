//! [ builtin command for rs-dash (alias for test)

use crate::modules::shell::Shell;
use super::Builtin;

/// [ builtin command (alias for test)
pub struct BracketBuiltin;

impl Builtin for BracketBuiltin {
    fn name(&self) -> &'static str {
        "["
    }
    
    fn execute(&self, _shell: &mut Shell, args: &[String]) -> i32 {
        // The "[" command is an alias for test
        // We need to pass the args as-is, including the "[" itself
        // The test builtin will handle it
        crate::modules::builtins::test::execute_test(args)
    }
}