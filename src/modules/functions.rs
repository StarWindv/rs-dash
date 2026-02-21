//! Function support for rs-dash
//! Supports function definition, local variables, and return builtin

use std::collections::HashMap;
use crate::modules::shell::Shell;

/// Function definition
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name
    pub name: String,
    /// Function body (as command string)
    pub body: String,
    /// Local variables (name -> value)
    pub local_vars: HashMap<String, String>,
}

/// Function table
#[derive(Debug, Clone)]
pub struct FunctionTable {
    /// Map from function name to function definition
    functions: HashMap<String, Function>,
}

impl FunctionTable {
    /// Create a new empty function table
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
    
    /// Define a new function
    pub fn define(&mut self, name: String, body: String) {
        let func = Function {
            name: name.clone(),
            body,
            local_vars: HashMap::new(),
        };
        self.functions.insert(name, func);
    }
    
    /// Check if a function exists
    pub fn exists(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
    
    /// Get a function by name
    pub fn get(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }
    
    /// Get a mutable function by name
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.functions.get_mut(name)
    }
    
    /// Remove a function
    pub fn remove(&mut self, name: &str) -> Option<Function> {
        self.functions.remove(name)
    }
    
    /// List all function names
    pub fn list_names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
}

/// Parse a function definition
pub fn parse_function_definition(line: &str) -> Result<(String, String), String> {
    // Function definition format: name() compound-command
    // or: name() { compound-command; }
    
    let trimmed = line.trim();
    
    // Find the function name
    let paren_pos = trimmed.find('(').ok_or("Missing '(' in function definition")?;
    let name = trimmed[..paren_pos].trim().to_string();
    
    if name.is_empty() {
        return Err("Empty function name".to_string());
    }
    
    // Check if name is valid (starts with letter)
    if !name.chars().next().unwrap().is_alphabetic() {
        return Err("Function name must start with a letter".to_string());
    }
    
    // Check for closing paren
    let after_paren = &trimmed[paren_pos..];
    if !after_paren.starts_with("()") {
        return Err("Expected '()' after function name".to_string());
    }
    
    // Get the body (after "()")
    let body_start = paren_pos + 2; // Skip "()"
    let body = trimmed[body_start..].trim();
    
    if body.is_empty() {
        return Err("Empty function body".to_string());
    }
    
    // If body starts with '{', find matching '}'
    let body = if body.starts_with('{') {
        let mut brace_count = 1;
        let mut end_pos = 1;
        
        for (i, c) in body.chars().enumerate().skip(1) {
            if c == '{' {
                brace_count += 1;
            } else if c == '}' {
                brace_count -= 1;
                if brace_count == 0 {
                    end_pos = i;
                    break;
                }
            }
        }
        
        if brace_count != 0 {
            return Err("Unmatched '{' in function body".to_string());
        }
        
        body[1..end_pos].trim().to_string()
    } else {
        body.to_string()
    };
    
    Ok((name, body))
}

/// Execute a function
pub fn execute_function(shell: &mut Shell, func_name: &str) -> i32 {
    // Get the function body first
    let body = if let Some(func) = shell.function_table.get(func_name) {
        func.body.clone()
    } else {
        eprintln!("{}: function not found", func_name);
        return 127;
    };
    
    // Save current environment variables
    let saved_env_vars = shell.env_vars.clone();
    
    // Execute the function body
    let status = shell.execute_command_line(&body);
    
    // Restore environment variables (removing local variables)
    shell.env_vars = saved_env_vars;
    
    status
}

/// Check if a line is a function definition
pub fn is_function_definition(line: &str) -> bool {
    let trimmed = line.trim();
    
    // Check for pattern: word() ...
    if let Some(paren_pos) = trimmed.find('(') {
        if paren_pos > 0 && trimmed[paren_pos..].starts_with("()") {
            let name_part = &trimmed[..paren_pos];
            // Check if name is valid (not empty, starts with letter)
            return !name_part.is_empty() && name_part.chars().next().unwrap().is_alphabetic();
        }
    }
    
    false
}