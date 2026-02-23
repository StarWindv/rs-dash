//! rs-dash - A Rust implementation of dash shell
//! Now with pipeline support

use std::env;
use std::process;
use std::rc::Rc;

mod modules;
// mod module_tests;

use modules::shell::Shell;

/// Compile-time constants
const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Main function
fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Create shell
    let mut shell = Shell::new();
    
    // Parse arguments
    if args.len() > 1 {
        if args[1] == "-c" && args.len() > 2 {
            // Execute command string
            // Arguments after -c command are positional parameters
            let mut positional_params = Vec::new();
            if args.len() > 3 {
                positional_params = args[3..].to_vec();
            }
            shell.set_positional_params(positional_params);
            
            let exit_status = shell.run_command_string(&args[2]);
            process::exit(exit_status);
        } else if args[1] == "--help" || args[1] == "-h" {
            // Execute help builtin through registry
            let registry = Rc::clone(&shell.builtin_registry);
            let empty_args: Vec<String> = Vec::new();
            let _ = registry.execute_builtin(&mut shell, "help", &empty_args);
            process::exit(0);
        } else if args[1] == "--version" || args[1] == "-v" {
            println!("{} version {}", NAME, VERSION);
            println!("{}", DESCRIPTION);
            process::exit(0);
        } else {
            // Assume it's a script file
            // Arguments after script name are positional parameters
            let mut positional_params = Vec::new();
            if args.len() > 2 {
                positional_params = args[2..].to_vec();
            }
            shell.set_positional_params(positional_params);
            
            let exit_status = shell.run_script_file(&args[1]);
            process::exit(exit_status);
        }
    }
    
    // Interactive mode
    shell.run_interactive();
    process::exit(shell.last_exit_status);
}

