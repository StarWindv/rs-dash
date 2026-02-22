//! Utility functions for string manipulation

/// Remove quotes from a string if it's quoted
pub fn remove_quotes(s: &str) -> String {
    let mut chars = s.chars();
    let first = chars.next();
    let last = chars.last();
    
    match (first, last) {
        (Some('"'), Some('"')) | (Some('\''), Some('\'')) => {
            // Remove first and last character
            s[1..s.len()-1].to_string()
        }
        _ => s.to_string()
    }
}

/// Check if a string is a valid variable name
pub fn is_valid_var_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    
    let mut chars = name.chars();
    let first = chars.next().unwrap();
    
    // First character must be letter or underscore
    if !first.is_alphabetic() && first != '_' {
        return false;
    }
    
    // All characters must be alphanumeric or underscore
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

/// Parse a variable assignment string like "VAR=value"
/// Returns (var_name, var_value) if valid, None otherwise
pub fn parse_var_assignment(s: &str) -> Option<(String, String)> {
    let equals_pos = s.find('=')?;
    let var_name = &s[..equals_pos];
    let var_value = &s[equals_pos + 1..];
    
    if is_valid_var_name(var_name) {
        Some((var_name.to_string(), remove_quotes(var_value)))
    } else {
        None
    }
}