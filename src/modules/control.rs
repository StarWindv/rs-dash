//! Control structures implementation for rs-dash
//! Supports if, for, while, until, case, and select statements

use crate::modules::shell::Shell;

/// Control structure types
#[derive(Debug, Clone, PartialEq)]
pub enum ControlType {
    If,
    For,
    While,
    Until,
    Case,
    Select,
}

/// Condition for if/while/until statements
#[derive(Debug, Clone)]
pub enum Condition {
    /// Command condition (execute and check exit status)
    Command(String),
    /// Compound condition (for && and ||)
    Compound(Box<Condition>, LogicalOp, Box<Condition>),
    /// Negated condition
    Negated(Box<Condition>),
    /// True condition
    True,
    /// False condition
    False,
}

/// Logical operators for compound conditions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
}

/// For loop iterator
#[derive(Debug, Clone)]
pub struct ForLoop {
    pub variable: String,
    pub items: Vec<String>,
}

/// While/Until loop
#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: Condition,
    pub is_until: bool, // true for until, false for while
}

/// Case statement pattern
#[derive(Debug, Clone)]
pub struct CasePattern {
    pub pattern: String,
    pub commands: Vec<String>,
}

/// Case statement
#[derive(Debug, Clone)]
pub struct CaseStatement {
    pub word: String,
    pub patterns: Vec<CasePattern>,
}

/// Control structure
#[derive(Debug, Clone)]
pub struct ControlStructure {
    pub ctype: ControlType,
    pub condition: Option<Condition>,
    pub body: Vec<String>,
    pub else_body: Option<Vec<String>>,
    pub elif_conditions: Vec<(Condition, Vec<String>)>,
    pub for_loop: Option<ForLoop>,
    pub case_stmt: Option<CaseStatement>,
}

impl ControlStructure {
    /// Create a new if statement
    pub fn new_if(condition: Condition, body: Vec<String>) -> Self {
        Self {
            ctype: ControlType::If,
            condition: Some(condition),
            body,
            else_body: None,
            elif_conditions: Vec::new(),
            for_loop: None,
            case_stmt: None,
        }
    }
    
    /// Add else clause to if statement
    pub fn with_else(mut self, else_body: Vec<String>) -> Self {
        self.else_body = Some(else_body);
        self
    }
    
    /// Add elif clause to if statement
    pub fn add_elif(&mut self, condition: Condition, body: Vec<String>) {
        self.elif_conditions.push((condition, body));
    }
    
    /// Create a new for loop
    pub fn new_for(variable: String, items: Vec<String>, body: Vec<String>) -> Self {
        Self {
            ctype: ControlType::For,
            condition: None,
            body,
            else_body: None,
            elif_conditions: Vec::new(),
            for_loop: Some(ForLoop { variable, items }),
            case_stmt: None,
        }
    }
    
    /// Create a new while loop
    pub fn new_while(condition: Condition, body: Vec<String>) -> Self {
        Self {
            ctype: ControlType::While,
            condition: Some(condition),
            body,
            else_body: None,
            elif_conditions: Vec::new(),
            for_loop: None,
            case_stmt: None,
        }
    }
    
    /// Create a new until loop
    pub fn new_until(condition: Condition, body: Vec<String>) -> Self {
        Self {
            ctype: ControlType::Until,
            condition: Some(condition),
            body,
            else_body: None,
            elif_conditions: Vec::new(),
            for_loop: None,
            case_stmt: None,
        }
    }
    
    /// Create a new case statement
    pub fn new_case(word: String, patterns: Vec<CasePattern>) -> Self {
        Self {
            ctype: ControlType::Case,
            condition: None,
            body: Vec::new(),
            else_body: None,
            elif_conditions: Vec::new(),
            for_loop: None,
            case_stmt: Some(CaseStatement { word, patterns }),
        }
    }
}

/// Parser for control structures
pub struct ControlParser;

