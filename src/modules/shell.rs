//! Shell core structure and main execution logic

use std::env;
use std::io::{self, Write};
use std::collections::HashMap;

use crate::modules::builtins;
use crate::modules::expansion;
use crate::modules::parser;
use crate::modules::pipeline;
use crate::modules::redirection;

/// Main shell structure
pub struct Shell {
    /// Current working directory
    pub current_dir: String,
    /// Exit status of last command
    pub last_exit_status: i32,
    /// Is interactive mode
    pub interactive: bool,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
    /// Positional parameters ($1, $2, ...)
    pub positional_params: Vec<String>,
    /// Shell name ($0)
    pub shell_name: String,
    /// Shell options ($-)
    pub options: String,
}

impl Shell {
    /// Create a new shell
    pub fn new() -> Self {
        let current_dir = env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        // Initialize environment variables
        let mut env_vars = HashMap::new();
        for (key, value) in env::vars() {
            env_vars.insert(key, value);
        }
        
        // Get shell name from command line
        let shell_name = env::args()
            .next()
            .unwrap_or_else(|| "rs-dash".to_string());
        
        Self {
            current_dir,
            last_exit_status: 0,
            interactive: false,
            env_vars,
            positional_params: Vec::new(),
            shell_name,
            options: String::new(),  // Empty options for now
        }
    }
    
    /// Set positional parameters
    pub fn set_positional_params(&mut self, params: Vec<String>) {
        self.positional_params = params;
    }
    
    /// Get positional parameter by index (1-based)
    pub fn get_positional_param(&self, index: usize) -> Option<&str> {
        if index == 0 {
            Some(&self.shell_name)
        } else if index <= self.positional_params.len() {
            Some(&self.positional_params[index - 1])
        } else {
            None
        }
    }
    
    /// Get number of positional parameters
    pub fn positional_param_count(&self) -> usize {
        self.positional_params.len()
    }
    
    /// Execute a command line (may contain multiple commands separated by ; && || |)
    pub fn execute_command_line(&mut self, line: &str) -> i32 {
        // Check for pipelines
        if pipeline::has_pipeline(line) {
            // Handle pipeline
            return pipeline::execute_pipeline(self, line);
        }
        
        // Handle regular commands with separators
        let mut commands = parser::split_commands(line);
        let mut last_status = 0;
        
        while let Some((cmd_str, sep)) = commands.next() {
            let status = self.execute_single_command(cmd_str);
            last_status = status;
            
            // Check if we should continue based on separator
            match sep {
                Some(';') => continue, // Always continue
                Some('&') => { // && operator
                    if status != 0 {
                        break; // Stop if command failed
                    }
                }
                Some('|') => { // || operator (simplified)
                    if status == 0 {
                        break; // Stop if command succeeded
                    }
                }
                None => break, // No more commands
                _ => continue,
            }
        }
        
        last_status
    }
    
    /// Execute a single command (no separators, no pipes)
    pub fn execute_single_command(&mut self, cmd_str: &str) -> i32 {
        // Check for variable assignment
        if let Some(equals_pos) = cmd_str.find('=') {
            // Check if it's a valid variable assignment (no spaces before =)
            let before_equals = &cmd_str[..equals_pos];
            if !before_equals.contains(char::is_whitespace) && 
               !before_equals.is_empty() &&
               before_equals.chars().next().unwrap().is_alphabetic() {
                // It's a variable assignment
                let var_name = before_equals.to_string();
                let var_value = cmd_str[equals_pos + 1..].to_string();
                self.env_vars.insert(var_name, var_value);
                let status = 0;
                self.last_exit_status = status;
                return status;
            }
        }
        
        let (cmd, args) = parser::parse_command(cmd_str);
        if cmd.is_empty() {
            let status = 0;
            self.last_exit_status = status;
            return status;
        }
        
        // Expand variables in arguments
        let args = args.iter()
            .map(|arg| expansion::expand_variables(self, arg))
            .collect::<Vec<String>>();
        
        let status = if builtins::is_builtin(&cmd) {
            builtins::execute_builtin(self, &cmd, &args)
        } else {
            self.external_command(&cmd, &args, false, None)
        };
        
        self.last_exit_status = status;
        status
    }
    
