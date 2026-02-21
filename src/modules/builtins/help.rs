//! help builtin command

use std::env;
use crate::modules::shell::Shell;
use super::Builtin;

/// Compile-time constants
const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// help builtin command
pub struct HelpBuiltin;

impl Builtin for HelpBuiltin {
    fn name(&self) -> &'static str {
        "help"
    }
    
    fn execute(&self, _shell: &mut Shell, _args: &[String]) -> i32 {
        println!("{} v{} - {}", NAME, VERSION, DESCRIPTION);
        println!();
        println!("Built-in commands:");
        println!("  cd [dir]       Change directory");
        println!("  pwd            Print working directory");
        println!("  echo [args]    Print arguments");
        println!("  exit [n]       Exit shell with status n");
        println!("  true           Return success (0)");
        println!("  false          Return failure (1)");
        println!("  help           Show this help");
        println!();
        println!("Variable assignment: VAR=value");
        println!("Variable expansion: $VAR");
        println!();
        println!("Command separators:");
        println!("  ;              Run commands sequentially");
        println!("  &&             Run next command only if previous succeeded");
        println!("  ||             Run next command only if previous failed");
        println!("  |              Pipe output from one command to another");
        println!();
        println!("External commands are also supported.");
        0
    }
}