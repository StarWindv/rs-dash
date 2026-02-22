//! Shell core structure and main execution logic

use std::env;
use std::io::{self, Write};
use std::collections::HashMap;
use std::rc::Rc;

use crate::modules::builtins;
use crate::modules::control;
use crate::modules::expansion;
use crate::modules::functions;
use crate::modules::parser;
use crate::modules::pipeline;
use crate::modules::process_substitution;
use crate::modules::redirection;
use crate::modules::subshell;

/// Compile-time constants
const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

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
    /// Builtin command registry
    pub builtin_registry: Rc<builtins::BuiltinRegistry>,
    /// Function table
    pub function_table: functions::FunctionTable,
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
            .unwrap_or_else(|| NAME.to_string());
        
        // Create builtin registry
        let builtin_registry = builtins::create_registry();
        
        Self {
            current_dir,
            last_exit_status: 0,
            interactive: false,
            env_vars,
            positional_params: Vec::new(),
            shell_name,
            options: String::new(),  // Empty options for now
            builtin_registry,
            function_table: functions::FunctionTable::new(),
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
        // Handle process substitution first
        let processed_line = match process_substitution::handle_process_substitution(self, line) {
            Ok(line) => line,
            Err(status) => {
                self.last_exit_status = status;
                return status;
            }
        };
        
        // Check if it's a control structure (needs to be handled as a whole)
        if control::is_control_structure(&processed_line) {
            match control::parse_control_structure(&processed_line) {
                Ok(control_struct) => {
                    let status = control::ControlExecutor::execute(self, &control_struct);
                    self.last_exit_status = status;
                    return status;
                }
                Err(e) => {
                    eprintln!("Error parsing control structure: {}", e);
                    self.last_exit_status = 1;
                    return 1;
                }
            }
        }
        
        // Check for pipelines
        if pipeline::has_pipeline(&processed_line) {
            // Handle pipeline
            return pipeline::execute_pipeline(self, &processed_line);
        }
        
        // Parse and execute commands with proper operator precedence
        // ; has lowest precedence, then && and || have same precedence
        self.execute_commands_with_precedence(&processed_line)
    }
    
    /// Execute commands with proper operator precedence
    fn execute_commands_with_precedence(&mut self, line: &str) -> i32 {
        // First split by ; (lowest precedence)
        let commands = self.split_by_separator(line, ';');
        // println!("DEBUG: split commands: {:?}", commands); // DEBUG
        let mut last_status = 0;
        
        for cmd in commands {
            if cmd.trim().is_empty() {
                continue;
            }
            
            // Now handle && and || within this command
            last_status = self.execute_logical_expression(cmd.trim());
        }
        
        last_status
    }
    
    /// Split a line by a separator, respecting quotes and parentheses
    fn split_by_separator(&self, line: &str, separator: char) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut in_quote = false;
        let mut quote_char = '\0';
        let mut escape_next = false;
        let mut paren_depth = 0;
        let mut brace_depth = 0;
        
        for c in line.chars() {
            if escape_next {
                current.push(c);
                escape_next = false;
                continue;
            }
            
            if c == '\\' {
                escape_next = true;
                current.push(c);
                continue;
            }
            
            if (c == '\'' || c == '"') && !in_quote && paren_depth == 0 && brace_depth == 0 {
                in_quote = true;
                quote_char = c;
                current.push(c);
                continue;
            }
            
            if c == quote_char && in_quote {
                in_quote = false;
                quote_char = '\0';
                current.push(c);
                continue;
            }
            
            if !in_quote {
                // Track parentheses
                if c == '(' {
                    paren_depth += 1;
                } else if c == ')' {
                    paren_depth -= 1;
                }
                
                // Track braces
                if c == '{' {
                    brace_depth += 1;
                } else if c == '}' {
                    brace_depth -= 1;
                }
                
                // Check for separator
                if c == separator && paren_depth == 0 && brace_depth == 0 {
                    result.push(current.trim().to_string());
                    current.clear();
                    continue;
                }
            }
            
            current.push(c);
        }
        
        if !current.trim().is_empty() {
            result.push(current.trim().to_string());
        }
        
        result
    }
    
    /// Execute a logical expression with && and ||
    fn execute_logical_expression(&mut self, expr: &str) -> i32 {
        // Split by && first (higher precedence than || in some shells, but same in dash)
        // Actually, && and || have same precedence and are left-associative in dash
        // So we need to parse them properly
        
        // Simple approach: split by || first, then by && within each part
        let or_parts = self.split_by_logical_operator(expr, "||");
        let mut last_status = 0;
        
        for or_part in or_parts {
            if or_part.trim().is_empty() {
                continue;
            }
            
            // Split by &&
            let and_parts = self.split_by_logical_operator(&or_part, "&&");
            let mut and_status = 0;
            let mut all_succeeded = true;
            
            for and_part in and_parts {
                if and_part.trim().is_empty() {
                    continue;
                }
                
                // Execute this part
                and_status = self.execute_single_command(and_part.trim());
                
                // If this is an && chain and any part fails, stop
                if and_status != 0 {
                    all_succeeded = false;
                    break;
                }
            }
            
            last_status = if all_succeeded { 0 } else { and_status };
            
            // If this OR part succeeded (all its AND parts succeeded), stop
            if last_status == 0 {
                break;
            }
        }
        
        // Set last exit status
        self.last_exit_status = last_status;
        last_status
    }
    
    /// Split by logical operator, respecting quotes and parentheses
    fn split_by_logical_operator(&self, expr: &str, operator: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut in_quote = false;
        let mut quote_char = '\0';
        let mut escape_next = false;
        let mut paren_depth = 0;
        let mut brace_depth = 0;
        let mut i = 0;
        let chars: Vec<char> = expr.chars().collect();
        let op_chars: Vec<char> = operator.chars().collect();
        
        while i < chars.len() {
            let c = chars[i];
            
            if escape_next {
                current.push(c);
                escape_next = false;
                i += 1;
                continue;
            }
            
            if c == '\\' {
                escape_next = true;
                current.push(c);
                i += 1;
                continue;
            }
            
            if (c == '\'' || c == '"') && !in_quote && paren_depth == 0 && brace_depth == 0 {
                in_quote = true;
                quote_char = c;
                current.push(c);
                i += 1;
                continue;
            }
            
            if c == quote_char && in_quote {
                in_quote = false;
                quote_char = '\0';
                current.push(c);
                i += 1;
                continue;
            }
            
            if !in_quote {
                // Track parentheses
                if c == '(' {
                    paren_depth += 1;
                } else if c == ')' {
                    paren_depth -= 1;
                }
                
                // Track braces
                if c == '{' {
                    brace_depth += 1;
                } else if c == '}' {
                    brace_depth -= 1;
                }
                
                // Check for operator
                if paren_depth == 0 && brace_depth == 0 && i + op_chars.len() <= chars.len() {
                    let slice: String = chars[i..i + op_chars.len()].iter().collect();
                    if slice == operator {
                        // Check if it's really the operator (not part of a word)
                        let before_ok = if i == 0 {
                            true
                        } else {
                            let before = chars[i - 1];
                            before.is_whitespace()
                        };
                        
                        let after_ok = if i + op_chars.len() >= chars.len() {
                            true
                        } else {
                            let after = chars[i + op_chars.len()];
                            after.is_whitespace() || after == ';'
                        };
                        
                        if before_ok && after_ok {
                            result.push(current.trim().to_string());
                            current.clear();
                            i += op_chars.len();
                            continue;
                        }
                    }
                }
            }
            
            current.push(c);
            i += 1;
        }
        
        if !current.trim().is_empty() {
            result.push(current.trim().to_string());
        }
        
        result
    }
    
    /// Execute a single command (no separators, no pipes)
    pub fn execute_single_command(&mut self, cmd_str: &str) -> i32 {
        // Check for subshell first
        if subshell::has_subshell(cmd_str) {
            return subshell::execute_subshell(self, cmd_str);
        }
        
        // Check for function definitions first
        if functions::is_function_definition(cmd_str) {
            match functions::parse_function_definition(cmd_str) {
                Ok((name, body)) => {
                    self.function_table.define(name, body);
                    return 0;
                }
                Err(e) => {
                    eprintln!("Error parsing function definition: {}", e);
                    return 1;
                }
            }
        }
        
        // Check if it's a function call
        let (cmd, args) = parser::parse_command(cmd_str);
        if self.function_table.exists(&cmd) {
            // Save current positional parameters
            let saved_positional_params = self.positional_params.clone();
            
            // Set new positional parameters from function arguments
            self.positional_params = args;
            
            // Execute the function
            let status = functions::execute_function(self, &cmd);
            
            // Restore positional parameters
            self.positional_params = saved_positional_params;
            
            self.last_exit_status = status;
            return status;
        }
        
        // Check for variable assignments at the beginning
        // In dash, variable assignments can appear before a command
        // Example: VAR1=value1 VAR2=value2 command args
        let mut var_assignments = Vec::new();
        let mut remaining_cmd = cmd_str;
        
        // Parse variable assignments
        loop {
            let trimmed = remaining_cmd.trim_start();
            if trimmed.is_empty() {
                break;
            }
            
            // Find the first '=' that's not inside quotes
            let mut equals_pos = None;
            let mut in_quote = false;
            let mut quote_char = '\0';
            let mut escape_next = false;
            
            for (i, c) in trimmed.chars().enumerate() {
                if escape_next {
                    escape_next = false;
                    continue;
                }
                
                match c {
                    '\\' => {
                        escape_next = true;
                    }
                    '\'' | '"' => {
                        if !in_quote {
                            in_quote = true;
                            quote_char = c;
                        } else if c == quote_char {
                            in_quote = false;
                            quote_char = '\0';
                        }
                    }
                    '=' => {
                        if !in_quote {
                            equals_pos = Some(i);
                            break;
                        }
                    }
                    ' ' | '\t' | '\n' => {
                        if !in_quote && equals_pos.is_none() {
                            // Found whitespace before '=', not a variable assignment
                            break;
                        }
                    }
                    _ => {}
                }
            }
            
            if let Some(pos) = equals_pos {
                // Check if the part before '=' is a valid variable name
                let before_equals = &trimmed[..pos];
                if !before_equals.is_empty() && 
                   before_equals.chars().next().unwrap().is_alphabetic() &&
                   before_equals.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    
                    // It's a variable assignment
                    // Find the end of the value
                    let after_equals = &trimmed[pos + 1..];
                    let mut value_end = 0;
                    let mut in_quote = false;
                    let mut quote_char = '\0';
                    let mut escape_next = false;
                    
                    for (i, c) in after_equals.chars().enumerate() {
                        if escape_next {
                            escape_next = false;
                            continue;
                        }
                        
                        match c {
                            '\\' => {
                                escape_next = true;
                            }
                            '\'' | '"' => {
                                if !in_quote {
                                    in_quote = true;
                                    quote_char = c;
                                } else if c == quote_char {
                                    in_quote = false;
                                    quote_char = '\0';
                                }
                            }
                            ' ' | '\t' | '\n' => {
                                if !in_quote {
                                    value_end = i;
                                    break;
                                }
                            }
                            _ => {}
                        }
                        
                        value_end = i + 1;
                    }
                    
                    let var_name = before_equals.to_string();
                    let var_value = if value_end > 0 {
                        &after_equals[..value_end]
                    } else {
                        after_equals
                    }.to_string();
                    
                    var_assignments.push((var_name, var_value));
                    
                    // Move to next part
                    let next_start = pos + 1 + value_end;
                    if next_start >= trimmed.len() {
                        remaining_cmd = "";
                        break;
                    }
                    remaining_cmd = &trimmed[next_start..].trim_start();
                } else {
                    // Not a valid variable assignment, stop parsing
                    break;
                }
            } else {
                // No '=' found, stop parsing variable assignments
                break;
            }
        }
        
        // Apply variable assignments
        for (var_name, var_value) in var_assignments {
            self.env_vars.insert(var_name, var_value);
        }
        
        // If there are no more commands after variable assignments, return success
        if remaining_cmd.trim().is_empty() {
            self.last_exit_status = 0;
            return 0;
        }
        
        // Parse the remaining command
        let (cmd, args) = parser::parse_command(remaining_cmd);
        if cmd.is_empty() {
            self.last_exit_status = 0;
            return 0;
        }
        
        // Expand variables in arguments
        let args = args.iter()
            .map(|arg| expansion::expand_variables(self, arg))
            .collect::<Vec<String>>();
        
        let status = if self.builtin_registry.has_builtin(&cmd) {
            // Clone the Rc to avoid borrowing issues
            let registry = Rc::clone(&self.builtin_registry);
            registry.execute_builtin(self, &cmd, &args)
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
        
        if self.builtin_registry.has_builtin(&cmd) {
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
                    let registry = Rc::clone(&self.builtin_registry);
                    let _ = registry.execute_builtin(self, &cmd, &args);
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
        
        println!("{} v{}", NAME, VERSION);
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