impl ControlParser {
    /// Parse an if statement from tokens
    pub fn parse_if(tokens: &[String]) -> Result<ControlStructure, String> {
        // TODO: Implement full if statement parsing
        // For now, return a simple implementation
        if tokens.len() < 4 {
            return Err("Invalid if statement: too few tokens".to_string());
        }
        
        // Simple condition: just the command after "if"
        let condition_start = 1; // Skip "if"
        let mut condition_end = condition_start;
        while condition_end < tokens.len() && tokens[condition_end] != "then" {
            condition_end += 1;
        }
        
        if condition_end >= tokens.len() {
            return Err("Invalid if statement: missing 'then'".to_string());
        }
        
        let condition_str = tokens[condition_start..condition_end].join(" ");
        let condition = Condition::Command(condition_str);
        
        // Find "then" and body
        let then_pos = condition_end;
        let body_start = then_pos + 1;
        let mut body_end = body_start;
        
        // Find "fi", "elif", or "else"
        while body_end < tokens.len() {
            if tokens[body_end] == "fi" || tokens[body_end] == "elif" || tokens[body_end] == "else" {
                break;
            }
            body_end += 1;
        }
        
        if body_end >= tokens.len() {
            return Err("Invalid if statement: missing 'fi'".to_string());
        }
        
        let body = tokens[body_start..body_end].join(" ");
        let body_commands = vec![body];
        
        let mut control = ControlStructure::new_if(condition, body_commands);
        
        // Check for elif or else
        let mut pos = body_end;
        while pos < tokens.len() {
            if tokens[pos] == "elif" {
                // Parse elif condition
                pos += 1;
                let elif_condition_start = pos;
                while pos < tokens.len() && tokens[pos] != "then" {
                    pos += 1;
                }
                
                if pos >= tokens.len() {
                    return Err("Invalid elif: missing 'then'".to_string());
                }
                
                let elif_condition_str = tokens[elif_condition_start..pos].join(" ");
                let elif_condition = Condition::Command(elif_condition_str);
                
                // Skip "then"
                pos += 1;
                let elif_body_start = pos;
                while pos < tokens.len() && tokens[pos] != "fi" && tokens[pos] != "elif" && tokens[pos] != "else" {
                    pos += 1;
                }
                
                let elif_body_str = tokens[elif_body_start..pos].join(" ");
                control.add_elif(elif_condition, vec![elif_body_str]);
                
                // Continue to check for more elif or else
                continue;
            } else if tokens[pos] == "else" {
                // Parse else body
                pos += 1;
                let else_body_start = pos;
                while pos < tokens.len() && tokens[pos] != "fi" {
                    pos += 1;
                }
                
                if pos >= tokens.len() {
                    return Err("Invalid else: missing 'fi'".to_string());
                }
                
                let else_body_str = tokens[else_body_start..pos].join(" ");
                control = control.with_else(vec![else_body_str]);
                break;
            } else if tokens[pos] == "fi" {
                break;
            } else {
                return Err(format!("Unexpected token in if statement: {}", tokens[pos]));
            }
        }
        
        Ok(control)
    }
    
    /// Parse a for loop from tokens
    pub fn parse_for(tokens: &[String]) -> Result<ControlStructure, String> {
        // TODO: Implement full for loop parsing
        // For now, return a simple implementation
        if tokens.len() < 4 {
            return Err("Invalid for loop: too few tokens".to_string());
        }
        
        // Expect format: for VAR in ITEMS; do COMMANDS; done
        if tokens[0] != "for" {
            return Err("Expected 'for' keyword".to_string());
        }
        
        let variable = tokens[1].clone();
        
        if tokens.len() < 3 || tokens[2] != "in" {
            return Err("Expected 'in' keyword".to_string());
        }
        
        let mut items = Vec::new();
        let mut pos = 3;
        
        // Collect items until ";" or "do"
        while pos < tokens.len() && tokens[pos] != ";" && tokens[pos] != "do" {
            items.push(tokens[pos].clone());
            pos += 1;
        }
        
        if pos >= tokens.len() {
            return Err("Invalid for loop: missing 'do'".to_string());
        }
        
        // Skip ";" if present
        if tokens[pos] == ";" {
            pos += 1;
        }
        
        if pos >= tokens.len() || tokens[pos] != "do" {
            return Err("Expected 'do' keyword".to_string());
        }
        
        pos += 1;
        let body_start = pos;
        
        // Find "done"
        while pos < tokens.len() && tokens[pos] != "done" {
            pos += 1;
        }
        
        if pos >= tokens.len() {
            return Err("Invalid for loop: missing 'done'".to_string());
        }
        
        let body_str = tokens[body_start..pos].join(" ");
        let body_commands = vec![body_str];
        
        Ok(ControlStructure::new_for(variable, items, body_commands))
    }
    
