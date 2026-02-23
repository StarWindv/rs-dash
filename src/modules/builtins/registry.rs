//! Builtin command registry

use std::collections::HashMap;
use crate::modules::shell::Shell;
use super::Builtin;

/// Registry for builtin commands
pub struct BuiltinRegistry {
    builtins: HashMap<String, Box<dyn Builtin>>,
}

impl BuiltinRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            builtins: HashMap::new(),
        }
    }
    
    /// Register a builtin command
    pub fn register(&mut self, builtin: Box<dyn Builtin>) {
        self.builtins.insert(builtin.name().to_string(), builtin);
    }
    
    /// Check if a command is a builtin
    pub fn has_builtin(&self, cmd: &str) -> bool {
        self.builtins.contains_key(cmd)
    }
    
    /// Get a builtin command
    pub fn get_builtin(&self, cmd: &str) -> Option<&dyn Builtin> {
        self.builtins.get(cmd).map(|b| b.as_ref())
    }
    
    /// Execute a builtin command (helper method to avoid borrowing issues)
    pub fn execute_builtin(&self, shell: &mut Shell, cmd: &str, args: &[String]) -> i32 {
        // Get the builtin before we need to mutably borrow shell
        let builtin = match self.get_builtin(cmd) {
            Some(b) => b,
            None => return 127,
        };
        
        // Now execute with the builtin reference
        builtin.execute(shell, args)
    }
    
    /// Execute a builtin command in a pipeline context
    pub fn execute_builtin_in_pipeline(&self, shell: &mut Shell, cmd: &str, args: &[String], is_last: bool) -> i32 {
        // Get the builtin before we need to mutably borrow shell
        let builtin = match self.get_builtin(cmd) {
            Some(b) => b,
            None => return 127,
        };
        
        // Now execute with the builtin reference
        builtin.execute_in_pipeline(shell, args, is_last)
    }
    
    /// Execute a builtin command and capture its output
    pub fn execute_builtin_and_capture(&self, shell: &mut Shell, cmd: &str, args: &[String]) -> (i32, String) {
        // Get the builtin before we need to mutably borrow shell
        let builtin = match self.get_builtin(cmd) {
            Some(b) => b,
            None => return (127, String::new()),
        };
        
        // Now execute with the builtin reference
        builtin.execute_and_capture(shell, args)
    }
}

impl Default for BuiltinRegistry {
    fn default() -> Self {
        Self::new()
    }
}