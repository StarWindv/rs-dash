use std::process::Command;

fn test_escape_handling() {
    println!("=== Testing Escape Handling in rs-dash ===");
    
    // Test cases from the user's example
    let test_cases = vec![
        // Windows paths with backslashes
        (r#"echo "C:\ProgramData\chocolatey\bin\tree.exe""#, 
         vec!["echo", r#""C:\ProgramData\chocolatey\bin\tree.exe""#]),
        (r#"echo C:\ProgramData\chocolatey\bin\tree.exe"#, 
         vec!["echo", r#"C:\ProgramData\chocolatey\bin\tree.exe"#]),
        (r#"echo "C:\\""#, 
         vec!["echo", r#""C:\\""#]),
        (r#"echo C:\\"#, 
         vec!["echo", r#"C:\\"#]),
        
        // Test escape sequences
        (r#"echo "test\ttab""#, 
         vec!["echo", r#""test\ttab""#]),
        (r#"echo test\ttab"#, 
         vec!["echo", "test\ttab"]),
        (r#"echo "test\nnewline""#, 
         vec!["echo", r#""test\nnewline""#]),
        (r#"echo test\nnewline"#, 
         vec!["echo", "test\nnewline"]),
        
        // Test line continuation
        (r#"echo hello \
world"#, 
         vec!["echo", "hello", "world"]),
        
        // Test quotes and escapes
        (r#"echo "test\"quote""#, 
         vec!["echo", r#""test\"quote""#]),
        (r#"echo 'test\'quote'"#, 
         vec!["echo", r#"'test\'quote'"#]),
        (r#"echo test\'quote"#, 
         vec!["echo", "test'quote"]),
        
        // Test special characters
        (r#"echo \$PATH"#, 
         vec!["echo", "$PATH"]),
        (r#"echo \`command\`"#, 
         vec!["echo", "`command`"]),
    ];
    
    for (i, (input, expected)) in test_cases.iter().enumerate() {
        println!("\nTest {}: {}", i + 1, input);
        
        // Parse using our parse_command function
        let (cmd, args) = parse_command(input);
        let mut result = vec![cmd];
        result.extend(args);
        
        println!("Result:   {:?}", result);
        println!("Expected: {:?}", expected);
        
        if &result == expected {
            println!("✓ PASS");
        } else {
            println!("✗ FAIL");
            
            // Show character-by-character comparison
            println!("Detailed comparison:");
            for (j, (res, exp)) in result.iter().zip(expected.iter()).enumerate() {
                if res != exp {
                    println!("  Arg {}: '{}' != '{}'", j, res, exp);
                    println!("    Result chars:   {:?}", res.chars().collect::<Vec<_>>());
                    println!("    Expected chars: {:?}", exp.chars().collect::<Vec<_>>());
                }
            }
        }
    }
}

// Copy of the updated parse_command function
fn parse_command(line: &str) -> (String, Vec<String>) {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = '\0';
    let mut escape_next = false;
    let mut paren_depth = 0;
    
    let mut chars = line.chars().peekable();
    
    while let Some(c) = chars.next() {
        if escape_next {
            // Handle escape sequences based on context
            match (c, quote_char) {
                // In double quotes, only certain characters can be escaped
                (_, '"') => {
                    match c {
                        '\\' | '"' | '$' | '`' | '\n' => {
                            // These are properly escaped in double quotes
                            current.push(c);
                        }
                        _ => {
                            // Other characters: keep both backslash and character
                            current.push('\\');
                            current.push(c);
                        }
                    }
                }
                // In single quotes, backslash has no special meaning
                // (but we shouldn't reach here since backslash isn't special in single quotes)
                (_, '\'') => {
                    current.push('\\');
                    current.push(c);
                }
                // Not in quotes
                (_, _) => {
                    match c {
                        '\n' => {
                            // Line continuation - skip the newline
                            // Don't add anything to current
                        }
                        '\\' | '\'' | '"' | '$' | '`' | ' ' | '\t' | '|' | '&' | ';' | '<' | '>' | '(' | ')' => {
                            // These characters need escaping outside quotes
                            current.push(c);
                        }
                        _ => {
                            // Other characters: just add the character
                            // The backslash was consumed to escape it
                            current.push(c);
                        }
                    }
                }
            }
            escape_next = false;
        } else if c == '\\' {
            // Check if next character is newline for line continuation
            if let Some(&next_c) = chars.peek() {
                if next_c == '\n' {
                    // Line continuation - skip both backslash and newline
                    chars.next(); // Skip the newline
                    continue;
                }
            }
            escape_next = true;
        } else if (c == '\'' || c == '"') && !in_quote && paren_depth == 0 {
            in_quote = true;
            quote_char = c;
            current.push(c);
        } else if c == quote_char && in_quote {
            in_quote = false;
            quote_char = '\0';
            current.push(c);
        } else if c == '$' {
            // Check for command substitution start
            current.push(c);
        } else if c == '(' && !in_quote && current.ends_with('$') {
            // Start of command substitution
            paren_depth += 1;
            current.push(c);
        } else if c == '(' && !in_quote {
            // Regular parenthesis (not part of command substitution)
            current.push(c);
        } else if c == ')' && !in_quote && paren_depth > 0 {
            // End of command substitution parenthesis
            paren_depth -= 1;
            current.push(c);
        } else if c == ')' && !in_quote {
            // Regular parenthesis
            current.push(c);
        } else if c.is_whitespace() && !in_quote && paren_depth == 0 {
            if !current.is_empty() {
                parts.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
    }
    
    // Handle trailing backslash
    if escape_next {
        current.push('\\');
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

fn main() {
    test_escape_handling();
}