    /// Parse a while loop from tokens
    pub fn parse_while(tokens: &[String]) -> Result<ControlStructure, String> {
        // TODO: Implement full while loop parsing
        // For now, return a simple implementation
        if tokens.len() < 4 {
            return Err("Invalid while loop: too few tokens".to_string());
        }
        
        // Expect format: while CONDITION; do COMMANDS; done
        if tokens[0] != "while" {
            return Err("Expected 'while' keyword".to_string());
        }
        
        let mut pos = 1;
        let mut condition_end = pos;
        
        // Find condition end (until ";" or "do")
        while condition_end < tokens.len() && tokens[condition_end] != ";" && tokens[condition_end] != "do" {
            condition_end += 1;
        }
        
        if condition_end >= tokens.len() {
            return Err("Invalid while loop: missing 'do'".to_string());
        }
        
        let condition_str = tokens[pos..condition_end].join(" ");
        let condition = Condition::Command(condition_str);
        
        pos = condition_end;
        
        // Skip ";" if present
        if tokens[pos] == ";" {
            pos += 1;
        }
        
        if pos >= tokens.len() || tokens[pos] != "do" {
            return Err("Expected 'do' keyword".to_string());
        }
        
        pos += 1;
        let body_start = pos;
        
        // Find "done"
        while pos < tokens.len() && tokens[pos] != "done" {
            pos += 1;
        }
        
        if pos >= tokens.len() {
            return Err("Invalid while loop: missing 'done'".to_string());
        }
        
        let body_str = tokens[body_start..pos].join(" ");
        let body_commands = vec![body_str];
        
        Ok(ControlStructure::new_while(condition, body_commands))
    }
    
    /// Parse an until loop from tokens
    pub fn parse_until(tokens: &[String]) -> Result<ControlStructure, String> {
        // TODO: Implement full until loop parsing
        // For now, return a simple implementation (same as while but with is_until flag)
        if tokens.len() < 4 {
            return Err("Invalid until loop: too few tokens".to_string());
        }
        
        // Expect format: until CONDITION; do COMMANDS; done
        if tokens[0] != "until" {
            return Err("Expected 'until' keyword".to_string());
        }
        
        let mut pos = 1;
        let mut condition_end = pos;
        
        // Find condition end (until ";" or "do")
        while condition_end < tokens.len() && tokens[condition_end] != ";" && tokens[condition_end] != "do" {
            condition_end += 1;
        }
        
        if condition_end >= tokens.len() {
            return Err("Invalid until loop: missing 'do'".to_string());
        }
        
        let condition_str = tokens[pos..condition_end].join(" ");
        let condition = Condition::Command(condition_str);
        
        pos = condition_end;
        
        // Skip ";" if present
        if tokens[pos] == ";" {
            pos += 1;
        }
        
        if pos >= tokens.len() || tokens[pos] != "do" {
            return Err("Expected 'do' keyword".to_string());
        }
        
        pos += 1;
        let body_start = pos;
        
        // Find "done"
        while pos < tokens.len() && tokens[pos] != "done" {
            pos += 1;
        }
        
        if pos >= tokens.len() {
            return Err("Invalid until loop: missing 'done'".to_string());
        }
        
        let body_str = tokens[body_start..pos].join(" ");
        let body_commands = vec![body_str];
        
        Ok(ControlStructure::new_until(condition, body_commands))
    }
    
