//! Unit tests for control structures and functions

#[cfg(test)]
mod tests {
    use crate::modules::shell::Shell;
    use crate::modules::control;
    use crate::modules::functions;
    
    #[test]
    fn test_control_structure_detection() {
        assert!(control::is_control_structure("if true; then echo 'test'; fi"));
        assert!(control::is_control_structure("for i in 1 2 3; do echo $i; done"));
        assert!(control::is_control_structure("while true; do echo 'loop'; done"));
        assert!(control::is_control_structure("until false; do echo 'loop'; done"));
        assert!(control::is_control_structure("case $var in pattern) echo 'match';; esac"));
        
        assert!(!control::is_control_structure("echo 'hello'"));
        assert!(!control::is_control_structure("ls -la"));
    }
    
    #[test]
    fn test_function_detection() {
        assert!(functions::is_function_definition("myfunc() { echo 'hello'; }"));
        assert!(functions::is_function_definition("greet() echo 'hi'"));
        assert!(functions::is_function_definition("test() { return 0; }"));
        
        assert!(!functions::is_function_definition("echo 'hello'"));
        assert!(!functions::is_function_definition("() { echo 'missing name'; }"));
        assert!(!functions::is_function_definition("123func() { echo 'invalid name'; }"));
    }
    
    #[test]
    fn test_function_parsing() {
        // Test simple function
        let result = functions::parse_function_definition("myfunc() { echo 'hello'; }");
        assert!(result.is_ok());
        let (name, body) = result.unwrap();
        assert_eq!(name, "myfunc");
        assert_eq!(body, "echo 'hello';");
        
        // Test function without braces
        let result = functions::parse_function_definition("simple() echo 'hi'");
        assert!(result.is_ok());
        let (name, body) = result.unwrap();
        assert_eq!(name, "simple");
        assert_eq!(body, "echo 'hi'");
        
        // Test nested braces
        let result = functions::parse_function_definition("nested() { if true; then echo 'nested'; fi; }");
        assert!(result.is_ok());
        let (name, body) = result.unwrap();
        assert_eq!(name, "nested");
        assert_eq!(body, "if true; then echo 'nested'; fi;");
        
        // Test invalid function
        let result = functions::parse_function_definition("missingparen { echo 'error'; }");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_if_statement_execution() {
        let mut shell = Shell::new();
        
        // Test simple if with echo (echo is a builtin)
        let status = shell.execute_command_line("if echo test; then echo 'true'; fi");
        assert_eq!(status, 0);
        
        // Test if with false builtin
        let status = shell.execute_command_line("if false; then echo 'false'; fi");
        assert_eq!(status, 0);
        
        // Test if-else with true/false
        let status = shell.execute_command_line("if true; then echo 'if'; else echo 'else'; fi");
        assert_eq!(status, 0);
    }
    
    #[test]
    fn test_for_loop_execution() {
        let mut shell = Shell::new();
        
        // Test basic for loop
        let status = shell.execute_command_line("for i in 1 2 3; do echo $i; done");
        assert_eq!(status, 0);
        
        // Test for loop with variable assignment in body
        let status = shell.execute_command_line("for i in a b c; do var=$i; done");
        assert_eq!(status, 0);
    }
    
    #[test]
    fn test_function_execution() {
        let mut shell = Shell::new();
        
        // Define a function
        let status = shell.execute_command_line("myfunc() { echo 'hello'; }");
        assert_eq!(status, 0);
        
        // Call the function
        let status = shell.execute_command_line("myfunc");
        assert_eq!(status, 0);
        
        // Define function with arguments
        let status = shell.execute_command_line("greet() { echo \"Hello, $1\"; }");
        assert_eq!(status, 0);
        
        // Call function with argument
        let status = shell.execute_command_line("greet World");
        assert_eq!(status, 0);
        
        // Test multiple functions
        let status = shell.execute_command_line("f1() { echo 'f1'; }; f2() { echo 'f2'; }; f1; f2");
        assert_eq!(status, 0);
    }
    
    #[test]
    fn test_return_builtin_execution() {
        let mut shell = Shell::new();
        
        // Define function with return
        let status = shell.execute_command_line("myfunc() { return 42; }");
        assert_eq!(status, 0);
        
        // Call function and check exit status
        let status = shell.execute_command_line("myfunc");
        assert_eq!(status, 0); // Function execution status
        
        // Note: $? would be 42, but we need to check last_exit_status
        // For now, just verify the function doesn't crash
    }
    
    #[test]
    fn test_variable_assignment_in_control_structures() {
        let mut shell = Shell::new();
        
        // Test variable assignment in for loop
        let status = shell.execute_command_line("for i in 1 2 3; do counter=$i; done");
        assert_eq!(status, 0);
        
        // Test variable expansion in if condition
        let status = shell.execute_command_line("condition=true; if $condition; then echo 'works'; fi");
        assert_eq!(status, 0);
    }
    
    #[test]
    fn test_nested_control_structures() {
        let mut shell = Shell::new();
        
        // Test if inside for loop
        let status = shell.execute_command_line("for i in 1 2 3; do if true; then echo $i; fi; done");
        assert_eq!(status, 0);
        
        // Test for loop inside function
        let status = shell.execute_command_line("count() { for i in 1 2 3; do echo $i; done; }; count");
        assert_eq!(status, 0);
    }
    
    #[test]
    fn test_command_substitution_in_control_structures() {
        let mut shell = Shell::new();
        
        // Test command substitution in if condition
        let status = shell.execute_command_line("if echo true; then echo 'success'; fi");
        assert_eq!(status, 0);
        
        // Test command substitution in for loop
        let status = shell.execute_command_line("for i in $(echo 1 2 3); do echo $i; done");
        assert_eq!(status, 0);
    }
}