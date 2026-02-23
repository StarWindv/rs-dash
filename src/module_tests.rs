//! Module tests for rs-dash
//! Tests individual modules and their internal functions

#[cfg(test)]
mod module_tests {
    use crate::modules::arithmetic;
    use crate::modules::expansion;
    use crate::modules::parser;
    use crate::modules::utils;
    use crate::modules::shell::Shell;
    
    #[test]
    fn test_arithmetic_evaluation() {
        let mut shell = Shell::new();
        let mut evaluator = arithmetic::ArithmeticEvaluator::new();
        
        // Test basic arithmetic
        assert_eq!(evaluator.evaluate("1 + 1", &shell), Ok(2));
        assert_eq!(evaluator.evaluate("2 * 3", &shell), Ok(6));
        assert_eq!(evaluator.evaluate("10 / 2", &shell), Ok(5));
        assert_eq!(evaluator.evaluate("7 - 3", &shell), Ok(4));
        
        // Test precedence
        assert_eq!(evaluator.evaluate("2 + 3 * 4", &shell), Ok(14)); // 2 + 12 = 14
        assert_eq!(evaluator.evaluate("(2 + 3) * 4", &shell), Ok(20)); // 5 * 4 = 20
        
        // Test negative numbers
        assert_eq!(evaluator.evaluate("-5 + 10", &shell), Ok(5));
        assert_eq!(evaluator.evaluate("5 - -3", &shell), Ok(8));
        
        // Test division by zero
        assert!(evaluator.evaluate("10 / 0", &shell).is_err());
        
        // Test bitwise operations
        assert_eq!(evaluator.evaluate("5 & 3", &shell), Ok(1)); // 0101 & 0011 = 0001
        assert_eq!(evaluator.evaluate("5 | 3", &shell), Ok(7)); // 0101 | 0011 = 0111
        assert_eq!(evaluator.evaluate("5 ^ 3", &shell), Ok(6)); // 0101 ^ 0011 = 0110
        
        // Test shift operations
        assert_eq!(evaluator.evaluate("8 << 1", &shell), Ok(16));
        assert_eq!(evaluator.evaluate("8 >> 1", &shell), Ok(4));
        
        // Test comparison operators
        assert_eq!(evaluator.evaluate("5 == 5", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("5 != 3", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("5 < 10", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("5 > 3", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("5 <= 5", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("5 >= 3", &shell), Ok(1));
        
        // Test logical operators
        assert_eq!(evaluator.evaluate("1 && 1", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("1 && 0", &shell), Ok(0));
        assert_eq!(evaluator.evaluate("0 || 1", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("!0", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("!1", &shell), Ok(0));
        
        // Test ternary operator
        assert_eq!(evaluator.evaluate("1 ? 10 : 20", &shell), Ok(10));
        assert_eq!(evaluator.evaluate("0 ? 10 : 20", &shell), Ok(20));
    }
    
    #[test]
    fn test_variable_expansion() {
        let mut shell = Shell::new();
        
        // Set up some variables
        shell.env_vars.insert("VAR1".to_string(), "value1".to_string());
        shell.env_vars.insert("VAR2".to_string(), "value2".to_string());
        shell.shell_name = "rs-dash".to_string();
        shell.positional_params = vec!["arg1".to_string(), "arg2".to_string()];
        
        // Test simple variable expansion
        assert_eq!(expansion::expand_variables(&mut shell, "$VAR1"), "value1");
        assert_eq!(expansion::expand_variables(&mut shell, "prefix${VAR1}suffix"), "prefixvalue1suffix");
        
        // Test multiple variables
        assert_eq!(expansion::expand_variables(&mut shell, "$VAR1 $VAR2"), "value1 value2");
        
        // Test special variables
        assert_eq!(expansion::expand_variables(&mut shell, "$0"), "rs-dash");
        assert_eq!(expansion::expand_variables(&mut shell, "$1"), "arg1");
        assert_eq!(expansion::expand_variables(&mut shell, "$2"), "arg2");
        
        // Test exit status variable (initially 0)
        assert_eq!(expansion::expand_variables(&mut shell, "$?"), "0");
        
        // Test shell PID (should be a number)
        let pid_expansion = expansion::expand_variables(&mut shell, "$$");
        assert!(!pid_expansion.is_empty());
        
        // Test undefined variable (should expand to empty string)
        assert_eq!(expansion::expand_variables(&mut shell, "$UNDEFINED"), "");
        
        // Test variable with braces
        assert_eq!(expansion::expand_variables(&mut shell, "${VAR1}"), "value1");
        
        // Test mixed content
        assert_eq!(expansion::expand_variables(&mut shell, "The value is $VAR1 and $VAR2"), 
                   "The value is value1 and value2");
    }
    
    #[test]
    fn test_command_parsing() {
        // Test basic command parsing
        let (cmd, args) = parser::parse_command("echo hello world");
        assert_eq!(cmd, "echo");
        assert_eq!(args, vec!["hello", "world"]);
        
        // Test command with quotes
        let (cmd, args) = parser::parse_command("echo \"hello world\"");
        assert_eq!(cmd, "echo");
        assert_eq!(args, vec!["hello world"]);
        
        // Test command with single quotes
        let (cmd, args) = parser::parse_command("echo 'hello world'");
        assert_eq!(cmd, "echo");
        assert_eq!(args, vec!["hello world"]);
        
        // Test command with escaped spaces
        let (cmd, args) = parser::parse_command("echo hello\\ world");
        assert_eq!(cmd, "echo");
        assert_eq!(args, vec!["hello world"]);
        
        // Test empty command
        let (cmd, args) = parser::parse_command("");
        assert_eq!(cmd, "");
        assert_eq!(args.len(), 0);
        
        // Test command with only whitespace
        let (cmd, args) = parser::parse_command("   ");
        assert_eq!(cmd, "");
        assert_eq!(args.len(), 0);
        
        // Test command with variable assignments
        let (cmd, args) = parser::parse_command("VAR=value echo test");
        assert_eq!(cmd, "VAR=value");
        assert_eq!(args, vec!["echo", "test"]);
        
        // Test command with redirection
        let (cmd, args) = parser::parse_command("echo test > file.txt");
        assert_eq!(cmd, "echo");
        assert_eq!(args, vec!["test", ">", "file.txt"]);
    }
    
    #[test]
    fn test_utils_functions() {
        // Test is_valid_var_name
        assert!(utils::is_valid_var_name("VAR"));
        assert!(utils::is_valid_var_name("VAR1"));
        assert!(utils::is_valid_var_name("_VAR"));
        assert!(utils::is_valid_var_name("var_name"));
        
        assert!(!utils::is_valid_var_name("1VAR")); // Cannot start with digit
        assert!(!utils::is_valid_var_name("VAR-NAME")); // Cannot contain hyphen
        assert!(!utils::is_valid_var_name("VAR NAME")); // Cannot contain space
        assert!(!utils::is_valid_var_name("")); // Cannot be empty
        
        // Test remove_quotes
        assert_eq!(utils::remove_quotes("\"hello\""), "hello");
        assert_eq!(utils::remove_quotes("'hello'"), "hello");
        assert_eq!(utils::remove_quotes("hello"), "hello");
        assert_eq!(utils::remove_quotes("\"hello world\""), "hello world");
        assert_eq!(utils::remove_quotes("'hello world'"), "hello world");
        
        // Test nested quotes (should only remove outer quotes)
        assert_eq!(utils::remove_quotes("\"'hello'\""), "'hello'");
        assert_eq!(utils::remove_quotes("'\"hello\"'"), "\"hello\"");
        
        // Test unclosed quotes (should return as-is)
        assert_eq!(utils::remove_quotes("\"hello"), "\"hello");
        assert_eq!(utils::remove_quotes("hello\""), "hello\"");
        
        // Test escape sequences (simplified - actual implementation may vary)
        let result = utils::remove_quotes(r#""hello\nworld""#);
        assert!(result.contains("hello") && result.contains("world"));
    }
    
    #[test]
    fn test_shell_creation() {
        // Test that shell is created with correct initial state
        let shell = Shell::new();
        
        // Check that current directory is set
        assert!(!shell.current_dir.is_empty());
        assert!(!shell.physical_dir.is_empty());
        
        // Check initial exit status
        assert_eq!(shell.last_exit_status, 0);
        
        // Check that shell is not in interactive mode initially
        assert!(!shell.interactive);
        
        // Check that shell name is set
        assert!(!shell.shell_name.is_empty());
        
        // Check that environment variables are populated
        assert!(!shell.env_vars.is_empty());
        
        // Check that builtin registry is created
        assert!(shell.builtin_registry.has_builtin("echo"));
        assert!(shell.builtin_registry.has_builtin("cd"));
        assert!(shell.builtin_registry.has_builtin("pwd"));
        assert!(shell.builtin_registry.has_builtin("exit"));
        assert!(shell.builtin_registry.has_builtin("true"));
        assert!(shell.builtin_registry.has_builtin("false"));
        assert!(shell.builtin_registry.has_builtin("help"));
        
        // Check that function table is empty initially
        assert!(!shell.function_table.exists("nonexistent"));
    }
    
    #[test]
    fn test_positional_parameters() {
        let mut shell = Shell::new();
        
        // Set positional parameters
        shell.set_positional_params(vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()]);
        
        // Set shell name for testing
        shell.shell_name = "rs-dash".to_string();
        
        // Test getting parameters
        assert_eq!(shell.get_positional_param(0), Some("rs-dash")); // $0 is shell name
        assert_eq!(shell.get_positional_param(1), Some("arg1"));
        assert_eq!(shell.get_positional_param(2), Some("arg2"));
        assert_eq!(shell.get_positional_param(3), Some("arg3"));
        assert_eq!(shell.get_positional_param(4), None); // Out of bounds
        
        // Test count
        assert_eq!(shell.positional_param_count(), 3);
        
        // Test with empty parameters
        shell.set_positional_params(vec![]);
        assert_eq!(shell.positional_param_count(), 0);
        assert_eq!(shell.get_positional_param(1), None);
    }
    
    #[test]
    fn test_command_splitting() {
        let shell = Shell::new();
        
        // Test split by semicolon
        let commands = shell.split_by_separator("echo first; echo second", ';');
        assert_eq!(commands, vec!["echo first", "echo second"]);
        
        // Test with whitespace
        let commands = shell.split_by_separator("echo first ; echo second", ';');
        assert_eq!(commands, vec!["echo first", "echo second"]);
        
        // Test empty commands
        let commands = shell.split_by_separator("echo first;; echo second", ';');
        assert_eq!(commands, vec!["echo first", "echo second"]);
        
        // Test with quotes (semicolon inside quotes should not split)
        let commands = shell.split_by_separator("echo \"; inside quotes\"; echo after", ';');
        assert_eq!(commands.len(), 2);
        assert!(commands[0].contains("; inside quotes"));
        assert_eq!(commands[1], "echo after");
        
        // Test with single quotes
        let commands = shell.split_by_separator("echo '; inside quotes'; echo after", ';');
        assert_eq!(commands.len(), 2);
        assert!(commands[0].contains("; inside quotes"));
        assert_eq!(commands[1], "echo after");
        
        // Test with escaped semicolon
        let commands = shell.split_by_separator("echo escaped\\; semicolon; echo after", ';');
        assert_eq!(commands, vec!["echo escaped; semicolon", "echo after"]);
        
        // Test with parentheses (semicolon inside should not split)
        let commands = shell.split_by_separator("(echo inside; echo paren); echo after", ';');
        assert_eq!(commands.len(), 2);
        assert!(commands[0].contains("(echo inside; echo paren)"));
        assert_eq!(commands[1], "echo after");
    }
    
    #[test]
    fn test_logical_operator_splitting() {
        let shell = Shell::new();
        
        // Test split by &&
        let parts = shell.split_by_logical_operator("echo first && echo second", "&&");
        assert_eq!(parts, vec!["echo first", "echo second"]);
        
        // Test split by ||
        let parts = shell.split_by_logical_operator("echo first || echo second", "||");
        assert_eq!(parts, vec!["echo first", "echo second"]);
        
        // Test with quotes (operators inside quotes should not split)
        let parts = shell.split_by_logical_operator("echo \"&& inside quotes\" && echo after", "&&");
        assert_eq!(parts.len(), 2);
        assert!(parts[0].contains("&& inside quotes"));
        assert_eq!(parts[1], "echo after");
        
        // Test with escaped operators
        let parts = shell.split_by_logical_operator("echo escaped\\&\\& operator && echo after", "&&");
        assert_eq!(parts, vec!["echo escaped&& operator", "echo after"]);
        
        // Test with parentheses
        let parts = shell.split_by_logical_operator("(echo inside && echo paren) && echo after", "&&");
        assert_eq!(parts.len(), 2);
        assert!(parts[0].contains("(echo inside && echo paren)"));
        assert_eq!(parts[1], "echo after");
        
        // Test complex expression
        let parts = shell.split_by_logical_operator("echo first && echo second || echo third", "||");
        assert_eq!(parts, vec!["echo first && echo second", "echo third"]);
    }
}