    /// Parse a case statement from tokens
    pub fn parse_case(tokens: &[String]) -> Result<ControlStructure, String> {
        // TODO: Implement full case statement parsing
        // For now, return a simple implementation
        if tokens.len() < 4 {
            return Err("Invalid case statement: too few tokens".to_string());
        }
        
        // Expect format: case WORD in PATTERN) COMMANDS;; esac
        if tokens[0] != "case" {
            return Err("Expected 'case' keyword".to_string());
        }
        
        let word = tokens[1].clone();
        
        if tokens.len() < 3 || tokens[2] != "in" {
            return Err("Expected 'in' keyword".to_string());
        }
        
        let mut patterns = Vec::new();
        let mut pos = 3;
        
        while pos < tokens.len() && tokens[pos] != "esac" {
            // Parse pattern
            let pattern = tokens[pos].clone();
            pos += 1;
            
            if pos >= tokens.len() || tokens[pos] != ")" {
                return Err("Expected ')' after pattern".to_string());
            }
            pos += 1;
            
            // Parse commands until ;;
            let mut commands = Vec::new();
            while pos < tokens.len() && tokens[pos] != ";;" && tokens[pos] != "esac" {
                commands.push(tokens[pos].clone());
                pos += 1;
            }
            
            patterns.push(CasePattern { pattern, commands });
            
            // Skip ;;
            if pos < tokens.len() && tokens[pos] == ";;" {
                pos += 1;
            }
        }
        
        if pos >= tokens.len() || tokens[pos] != "esac" {
            return Err("Invalid case statement: missing 'esac'".to_string());
        }
        
        Ok(ControlStructure::new_case(word, patterns))
    }
    
    /// Parse a select statement from tokens
    pub fn parse_select(tokens: &[String]) -> Result<ControlStructure, String> {
        // TODO: Implement full select statement parsing
        // For now, return a placeholder implementation
        if tokens.len() < 4 {
            return Err("Invalid select statement: too few tokens".to_string());
        }
        
        // Expect format: select VAR in ITEMS; do COMMANDS; done
        if tokens[0] != "select" {
            return Err("Expected 'select' keyword".to_string());
        }
        
        let variable = tokens[1].clone();
        
        if tokens.len() < 3 || tokens[2] != "in" {
            return Err("Expected 'in' keyword".to_string());
        }
        
        let mut items = Vec::new();
        let mut pos = 3;
        
        // Collect items until ";" or "do"
        while pos < tokens.len() && tokens[pos] != ";" && tokens[pos] != "do" {
            items.push(tokens[pos].clone());
            pos += 1;
        }
        
        if pos >= tokens.len() {
            return Err("Invalid select statement: missing 'do'".to_string());
        }
        
        // Skip ";" if present
        if tokens[pos] == ";" {
            pos += 1;
        }
        
        if pos >= tokens.len() || tokens[pos] != "do" {
            return Err("Expected 'do' keyword".to_string());
        }
        
        pos += 1;
        let body_start = pos;
        
        // Find "done"
        while pos < tokens.len() && tokens[pos] != "done" {
            pos += 1;
        }
        
        if pos >= tokens.len() {
            return Err("Invalid select statement: missing 'done'".to_string());
        }
        
        let body_str = tokens[body_start..pos].join(" ");
        let body_commands = vec![body_str];
        
        // Create a for loop as a placeholder (select is similar to for but with interactive selection)
        Ok(ControlStructure::new_for(variable, items, body_commands))
    }
}

/// Executor for control structures
pub struct ControlExecutor;

impl ControlExecutor {
    /// Execute a control structure
    pub fn execute(shell: &mut Shell, control: &ControlStructure) -> i32 {
        match control.ctype {
            ControlType::If => Self::execute_if(shell, control),
            ControlType::For => Self::execute_for(shell, control),
            ControlType::While => Self::execute_while(shell, control),
            ControlType::Until => Self::execute_until(shell, control),
            ControlType::Case => Self::execute_case(shell, control),
            ControlType::Select => Self::execute_select(shell, control),
        }
    }
    
