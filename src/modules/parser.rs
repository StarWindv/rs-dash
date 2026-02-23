//! Command parsing and splitting

/// Parse a command into command name and arguments
pub fn parse_command(line: &str) -> (String, Vec<String>) {
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
                            // In double quotes, backslash before non-special character
                            // is kept as literal backslash followed by the character
                            current.push('\\');
                            current.push(c);
                        }
                    }
                }
                // In single quotes, backslash has no special meaning
                (_, '\'') => {
                    // In single quotes, backslash is always literal
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
                            // Other characters: backslash was consumed to escape it
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
            // Start of quote - don't add quote character to output
            in_quote = true;
            quote_char = c;
            continue; // Skip the quote character
        } else if c == quote_char && in_quote {
            // End of quote - don't add quote character to output
            in_quote = false;
            quote_char = '\0';
            continue; // Skip the quote character
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

/// Iterator for splitting commands by separators
pub struct CommandSplitter<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> CommandSplitter<'a> {
    pub fn new(input: &'a str) -> Self {
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
        let mut paren_depth = 0;
        let mut brace_depth = 0;
        
        let chars: Vec<char> = self.input.chars().collect();
        
        while self.pos < chars.len() {
            let c = chars[self.pos];
            
            if escape_next {
                // Handle escape sequences based on context
                match (c, quote_char) {
                    // In double quotes, only certain characters can be escaped
                    (_, '"') => {
                        match c {
                            '\\' | '"' | '$' | '`' | '\n' => {
                                // These are properly escaped in double quotes
                            }
                            _ => {
                                // Other characters: backslash is kept
                            }
                        }
                    }
                    // In single quotes, backslash has no special meaning
                    (_, '\'') => {
                        // Backslash is kept
                    }
                    // Not in quotes
                    (_, _) => {
                        match c {
                            '\n' => {
                                // Line continuation - skip the newline
                                self.pos += 1;
                                escape_next = false;
                                continue;
                            }
                            '\\' | '\'' | '"' | '$' | '`' | ' ' | '\t' | '|' | '&' | ';' | '<' | '>' | '(' | ')' => {
                                // These characters need escaping outside quotes
                            }
                            _ => {
                                // Other characters: backslash was consumed to escape it
                            }
                        }
                    }
                }
                escape_next = false;
                self.pos += 1;
                continue;
            }
            
            if c == '\\' {
                // Check if next character is newline for line continuation
                if self.pos + 1 < chars.len() && chars[self.pos + 1] == '\n' {
                    // Line continuation - skip both backslash and newline
                    self.pos += 2;
                    continue;
                }
                escape_next = true;
                self.pos += 1;
                continue;
            }
            
            if (c == '\'' || c == '"') && !in_quote && paren_depth == 0 && brace_depth == 0 {
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
                // Track parentheses for command substitution and subshells
                if c == '(' {
                    paren_depth += 1;
                    self.pos += 1;
                    continue;
                } else if c == ')' && paren_depth > 0 {
                    paren_depth -= 1;
                    self.pos += 1;
                    continue;
                }
                
                // Track braces for parameter expansion
                if c == '{' {
                    brace_depth += 1;
                    self.pos += 1;
                    continue;
                } else if c == '}' && brace_depth > 0 {
                    brace_depth -= 1;
                    self.pos += 1;
                    continue;
                }
                
                // Only check for separators when not inside parentheses or braces
                if paren_depth == 0 && brace_depth == 0 {
                    // Check for command separators
                    if c == ';' {
                        let cmd = &self.input[start..self.pos].trim();
                        self.pos += 1; // Skip the separator
                        return Some((cmd, Some(';')));
                    }
                    
                    // Check for && and || only at word boundaries
                    // We need to check if we're at a word boundary
                    if c == '&' && self.pos + 1 < chars.len() && 
                       chars[self.pos + 1] == '&' {
                        // Check if we're at a word boundary (preceded by whitespace or start of string)
                        let is_word_boundary = if start == self.pos {
                            true // At start of command
                        } else {
                            let prev_char = chars[self.pos - 1];
                            prev_char.is_whitespace()
                        };
                        
                        if is_word_boundary {
                            let cmd = &self.input[start..self.pos].trim();
                            self.pos += 2; // Skip "&&"
                            return Some((cmd, Some('&')));
                        }
                    }
                    
                    if c == '|' && self.pos + 1 < chars.len() && 
                       chars[self.pos + 1] == '|' {
                        // Check if we're at a word boundary
                        let is_word_boundary = if start == self.pos {
                            true // At start of command
                        } else {
                            let prev_char = chars[self.pos - 1];
                            prev_char.is_whitespace()
                        };
                        
                        if is_word_boundary {
                            let cmd = &self.input[start..self.pos].trim();
                            self.pos += 2; // Skip "||"
                            return Some((cmd, Some('|')));
                        }
                    }
                    
                    // Single pipe - handled at higher level
                    // Just treat as regular character for now
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

/// Split command line into individual commands (for ; && ||)
pub fn split_commands(line: &str) -> CommandSplitter<'_> {
    CommandSplitter::new(line)
}