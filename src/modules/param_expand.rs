//! Parameter expansion system

use crate::modules::shell::Shell;

/// Parameter expansion types
#[derive(Debug, Clone)]
pub enum ParamExpansion {
    /// Simple variable: $VAR or ${VAR}
    Simple(String),
    
    /// Use default value if unset or null: ${VAR:-word}
    UseDefault {
        param: String,
        word: String,
        assign: bool,  // true for ${VAR:=word} (assign default)
    },
    
    /// Display error if unset or null: ${VAR:?word}
    ErrorIfUnset {
        param: String,
        message: String,
    },
    
    /// Use alternate value if set: ${VAR:+word}
    UseAlternate {
        param: String,
        word: String,
    },
    
    /// String length: ${#VAR}
    Length(String),
    
    /// Remove smallest suffix pattern: ${VAR%pattern}
    RemoveSuffix {
        param: String,
        pattern: String,
        longest: bool,  // true for %% (largest)
    },
    
    /// Remove smallest prefix pattern: ${VAR#pattern}
    RemovePrefix {
        param: String,
        pattern: String,
        longest: bool,  // true for ## (largest)
    },
    
    /// Pattern substitution: ${VAR/pattern/replacement}
    Substitution {
        param: String,
        pattern: String,
        replacement: String,
        global: bool,  // true for // (global substitution)
    },
    
    /// Case modification: ${VAR^} (uppercase first), ${VAR^^} (uppercase all)
    CaseModification {
        param: String,
        operation: CaseOp,
    },
}

#[derive(Debug, Clone)]
pub enum CaseOp {
    UpperFirst,    // ^
    UpperAll,      // ^^
    LowerFirst,    // ,
    LowerAll,      // ,,
}

/// Parse a parameter expansion expression
pub fn parse_param_expansion(input: &str) -> Result<ParamExpansion, String> {
    // Remove ${ and }
    let content = input.trim_start_matches('$').trim_start_matches('{').trim_end_matches('}');
    
    if content.is_empty() {
        return Err("Empty parameter expansion".to_string());
    }
    
    // Check for special forms
    if content.starts_with('#') {
        // String length: ${#VAR}
        let param = content[1..].to_string();
        return Ok(ParamExpansion::Length(param));
    }
    
    // Check for pattern operations
    if let Some(pos) = content.find(['%', '#', '/', '^', ','].as_ref()) {
        let param = content[..pos].to_string();
        let rest = &content[pos..];
        
        // Check operation type
        if rest.starts_with('%') {
            let (pattern, longest) = if rest.starts_with("%%") {
                (&rest[2..], true)
            } else {
                (&rest[1..], false)
            };
            return Ok(ParamExpansion::RemoveSuffix {
                param,
                pattern: pattern.to_string(),
                longest,
            });
        } else if rest.starts_with('#') {
            let (pattern, longest) = if rest.starts_with("##") {
                (&rest[2..], true)
            } else {
                (&rest[1..], false)
            };
            return Ok(ParamExpansion::RemovePrefix {
                param,
                pattern: pattern.to_string(),
                longest,
            });
        } else if rest.starts_with('/') {
            // Pattern substitution
            let parts: Vec<&str> = rest.split('/').collect();
            if parts.len() >= 3 {
                let global = rest.starts_with("//");
                // For global: "//pattern/replacement" -> parts = ["", "", "pattern", "replacement"]
                // For single: "/pattern/replacement" -> parts = ["", "pattern", "replacement"]
                let pattern_idx = if global { 2 } else { 1 };
                let replacement_idx = pattern_idx + 1;
                
                if replacement_idx < parts.len() {
                    let pattern = parts[pattern_idx].to_string();
                    let replacement = parts[replacement_idx].to_string();
                    return Ok(ParamExpansion::Substitution {
                        param,
                        pattern,
                        replacement,
                        global,
                    });
                }
            }
        } else if rest.starts_with('^') {
            let operation = if rest.starts_with("^^") {
                CaseOp::UpperAll
            } else {
                CaseOp::UpperFirst
            };
            return Ok(ParamExpansion::CaseModification {
                param,
                operation,
            });
        } else if rest.starts_with(',') {
            let operation = if rest.starts_with(",,") {
                CaseOp::LowerAll
            } else {
                CaseOp::LowerFirst
            };
            return Ok(ParamExpansion::CaseModification {
                param,
                operation,
            });
        }
    }
    
    // Check for :- := :? :+ operators
    if let Some(pos) = content.find(':') {
        let param = content[..pos].to_string();
        let rest = &content[pos..];
        
        if rest.starts_with(":-") {
            return Ok(ParamExpansion::UseDefault {
                param,
                word: rest[2..].to_string(),
                assign: false,
            });
        } else if rest.starts_with(":=") {
            return Ok(ParamExpansion::UseDefault {
                param,
                word: rest[2..].to_string(),
                assign: true,
            });
        } else if rest.starts_with(":?") {
            return Ok(ParamExpansion::ErrorIfUnset {
                param,
                message: rest[2..].to_string(),
            });
        } else if rest.starts_with(":+") {
            return Ok(ParamExpansion::UseAlternate {
                param,
                word: rest[2..].to_string(),
            });
        }
    }
    
    // Simple variable
    Ok(ParamExpansion::Simple(content.to_string()))
}