    /// Execute if statement
    fn execute_if(shell: &mut Shell, control: &ControlStructure) -> i32 {
        // Evaluate main condition
        if let Some(condition) = &control.condition {
            if Self::evaluate_condition(shell, condition) {
                // Execute main body
                return Self::execute_body(shell, &control.body);
            }
        }
        
        // Check elif conditions
        for (elif_condition, elif_body) in &control.elif_conditions {
            if Self::evaluate_condition(shell, elif_condition) {
                return Self::execute_body(shell, elif_body);
            }
        }
        
        // Execute else body if it exists
        if let Some(else_body) = &control.else_body {
            return Self::execute_body(shell, else_body);
        }
        
        0 // Default exit status
    }
    
    /// Execute for loop
    fn execute_for(shell: &mut Shell, control: &ControlStructure) -> i32 {
        if let Some(for_loop) = &control.for_loop {
            let mut last_status = 0;
            
            // Expand items (they might contain variables, command substitutions, etc.)
            let mut expanded_items = Vec::new();
            for item in &for_loop.items {
                // Expand variables in the item
                let expanded = crate::modules::expansion::expand_variables(shell, item);
                
                // If the expanded item contains word splitting (spaces), split it
                // This handles cases like: for i in $(echo "a b c"); do ...
                let split_items: Vec<String> = if expanded.contains(' ') {
                    expanded.split_whitespace()
                        .map(|s| s.to_string())
                        .collect()
                } else {
                    vec![expanded]
                };
                
                expanded_items.extend(split_items);
            }
            
            // If no items specified, use positional parameters
            let items = if expanded_items.is_empty() {
                shell.positional_params.clone()
            } else {
                expanded_items
            };
            
            for item in items {
                // Set the variable for this iteration
                shell.env_vars.insert(for_loop.variable.clone(), item.clone());
                
                // Execute the body
                last_status = Self::execute_body(shell, &control.body);
                
                // Break if a command in the body failed with set -e?
                // TODO: Implement set -e handling
                
                // Remove the loop variable to avoid polluting environment
                // (In dash, for loop variables persist after the loop)
                // shell.env_vars.remove(&for_loop.variable);
            }
            
            return last_status;
        }
        
        0
    }
    
    /// Execute while loop
    fn execute_while(shell: &mut Shell, control: &ControlStructure) -> i32 {
        let mut last_status = 0;
        
        if let Some(condition) = &control.condition {
            // Continue while condition is true
            while Self::evaluate_condition(shell, condition) {
                last_status = Self::execute_body(shell, &control.body);
                
                // TODO: Handle break/continue
                // TODO: Implement set -e handling
            }
        }
        
        last_status
    }
    
    /// Execute until loop
    fn execute_until(shell: &mut Shell, control: &ControlStructure) -> i32 {
        let mut last_status = 0;
        
        if let Some(condition) = &control.condition {
            // Continue until condition becomes true
            while !Self::evaluate_condition(shell, condition) {
                last_status = Self::execute_body(shell, &control.body);
                
                // TODO: Handle break/continue
                // TODO: Implement set -e handling
            }
        }
        
        last_status
    }
    
    /// Execute case statement
    fn execute_case(shell: &mut Shell, control: &ControlStructure) -> i32 {
        // TODO: Implement case statement
        // For now, just return 0
        if let Some(case_stmt) = &control.case_stmt {
            // Expand the word
            let word = case_stmt.word.clone(); // TODO: Expand variables
            
            for pattern in &case_stmt.patterns {
                // TODO: Implement pattern matching
                // For now, just check exact match
                if word == pattern.pattern {
                    return Self::execute_body(shell, &pattern.commands);
                }
            }
        }
        
        0
    }
    
    /// Execute select statement
    fn execute_select(_shell: &mut Shell, _control: &ControlStructure) -> i32 {
        // TODO: Implement select statement
        // For now, just return 0
        0
    }
    