    /// Execute a command and capture its output (for command substitution)
    pub fn execute_command_and_capture(&mut self, cmd_str: &str) -> String {
        use std::process::{Command, Stdio};
        
        // Parse the command
        let (cmd, args) = parser::parse_command(cmd_str);
        if cmd.is_empty() {
            return String::new();
        }
        
        // For command substitution, we need to expand variables AND command substitutions
        // because command substitutions can be nested
        let args = args.iter()
            .map(|arg| expansion::expand_variables(self, arg))
            .collect::<Vec<String>>();
        
        if builtins::is_builtin(&cmd) {
            // For builtins, we need to capture output
            // For simplicity, we'll handle common builtins
            match cmd.as_str() {
                "echo" => {
                    // Simulate echo output (without trailing newline for command substitution)
                    return args.join(" ");
                }
                "pwd" => {
                    return self.current_dir.clone();
                }
                _ => {
                    // Execute but don't capture output
                    let _ = builtins::execute_builtin(self, &cmd, &args);
                    return String::new();
                }
            }
        } else {
            // External command
            let path = self.find_in_path(&cmd)
                .unwrap_or_else(|| {
                    eprintln!("{}: command not found", cmd);
                    return String::new();
                });
            
            if path.is_empty() {
                return String::new();
            }
            
            let mut command = Command::new(path);
            for arg in &args {
                command.arg(arg);
            }
            command.current_dir(&self.current_dir);
            command.envs(&self.env_vars);
            command.stdout(Stdio::piped());
            
            match command.spawn() {
                Ok(child) => {
                    match child.wait_with_output() {
                        Ok(output) => {
                            if output.status.success() {
                                String::from_utf8_lossy(&output.stdout).trim().to_string()
                            } else {
                                String::new()
                            }
                        }
                        Err(_) => String::new(),
                    }
                }
                Err(_) => String::new(),
            }
        }
    }
    
    /// Execute external command
    pub fn external_command(&self, cmd: &str, args: &[String], _in_pipeline: bool, stdin_data: Option<&[u8]>) -> i32 {
        // Handle redirections
        let (cmd, args, redirects) = redirection::parse_redirections(cmd, args);
        
        // Try to find command in PATH
        let path = if cmd.contains('/') || cmd.contains('\\') {
            // Absolute or relative path
            cmd.to_string()
        } else {
            // Search in PATH
            match self.find_in_path(&cmd) {
                Some(full_path) => full_path,
                None => {
                    eprintln!("{}: command not found", cmd);
                    return 127;
                }
            }
        };
        
        redirection::execute_with_redirections(&path, &args, &self.current_dir, &self.env_vars, redirects, stdin_data)
    }
    
    /// Find command in PATH
    pub fn find_in_path(&self, cmd: &str) -> Option<String> {
        use std::path::Path;
        
        // Check if command exists as is
        if Path::new(cmd).exists() {
            return Some(cmd.to_string());
        }
        
        // Get PATH from environment variables
        let path_var = self.env_vars.get("PATH")
            .cloned()
            .unwrap_or_default();
        
        // Search in each directory
        for dir in path_var.split(if cfg!(windows) { ';' } else { ':' }) {
            if dir.is_empty() {
                continue;
            }
            
            let full_path = Path::new(dir).join(cmd);
            if full_path.exists() {
                return Some(full_path.to_string_lossy().to_string());
            }
            
            // On Windows, also check with .exe extension
            #[cfg(windows)]
            {
                let full_path_exe = Path::new(dir).join(format!("{}.exe", cmd));
                if full_path_exe.exists() {
                    return Some(full_path_exe.to_string_lossy().to_string());
                }
                
                // Also check with .bat and .cmd extensions
                let full_path_bat = Path::new(dir).join(format!("{}.bat", cmd));
                if full_path_bat.exists() {
                    return Some(full_path_bat.to_string_lossy().to_string());
                }
                
                let full_path_cmd = Path::new(dir).join(format!("{}.cmd", cmd));
                if full_path_cmd.exists() {
                    return Some(full_path_cmd.to_string_lossy().to_string());
                }
            }
        }
        
        None
    }
    
    /// Run interactive shell
    pub fn run_interactive(&mut self) {
        self.interactive = true;
        
        println!("rs-dash v0.1.0");
        println!("Type 'help' for help, 'exit' to quit");
        
        loop {
            // Print prompt
            print!("$ ");
            io::stdout().flush().unwrap();
            
            // Read input
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    // EOF
                    println!();
                    break;
                }
                Ok(_) => {
                    let line = input.trim();
                    if line.is_empty() {
                        continue;
                    }
                    
                    // Execute command line
                    let exit_status = self.execute_command_line(line);
                    self.last_exit_status = exit_status;
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            }
        }
    }
    
    /// Run command from string
    pub fn run_command_string(&mut self, cmd_str: &str) -> i32 {
        let exit_status = self.execute_command_line(cmd_str);
        self.last_exit_status = exit_status;
        exit_status
    }
    
    /// Run script from file
    pub fn run_script_file(&mut self, filename: &str) -> i32 {
        match std::fs::read_to_string(filename) {
            Ok(contents) => {
                let mut last_status = 0;
                for line in contents.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    
                    last_status = self.execute_command_line(line);
                }
                last_status
            }
            Err(e) => {
                eprintln!("Error reading script {}: {}", filename, e);
                1
            }
        }
    }
}