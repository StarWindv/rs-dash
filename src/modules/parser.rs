//! Command parsing and splitting

/// Parse a command into command name and arguments
pub fn parse_command(line: &str) -> (String, Vec<String>) {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = '\0';
    let mut escape_next = false;
    let mut paren_depth = 0;
    
    for c in line.chars() {
        if escape_next {
            current.push(c);
            escape_next = false;
        } else if c == '\\' {
            escape_next = true;
        } else if (c == '\'' || c == '"') && !in_quote && paren_depth == 0 {
            in_quote = true;
            quote_char = c;
        } else if c == quote_char && in_quote {
            in_quote = false;
            quote_char = '\0';
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
        
        while self.pos < self.input.len() {
            let c = match self.input.chars().nth(self.pos) {
                Some(ch) => ch,
                None => break,
            };
            
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

/// Split command line into individual commands (for ; && ||)
pub fn split_commands(line: &str) -> CommandSplitter<'_> {
    CommandSplitter::new(line)
}