    /// Evaluate a condition
    fn evaluate_condition(shell: &mut Shell, condition: &Condition) -> bool {
        match condition {
            Condition::Command(cmd_str) => {
                // Execute the command and check exit status
                let status = shell.execute_command_line(cmd_str);
                status == 0
            }
            Condition::Compound(left, op, right) => {
                let left_result = Self::evaluate_condition(shell, left);
                match op {
                    LogicalOp::And => left_result && Self::evaluate_condition(shell, right),
                    LogicalOp::Or => left_result || Self::evaluate_condition(shell, right),
                }
            }
            Condition::Negated(cond) => !Self::evaluate_condition(shell, cond),
            Condition::True => true,
            Condition::False => false,
        }
    }
    
    /// Execute a body of commands
    fn execute_body(shell: &mut Shell, body: &[String]) -> i32 {
        let mut last_status = 0;
        
        for cmd in body {
            last_status = shell.execute_command_line(cmd);
        }
        
        last_status
    }
}

/// Check if a line starts a control structure
pub fn is_control_structure(line: &str) -> bool {
    let trimmed = line.trim_start();
    
    // Check for multi-line control structures
    if trimmed.starts_with("if ") || trimmed == "if" {
        return true;
    }
    if trimmed.starts_with("for ") || trimmed == "for" {
        return true;
    }
    if trimmed.starts_with("while ") || trimmed == "while" {
        return true;
    }
    if trimmed.starts_with("until ") || trimmed == "until" {
        return true;
    }
    if trimmed.starts_with("case ") || trimmed == "case" {
        return true;
    }
    if trimmed.starts_with("select ") || trimmed == "select" {
        return true;
    }
    
    false
}

/// Parse a control structure from a line
pub fn parse_control_structure(line: &str) -> Result<ControlStructure, String> {
    // First, tokenize the line properly
    let tokens = tokenize_control_line(line)?;
    
    if tokens.is_empty() {
        return Err("Empty line".to_string());
    }
    
    match tokens[0].as_str() {
        "if" => ControlParser::parse_if(&tokens),
        "for" => ControlParser::parse_for(&tokens),
        "while" => ControlParser::parse_while(&tokens),
        "until" => ControlParser::parse_until(&tokens),
        "case" => ControlParser::parse_case(&tokens),
        "select" => ControlParser::parse_select(&tokens),
        _ => Err("Not a control structure".to_string()),
    }
}

/// Tokenize a control structure line, handling quotes and special characters
fn tokenize_control_line(line: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = '\0';
    let mut escape_next = false;
    let mut in_subshell = 0;
    let mut in_command_sub = 0;
    
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        let c = chars[i];
        
        if escape_next {
            current.push(c);
            escape_next = false;
            i += 1;
            continue;
        }
        
        match c {
            '\\' => {
                escape_next = true;
                i += 1;
                continue;
            }
            '\'' | '"' => {
                if !in_quote {
                    in_quote = true;
                    quote_char = c;
                    current.push(c);
                } else if c == quote_char {
                    in_quote = false;
                    quote_char = '\0';
                    current.push(c);
                } else {
                    current.push(c);
                }
                i += 1;
            }
            '(' => {
                // Check if it's command substitution: $(
                if i > 0 && chars[i-1] == '$' {
                    in_command_sub += 1;
                } else if !in_quote {
                    in_subshell += 1;
                }
                current.push(c);
                i += 1;
            }
            ')' => {
                if in_command_sub > 0 {
                    in_command_sub -= 1;
                } else if in_subshell > 0 {
                    in_subshell -= 1;
                }
                current.push(c);
                i += 1;
            }
            ';' | '\n' => {
                if !in_quote && in_subshell == 0 && in_command_sub == 0 {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    tokens.push(c.to_string());
                } else {
                    current.push(c);
                }
                i += 1;
            }
            ' ' | '\t' => {
                if !in_quote && in_subshell == 0 && in_command_sub == 0 {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                } else {
                    current.push(c);
                }
                i += 1;
            }
            _ => {
                current.push(c);
                i += 1;
            }
        }
    }
    
    if !current.is_empty() {
        tokens.push(current);
    }
    
    Ok(tokens)
}