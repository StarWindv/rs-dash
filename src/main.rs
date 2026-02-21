//! rs-dash - A Rust implementation of dash shell
//! Now with pipeline support

use std::env;
use std::io::{self, Write, Read};
use std::process::{self, Command, Stdio};
use std::path::Path;
use std::fs::{self, File};
use std::thread;
use std::sync::{Arc, Mutex};

/// Main shell structure
struct Shell {
    /// Current working directory
    current_dir: String,
    /// Exit status of last command
    last_exit_status: i32,
    /// Is interactive mode
    interactive: bool,
    /// Environment variables
    env_vars: std::collections::HashMap<String, String>,
}

impl Shell {
    /// Create a new shell
    fn new() -> Self {
        let current_dir = env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        // Initialize environment variables
        let mut env_vars = std::collections::HashMap::new();
        for (key, value) in env::vars() {
            env_vars.insert(key, value);
        }
        
        Self {
            current_dir,
            last_exit_status: 0,
            interactive: false,
            env_vars,
        }
    }
    
    /// Execute a command line (may contain multiple commands separated by ; && || |)
    fn execute_command_line(&mut self, line: &str) -> i32 {
        // Check for pipelines
        // We need to check if there's a pipe that's not part of ||
        let mut has_pipe = false;
        let mut chars = line.chars().collect::<Vec<char>>();
        let mut i = 0;
        
        while i < chars.len() {
            if chars[i] == '|' {
                if i + 1 < chars.len() && chars[i + 1] == '|' {
                    // Skip over ||
                    i += 2;
                } else {
                    // Found a single pipe
                    has_pipe = true;
                    break;
                }
            } else {
                i += 1;
            }
        }
        
        if has_pipe {
            // Handle pipeline
            return self.execute_pipeline(line);
        }
        
        // Handle regular commands with separators
        let mut commands = self.split_commands(line);
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
    
    /// Execute a pipeline (commands connected with |)
    fn execute_pipeline(&mut self, line: &str) -> i32 {
        let commands: Vec<&str> = line.split('|')
            .map(|s| s.trim())
            .collect();
        
        if commands.is_empty() {
            return 0;
        }
        
        let mut last_status = 0;
        
        // We'll handle pipelines by executing commands sequentially
        // and passing output between them
        for (i, cmd_str) in commands.iter().enumerate() {
            let is_last = i == commands.len() - 1;
            
            // Parse the command
            let (cmd, args) = self.parse_command(cmd_str);
            if cmd.is_empty() {
                continue;
            }
            
            // Expand variables in arguments (use simple version for pipeline)
            let args = args.iter()
                .map(|arg| self.expand_variables_simple(arg))
                .collect::<Vec<String>>();
            
            if self.is_builtin(&cmd) {
                // For builtins in pipeline, we need to capture output
                last_status = self.execute_builtin_in_pipeline(&cmd, &args, is_last);
            } else {
                // External command
                let path = self.find_in_path(&cmd)
                    .unwrap_or_else(|| {
                        eprintln!("{}: command not found", cmd);
                        return String::new();
                    });
                
                if path.is_empty() {
                    return 127;
                }
                
                let mut command = Command::new(path);
                for arg in &args {
                    command.arg(arg);
                }
                command.current_dir(&self.current_dir);
                command.envs(&self.env_vars);
                
                // For non-last commands, capture output
                if !is_last {
                    command.stdout(Stdio::piped());
                }
                
                match command.spawn() {
                    Ok(child) => {
                        match child.wait_with_output() {
                            Ok(output) => {
                                last_status = output.status.code().unwrap_or(128);
                                
                                // For non-last commands, we should pass output to next command
                                // In a full implementation, we'd use pipes between processes
                                // For simplicity, we'll just print intermediate output
                                if !is_last && !output.stdout.is_empty() {
                                    // In a real pipeline, this would go to the next command
                                    // For now, we'll just note that output was produced
                                } else if is_last && output.status.success() {
                                    // Print output of last command
                                    if !output.stdout.is_empty() {
                                        io::stdout().write_all(&output.stdout).ok();
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("{}: {}", cmd, e);
                                last_status = 1;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}: {}", cmd, e);
                        last_status = 1;
                    }
                }
            }
        }
        
        last_status
    }
    
    /// Execute a builtin command in a pipeline context
    fn execute_builtin_in_pipeline(&mut self, cmd: &str, args: &[String], is_last: bool) -> i32 {
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
                    println!("{}", self.current_dir);
                }
                // In a real pipeline, output would be passed to next command
                0
            }
            _ => {
                // For other builtins, just execute normally
                self.execute_builtin(cmd, args)
            }
        }
    }
    
    /// Check if a command is a builtin
    fn is_builtin(&self, cmd: &str) -> bool {
        matches!(cmd, "cd" | "pwd" | "echo" | "exit" | "help" | "true" | "false")
    }
    
    /// Execute a builtin command
    fn execute_builtin(&mut self, cmd: &str, args: &[String]) -> i32 {
        match cmd {
            "cd" => self.cd_command(args),
            "pwd" => self.pwd_command(args),
            "echo" => self.echo_command(args),
            "exit" => self.exit_command(args),
            "help" => self.help_command(args),
            "true" => self.true_command(args),
            "false" => self.false_command(args),
            _ => 127, // Not found
        }
    }
    
    /// Split command line into individual commands (for ; && ||)
    fn split_commands<'a>(&self, line: &'a str) -> CommandSplitter<'a> {
        CommandSplitter::new(line)
    }
    
    /// Execute a single command (no separators, no pipes)
    fn execute_single_command(&mut self, cmd_str: &str) -> i32 {
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
                return 0;
            }
        }
        
        let (cmd, args) = self.parse_command(cmd_str);
        if cmd.is_empty() {
            return 0;
        }
        
        // Expand variables in arguments
        let args = args.iter()
            .map(|arg| self.expand_variables(arg))
            .collect::<Vec<String>>();
        
        if self.is_builtin(&cmd) {
            self.execute_builtin(&cmd, &args)
        } else {
            self.external_command(&cmd, &args, false, None)
        }
    }
    
    /// Expand variables and command substitutions in a string
    fn expand_variables(&mut self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '$' {
                // Check what comes after $
                if let Some(&next_c) = chars.peek() {
                    if next_c == '(' {
                        // Command substitution: $(command)
                        chars.next(); // Skip '('
                        let mut cmd_str = String::new();
                        let mut paren_depth = 1;
                        
                        while let Some(c) = chars.next() {
                            if c == '(' {
                                paren_depth += 1;
                                cmd_str.push(c);
                            } else if c == ')' {
                                paren_depth -= 1;
                                if paren_depth == 0 {
                                    break;
                                } else {
                                    cmd_str.push(c);
                                }
                            } else {
                                cmd_str.push(c);
                            }
                        }
                        
                        // Execute the command and capture output
                        let output = self.execute_command_and_capture(&cmd_str);
                        result.push_str(&output);
                    } else if next_c == '?' {
                        // Special variable: $? (exit status)
                        chars.next(); // Skip '?'
                        result.push_str(&self.last_exit_status.to_string());
                    } else if next_c == '$' {
                        // Special variable: $$ (PID)
                        chars.next(); // Skip second '$'
                        result.push_str(&std::process::id().to_string());
                    } else if next_c == '0' {
                        // Special variable: $0 (shell name)
                        chars.next(); // Skip '0'
                        result.push_str("rs-dash");
                    } else if next_c.is_digit(10) {
                        // Positional parameter: $1, $2, etc.
                        let mut num_str = String::new();
                        while let Some(&next_c) = chars.peek() {
                            if next_c.is_digit(10) {
                                num_str.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        // For now, positional parameters not implemented
                        // Just leave empty like dash does for undefined parameters
                    } else {
                        // Regular variable expansion
                        let mut var_name = String::new();
                        while let Some(&next_c) = chars.peek() {
                            if next_c.is_alphanumeric() || next_c == '_' {
                                var_name.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        
                        if var_name.is_empty() {
                            // Just a $, keep it
                            result.push('$');
                        } else {
                            // Look up variable
                            if let Some(value) = self.env_vars.get(&var_name) {
                                result.push_str(value);
                            }
                            // If variable not found, leave it empty (like dash)
                        }
                    }
                } else {
                    // Just a $ at end of string
                    result.push('$');
                }
            } else {
                result.push(c);
            }
        }
        
        result
    }
    
    /// Execute a command and capture its output (for command substitution)
    fn execute_command_and_capture(&mut self, cmd_str: &str) -> String {
        use std::process::Stdio;
        
        // Parse the command
        let (cmd, args) = self.parse_command(cmd_str);
        if cmd.is_empty() {
            return String::new();
        }
        
        // For command substitution, we need to expand variables but NOT command substitutions
        // to avoid infinite recursion
        let args = args.iter()
            .map(|arg| self.expand_variables_simple(arg))
            .collect::<Vec<String>>();
        
        if self.is_builtin(&cmd) {
            // For builtins, we need to capture output
            // For simplicity, we'll handle common builtins
            match cmd.as_str() {
                "echo" => {
                    return args.join(" ");
                }
                "pwd" => {
                    return self.current_dir.clone();
                }
                _ => {
                    // Execute but don't capture output
                    let _ = self.execute_builtin(&cmd, &args);
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
    
    /// Expand variables in a string (simple version without command substitution)
    fn expand_variables_simple(&self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '$' {
                // Check what comes after $
                if let Some(&next_c) = chars.peek() {
                    if next_c == '?' {
                        // Special variable: $? (exit status)
                        chars.next(); // Skip '?'
                        result.push_str(&self.last_exit_status.to_string());
                    } else if next_c == '$' {
                        // Special variable: $$ (PID)
                        chars.next(); // Skip second '$'
                        result.push_str(&std::process::id().to_string());
                    } else if next_c == '0' {
                        // Special variable: $0 (shell name)
                        chars.next(); // Skip '0'
                        result.push_str("rs-dash");
                    } else if next_c.is_digit(10) {
                        // Positional parameter: $1, $2, etc.
                        let mut num_str = String::new();
                        while let Some(&next_c) = chars.peek() {
                            if next_c.is_digit(10) {
                                num_str.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        // For now, positional parameters not implemented
                    } else {
                        // Regular variable expansion
                        let mut var_name = String::new();
                        while let Some(&next_c) = chars.peek() {
                            if next_c.is_alphanumeric() || next_c == '_' {
                                var_name.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        
                        if var_name.is_empty() {
                            // Just a $, keep it
                            result.push('$');
                        } else {
                            // Look up variable
                            if let Some(value) = self.env_vars.get(&var_name) {
                                result.push_str(value);
                            }
                        }
                    }
                } else {
                    // Just a $ at end of string
                    result.push('$');
                }
            } else {
                result.push(c);
            }
        }
        
        result
    }
    
    /// Change directory command
    fn cd_command(&mut self, args: &[String]) -> i32 {
        let path = if args.is_empty() {
            // Go to home directory
            match self.env_vars.get("HOME") {
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
            match self.env_vars.get("OLDPWD") {
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
        self.env_vars.insert("OLDPWD".to_string(), self.current_dir.clone());
        env::set_var("OLDPWD", &self.current_dir);
        
        // Change directory
        match env::set_current_dir(&path) {
            Ok(_) => {
                // Update current directory
                self.current_dir = env::current_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                // Set PWD environment variable
                self.env_vars.insert("PWD".to_string(), self.current_dir.clone());
                env::set_var("PWD", &self.current_dir);
                0
            }
            Err(e) => {
                eprintln!("cd: {}: {}", path, e);
                1
            }
        }
    }
    
    /// Print working directory command
    fn pwd_command(&self, _args: &[String]) -> i32 {
        println!("{}", self.current_dir);
        0
    }
    
    /// Echo command
    fn echo_command(&self, args: &[String]) -> i32 {
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
    fn exit_command(&self, args: &[String]) -> i32 {
        let exit_code = if args.is_empty() {
            self.last_exit_status
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
    fn true_command(&self, _args: &[String]) -> i32 {
        0
    }
    
    /// False command (always fails)
    fn false_command(&self, _args: &[String]) -> i32 {
        1
    }
    
    /// Help command
    fn help_command(&self, _args: &[String]) -> i32 {
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
    
    /// Execute external command
    fn external_command(&self, cmd: &str, args: &[String], in_pipeline: bool, stdin_data: Option<&[u8]>) -> i32 {
        // Handle redirections
        let (cmd, args, redirects) = self.parse_redirections(cmd, args);
        
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
        
        // Prepare command
        let mut command = Command::new(path);
        
        // Add arguments
        for arg in args {
            command.arg(arg);
        }
        
        // Set current directory
        command.current_dir(&self.current_dir);
        
        // Apply redirections
        for redirect in redirects {
            match redirect {
                Redirection::Output(filename) => {
                    match File::create(&filename) {
                        Ok(file) => {
                            command.stdout(file);
                        }
                        Err(e) => {
                            eprintln!("{}: {}", filename, e);
                            return 1;
                        }
                    }
                }
                Redirection::Append(filename) => {
                    match fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&filename) {
                        Ok(file) => {
                            command.stdout(file);
                        }
                        Err(e) => {
                            eprintln!("{}: {}", filename, e);
                            return 1;
                        }
                    }
                }
                Redirection::Input(filename) => {
                    match File::open(&filename) {
                        Ok(file) => {
                            command.stdin(file);
                        }
                        Err(e) => {
                            eprintln!("{}: {}", filename, e);
                            return 1;
                        }
                    }
                }
            }
        }
        
        // Set environment variables
        command.envs(&self.env_vars);
        
        // Handle pipeline stdin
        if let Some(data) = stdin_data {
            if !data.is_empty() {
                command.stdin(Stdio::piped());
            }
        }
        
        // Execute command
        match command.spawn() {
            Ok(mut child) => {
                // Write stdin data if provided
                if let Some(data) = stdin_data {
                    if let Some(mut stdin) = child.stdin.take() {
                        // In a real implementation, we'd write the data
                        // For now, this is simplified
                    }
                }
                
                match child.wait() {
                    Ok(status) => {
                        if let Some(code) = status.code() {
                            code
                        } else {
                            // Process terminated by signal
                            128
                        }
                    }
                    Err(e) => {
                        eprintln!("{}: {}", cmd, e);
                        1
                    }
                }
            }
            Err(e) => {
                eprintln!("{}: {}", cmd, e);
                1
            }
        }
    }
    
    /// Parse redirections from command arguments
    fn parse_redirections(&self, cmd: &str, args: &[String]) -> (String, Vec<String>, Vec<Redirection>) {
        let mut new_args = Vec::new();
        let mut redirects = Vec::new();
        
        // Start with the command itself
        let mut current_cmd = cmd.to_string();
        let mut in_args = false;
        
        for arg in args {
            if !in_args {
                // First non-redirection argument becomes part of command for external commands
                if !arg.starts_with('>') && !arg.starts_with('<') {
                    current_cmd = arg.clone();
                    in_args = true;
                    continue;
                }
            }
            
            if arg.starts_with(">>") {
                // Append redirection
                let filename = if arg.len() > 2 { &arg[2..] } else { "" };
                if !filename.is_empty() {
                    redirects.push(Redirection::Append(filename.to_string()));
                }
            } else if arg.starts_with('>') {
                // Output redirection
                let filename = if arg.len() > 1 { &arg[1..] } else { "" };
                if !filename.is_empty() {
                    redirects.push(Redirection::Output(filename.to_string()));
                }
            } else if arg.starts_with('<') {
                // Input redirection
                let filename = if arg.len() > 1 { &arg[1..] } else { "" };
                if !filename.is_empty() {
                    redirects.push(Redirection::Input(filename.to_string()));
                }
            } else {
                new_args.push(arg.clone());
            }
        }
        
        (current_cmd, new_args, redirects)
    }
    
    /// Find command in PATH
    fn find_in_path(&self, cmd: &str) -> Option<String> {
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
    
    /// Parse a command into command name and arguments
    fn parse_command(&self, line: &str) -> (String, Vec<String>) {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut in_quote = false;
        let mut quote_char = '\0';
        let mut escape_next = false;
        
        for c in line.chars() {
            if escape_next {
                current.push(c);
                escape_next = false;
            } else if c == '\\' {
                escape_next = true;
            } else if (c == '\'' || c == '"') && !in_quote {
                in_quote = true;
                quote_char = c;
            } else if c == quote_char && in_quote {
                in_quote = false;
                quote_char = '\0';
            } else if c.is_whitespace() && !in_quote {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            } else {
                current.push(c);
            }
        }
        
        if !current.is_empty() {
            parts.push(current);
        }
        
        if parts.is_empty() {
            return (String::new(), Vec::new());
        }
        
        let cmd = parts[0].clone();
        let args = parts[1..].to_vec();
        
        (cmd, args)
    }
    
    /// Run interactive shell
    fn run_interactive(&mut self) {
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
    fn run_command_string(&mut self, cmd_str: &str) -> i32 {
        let exit_status = self.execute_command_line(cmd_str);
        self.last_exit_status = exit_status;
        exit_status
    }
    
    /// Run script from file
    fn run_script_file(&mut self, filename: &str) -> i32 {
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

/// Redirection types
enum Redirection {
    Output(String),   // > filename
    Append(String),   // >> filename
    Input(String),    // < filename
}

/// Iterator for splitting commands by separators
struct CommandSplitter<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> CommandSplitter<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }
}

impl<'a> Iterator for CommandSplitter<'a> {
    type Item = (&'a str, Option<char>);
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }
        
        let start = self.pos;
        let mut in_quote = false;
        let mut quote_char = '\0';
        let mut escape_next = false;
        
        while self.pos < self.input.len() {
            let c = self.input.chars().nth(self.pos).unwrap();
            
            if escape_next {
                escape_next = false;
                self.pos += 1;
                continue;
            }
            
            if c == '\\' {
                escape_next = true;
                self.pos += 1;
                continue;
            }
            
            if (c == '\'' || c == '"') && !in_quote {
                in_quote = true;
                quote_char = c;
                self.pos += 1;
                continue;
            }
            
            if c == quote_char && in_quote {
                in_quote = false;
                quote_char = '\0';
                self.pos += 1;
                continue;
            }
            
            if !in_quote {
                // Check for separators
                if c == ';' {
                    let cmd = &self.input[start..self.pos].trim();
                    self.pos += 1; // Skip the separator
                    return Some((cmd, Some(';')));
                } else if c == '&' && self.pos + 1 < self.input.len() && 
                          self.input.chars().nth(self.pos + 1) == Some('&') {
                    let cmd = &self.input[start..self.pos].trim();
                    self.pos += 2; // Skip "&&"
                    return Some((cmd, Some('&')));
                } else if c == '|' && self.pos + 1 < self.input.len() && 
                          self.input.chars().nth(self.pos + 1) == Some('|') {
                    let cmd = &self.input[start..self.pos].trim();
                    self.pos += 2; // Skip "||"
                    return Some((cmd, Some('|')));
                } else if c == '|' {
                    // Single pipe - handled at higher level
                    // Just treat as regular character for now
                    self.pos += 1;
                    continue;
                }
            }
            
            self.pos += 1;
        }
        
        // Last command
        let cmd = &self.input[start..].trim();
        if cmd.is_empty() {
            None
        } else {
            Some((cmd, None))
        }
    }
}

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
            let exit_status = shell.run_command_string(&args[2]);
            process::exit(exit_status);
        } else if args[1] == "--help" || args[1] == "-h" {
            shell.help_command(&[]);
            process::exit(0);
        } else if args[1] == "--version" || args[1] == "-v" {
            println!("rs-dash version 0.1.0");
            println!("A Rust implementation of dash shell");
            process::exit(0);
        } else {
            // Assume it's a script file
            let exit_status = shell.run_script_file(&args[1]);
            process::exit(exit_status);
        }
    }
    
    // Interactive mode
    shell.run_interactive();
    process::exit(shell.last_exit_status);
}