/// Expand a parameter expression
pub fn expand_param(shell: &mut Shell, expansion: &ParamExpansion) -> Result<String, String> {
    match expansion {
        ParamExpansion::Simple(param) => {
            // Get value from shell
            if param == "@" || param == "*" {
                // Positional parameters - special handling for @ vs *
                // For now, join all with space
                Ok(shell.positional_params.join(" "))
            } else if param == "#" {
                // Number of positional parameters
                Ok(shell.positional_param_count().to_string())
            } else if param == "?" {
                // Exit status
                Ok(shell.last_exit_status.to_string())
            } else if param == "$" {
                // PID
                Ok(std::process::id().to_string())
            } else if param == "0" {
                // Shell name
                Ok(shell.shell_name.clone())
            } else if param == "!" {
                // PID of last background command
                Ok("".to_string())  // TODO: Implement job control
            } else if param == "-" {
                // Current option flags
                Ok(shell.options.clone())
            } else if param.chars().all(|c| c.is_digit(10)) {
                // Positional parameter by number: $1, $2, etc.
                match param.parse::<usize>() {
                    Ok(n) => {
                        if n == 0 {
                            Ok(shell.shell_name.clone())
                        } else {
                            Ok(shell.get_positional_param(n).unwrap_or("").to_string())
                        }
                    }
                    Err(_) => Ok(shell.env_vars.get(param).cloned().unwrap_or_default()),
                }
            } else {
                // Regular variable
                Ok(shell.env_vars.get(param).cloned().unwrap_or_default())
            }
        }
        
        ParamExpansion::UseDefault { param, word, assign } => {
            let value = shell.env_vars.get(param).cloned();
            if value.is_none() || value.as_ref().unwrap().is_empty() {
                if *assign {
                    // Assign default value
                    shell.env_vars.insert(param.clone(), word.clone());
                }
                Ok(word.clone())
            } else {
                Ok(value.unwrap())
            }
        }
        
        ParamExpansion::ErrorIfUnset { param, message } => {
            let value = shell.env_vars.get(param).cloned();
            if value.is_none() || value.as_ref().unwrap().is_empty() {
                let msg = if message.is_empty() {
                    format!("{}: parameter null or not set", param)
                } else {
                    message.clone()
                };
                Err(msg)
            } else {
                Ok(value.unwrap())
            }
        }
        
        ParamExpansion::UseAlternate { param, word } => {
            let value = shell.env_vars.get(param).cloned();
            if value.is_none() || value.as_ref().unwrap().is_empty() {
                Ok(String::new())
            } else {
                Ok(word.clone())
            }
        }
        
        ParamExpansion::Length(param) => {
            let value = shell.env_vars.get(param).cloned().unwrap_or_default();
            Ok(value.len().to_string())
        }
        
        ParamExpansion::RemoveSuffix { param, pattern, longest } => {
            let value = shell.env_vars.get(param).cloned().unwrap_or_default();
            if *longest {
                // Remove largest suffix pattern
                if let Some(pos) = value.rfind(pattern) {
                    Ok(value[..pos].to_string())
                } else {
                    Ok(value)
                }
            } else {
                // Remove smallest suffix pattern
                if let Some(pos) = value.find(pattern) {
                    Ok(value[..pos].to_string())
                } else {
                    Ok(value)
                }
            }
        }
        
        ParamExpansion::RemovePrefix { param, pattern, longest } => {
            let value = shell.env_vars.get(param).cloned().unwrap_or_default();
            if *longest {
                // Remove largest prefix pattern
                if pattern.is_empty() {
                    Ok(value)
                } else {
                    let mut result = value.clone();
                    while result.starts_with(pattern) {
                        result = result[pattern.len()..].to_string();
                    }
                    Ok(result)
                }
            } else {
                // Remove smallest prefix pattern
                if value.starts_with(pattern) {
                    Ok(value[pattern.len()..].to_string())
                } else {
                    Ok(value)
                }
            }
        }
        
        ParamExpansion::Substitution { param, pattern, replacement, global } => {
            let value = shell.env_vars.get(param).cloned().unwrap_or_default();
            if *global {
                // Global substitution
                Ok(value.replace(pattern, replacement))
            } else {
                // First substitution only
                if let Some(pos) = value.find(pattern) {
                    let mut result = value[..pos].to_string();
                    result.push_str(replacement);
                    result.push_str(&value[pos + pattern.len()..]);
                    Ok(result)
                } else {
                    Ok(value)
                }
            }
        }
        
        ParamExpansion::CaseModification { param, operation } => {
            let value = shell.env_vars.get(param).cloned().unwrap_or_default();
            match operation {
                CaseOp::UpperFirst => {
                    let mut chars = value.chars();
                    if let Some(first) = chars.next() {
                        let mut result = first.to_uppercase().to_string();
                        result.extend(chars);
                        Ok(result)
                    } else {
                        Ok(value)
                    }
                }
                CaseOp::UpperAll => Ok(value.to_uppercase()),
                CaseOp::LowerFirst => {
                    let mut chars = value.chars();
                    if let Some(first) = chars.next() {
                        let mut result = first.to_lowercase().to_string();
                        result.extend(chars);
                        Ok(result)
                    } else {
                        Ok(value)
                    }
                }
                CaseOp::LowerAll => Ok(value.to_lowercase()),
            }
        }
    }
}