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
        // We need to pass the args as-is, but we need to add "[" as the first argument
        // because test expects it
        let mut full_args = vec!["[".to_string()];
        full_args.extend_from_slice(args);
        crate::modules::builtins::test::execute_test(&full_args)
    }
}