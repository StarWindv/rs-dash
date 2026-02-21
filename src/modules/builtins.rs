//! Built-in commands implementation

use std::env;
use std::process;
use crate::modules::shell::Shell;

/// Check if a command is a builtin
pub fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "cd" | "pwd" | "echo" | "exit" | "help" | "true" | "false")
}

/// Execute a builtin command
pub fn execute_builtin(shell: &mut Shell, cmd: &str, args: &[String]) -> i32 {
    match cmd {
        "cd" => cd_command(shell, args),
        "pwd" => pwd_command(shell, args),
        "echo" => echo_command(shell, args),
        "exit" => exit_command(shell, args),
        "help" => help_command(shell, args),
        "true" => true_command(shell, args),
        "false" => false_command(shell, args),
        _ => 127, // Not found
    }
}

/// Execute a builtin command in a pipeline context
pub fn execute_builtin_in_pipeline(shell: &mut Shell, cmd: &str, args: &[String], is_last: bool) -> i32 {
    // For builtins, we need to handle output differently
    match cmd {
        "echo" => {
            let output = args.join(" ");
            if is_last {
                println!("{}", output);
            }
            // In a real pipeline, output would be passed to next command
            0
        }
        "pwd" => {
            if is_last {
                println!("{}", shell.current_dir);
            }
            // In a real pipeline, output would be passed to next command
            0
        }
        _ => {
            // For other builtins, just execute normally
            execute_builtin(shell, cmd, args)
        }
    }
}

/// Change directory command
fn cd_command(shell: &mut Shell, args: &[String]) -> i32 {
    let path = if args.is_empty() {
        // Go to home directory
        match shell.env_vars.get("HOME") {
            Some(home) => home.clone(),
            None => {
                eprintln!("cd: HOME not set");
                return 1;
            }
        }
    } else {
        args[0].clone()
    };
    
    let path = if path == "-" {
        // Go to previous directory
        match shell.env_vars.get("OLDPWD") {
            Some(oldpwd) => oldpwd.clone(),
            None => {
                eprintln!("cd: OLDPWD not set");
                return 1;
            }
        }
    } else {
        path
    };
    
    // Save current directory as OLDPWD
    shell.env_vars.insert("OLDPWD".to_string(), shell.current_dir.clone());
    env::set_var("OLDPWD", &shell.current_dir);
    
    // Change directory
    match env::set_current_dir(&path) {
        Ok(_) => {
            // Update current directory
            shell.current_dir = env::current_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            // Set PWD environment variable
            shell.env_vars.insert("PWD".to_string(), shell.current_dir.clone());
            env::set_var("PWD", &shell.current_dir);
            0
        }
        Err(e) => {
            eprintln!("cd: {}: {}", path, e);
            1
        }
    }
}

/// Print working directory command
fn pwd_command(shell: &Shell, _args: &[String]) -> i32 {
    println!("{}", shell.current_dir);
    0
}

/// Echo command
fn echo_command(_shell: &Shell, args: &[String]) -> i32 {
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

/// Exit command
fn exit_command(shell: &Shell, args: &[String]) -> i32 {
    let exit_code = if args.is_empty() {
        shell.last_exit_status
    } else {
        match args[0].parse::<i32>() {
            Ok(code) => code,
            Err(_) => {
                eprintln!("exit: {}: numeric argument required", args[0]);
                2
            }
        }
    };
    
    // Exit process
    process::exit(exit_code);
}

/// True command (always succeeds)
fn true_command(_shell: &Shell, _args: &[String]) -> i32 {
    0
}

/// False command (always fails)
fn false_command(_shell: &Shell, _args: &[String]) -> i32 {
    1
}

/// Help command
pub fn help_command(_shell: &Shell, _args: &[String]) -> i32 {
    println!("rs-dash - A Rust implementation of dash shell");
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