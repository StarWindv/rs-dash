//! Arithmetic expansion

use std::collections::HashMap;

use crate::modules::shell::Shell;

/// Arithmetic expression token types
#[derive(Debug, Clone, PartialEq)]
pub enum ArithToken {
    Number(i64),
    Variable(String),
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Percent,   // %
    Caret,     // ^
    And,       // &
    Or,        // |
    Tilde,     // ~
    LShift,    // <<
    RShift,    // >>
    LParen,    // (
    RParen,    // )
    Question,  // ?
    Colon,     // :
    Comma,     // ,
    Eq,        // ==
    Ne,        // !=
    Lt,        // <
    Le,        // <=
    Gt,        // >
    Ge,        // >=
    AndAnd,    // &&
    OrOr,      // ||
    Not,       // !
    Assign,    // =
    MulAssign, // *=
    DivAssign, // /=
    ModAssign, // %=
    AddAssign, // +=
    SubAssign, // -=
    ShlAssign, // <<=
    ShrAssign, // >>=
    AndAssign, // &=
    OrAssign,  // |=
    XorAssign, // ^=
    Inc,       // ++
    Dec,       // --
    Eof,
    Error(String),
}

/// Arithmetic expression parser and evaluator
pub struct ArithmeticEvaluator {
    variables: HashMap<String, i64>,
}

impl ArithmeticEvaluator {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    
    /// Evaluate an arithmetic expression string
    pub fn evaluate(&mut self, expr: &str, shell: &Shell) -> Result<i64, String> {
        let tokens = self.tokenize(expr)?;
        self.parse_and_eval(&tokens, shell)
    }
    
    /// Tokenize arithmetic expression
    fn tokenize(&self, expr: &str) -> Result<Vec<ArithToken>, String> {
        let mut tokens = Vec::new();
        let mut chars = expr.chars().peekable();
        let mut pos = 0;
        
        while let Some(&c) = chars.peek() {
            pos += 1;
            
            match c {
                // Whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next();
                    continue;
                }
                
                // Numbers
                '0'..='9' => {
                    let mut num_str = String::new();
                    while let Some(&d) = chars.peek() {
                        if d.is_digit(10) {
                            num_str.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    // Parse number with different bases
                    let num = if num_str.starts_with("0x") || num_str.starts_with("0X") {
                        i64::from_str_radix(&num_str[2..], 16)
                    } else if num_str.starts_with("0") && num_str.len() > 1 {
                        i64::from_str_radix(&num_str[1..], 8)
                    } else {
                        num_str.parse()
                    };
                    
                    match num {
                        Ok(n) => tokens.push(ArithToken::Number(n)),
                        Err(_) => return Err(format!("Invalid number: {}", num_str)),
                    }
                }
                
                // Variables and identifiers
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            ident.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(ArithToken::Variable(ident));
                }
                
                // Operators
                '+' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '+' {
                            chars.next();
                            tokens.push(ArithToken::Inc);
                        } else if next == '=' {
                            chars.next();
                            tokens.push(ArithToken::AddAssign);
                        } else {
                            tokens.push(ArithToken::Plus);
                        }
                    } else {
                        tokens.push(ArithToken::Plus);
                    }
                }
                
                '-' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '-' {
                            chars.next();
                            tokens.push(ArithToken::Dec);
                        } else if next == '=' {
                            chars.next();
                            tokens.push(ArithToken::SubAssign);
                        } else {
                            tokens.push(ArithToken::Minus);
                        }
                    } else {
                        tokens.push(ArithToken::Minus);
                    }
                }
                
                '*' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            chars.next();
                            tokens.push(ArithToken::MulAssign);
                        } else {
                            tokens.push(ArithToken::Star);
                        }
                    } else {
                        tokens.push(ArithToken::Star);
                    }
                }
                
                '/' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            chars.next();
                            tokens.push(ArithToken::DivAssign);
                        } else {
                            tokens.push(ArithToken::Slash);
                        }
                    } else {
                        tokens.push(ArithToken::Slash);
                    }
                }
                
                '%' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            chars.next();
                            tokens.push(ArithToken::ModAssign);
                        } else {
                            tokens.push(ArithToken::Percent);
                        }
                    } else {
                        tokens.push(ArithToken::Percent);
                    }
                }
                
                '^' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            chars.next();
                            tokens.push(ArithToken::XorAssign);
                        } else {
                            tokens.push(ArithToken::Caret);
                        }
                    } else {
                        tokens.push(ArithToken::Caret);
                    }
                }
                
                '&' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '&' {
                            chars.next();
                            tokens.push(ArithToken::AndAnd);
                        } else if next == '=' {
                            chars.next();
                            tokens.push(ArithToken::AndAssign);
                        } else {
                            tokens.push(ArithToken::And);
                        }
                    } else {
                        tokens.push(ArithToken::And);
                    }
                }
                
                '|' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '|' {
                            chars.next();
                            tokens.push(ArithToken::OrOr);
                        } else if next == '=' {
                            chars.next();
                            tokens.push(ArithToken::OrAssign);
                        } else {
                            tokens.push(ArithToken::Or);
                        }
                    } else {
                        tokens.push(ArithToken::Or);
                    }
                }
                
                '~' => {
                    chars.next();
                    tokens.push(ArithToken::Tilde);
                }
                
                '<' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '<' {
                            chars.next();
                            if let Some(&next2) = chars.peek() {
                                if next2 == '=' {
                                    chars.next();
                                    tokens.push(ArithToken::ShlAssign);
                                } else {
                                    tokens.push(ArithToken::LShift);
                                }
                            } else {
                                tokens.push(ArithToken::LShift);
                            }
                        } else if next == '=' {
                            chars.next();
                            tokens.push(ArithToken::Le);
                        } else {
                            tokens.push(ArithToken::Lt);
                        }
                    } else {
                        tokens.push(ArithToken::Lt);
                    }
                }
                
                '>' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '>' {
                            chars.next();
                            if let Some(&next2) = chars.peek() {
                                if next2 == '=' {
                                    chars.next();
                                    tokens.push(ArithToken::ShrAssign);
                                } else {
                                    tokens.push(ArithToken::RShift);
                                }
                            } else {
                                tokens.push(ArithToken::RShift);
                            }
                        } else if next == '=' {
                            chars.next();
                            tokens.push(ArithToken::Ge);
                        } else {
                            tokens.push(ArithToken::Gt);
                        }
                    } else {
                        tokens.push(ArithToken::Gt);
                    }
                }
                
                '=' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            chars.next();
                            tokens.push(ArithToken::Eq);
                        } else {
                            tokens.push(ArithToken::Assign);
                        }
                    } else {
                        tokens.push(ArithToken::Assign);
                    }
                }
                
                '!' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            chars.next();
                            tokens.push(ArithToken::Ne);
                        } else {
                            tokens.push(ArithToken::Not);
                        }
                    } else {
                        tokens.push(ArithToken::Not);
                    }
                }
                
                '(' => {
                    chars.next();
                    tokens.push(ArithToken::LParen);
                }
                
                ')' => {
                    chars.next();
                    tokens.push(ArithToken::RParen);
                }
                
                '?' => {
                    chars.next();
                    tokens.push(ArithToken::Question);
                }
                
                ':' => {
                    chars.next();
                    tokens.push(ArithToken::Colon);
                }
                
                ',' => {
                    chars.next();
                    tokens.push(ArithToken::Comma);
                }
                
                // Unknown character
                _ => {
                    return Err(format!("Unexpected character '{}' at position {}", c, pos));
                }
            }
        }
        
        tokens.push(ArithToken::Eof);
        Ok(tokens)
    }
    
    /// Parse and evaluate token stream
    fn parse_and_eval(&mut self, tokens: &[ArithToken], shell: &Shell) -> Result<i64, String> {
        // Simple recursive descent parser
        let mut pos = 0;
        self.parse_expression(tokens, &mut pos, shell)
    }
    
    fn parse_expression(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        self.parse_assignment(tokens, pos, shell)
    }
    
    fn parse_assignment(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        let left = self.parse_conditional(tokens, pos, shell)?;
        
        match tokens.get(*pos) {
            Some(ArithToken::Assign) => {
                // Assignment - need variable name
                // For simplicity, we'll handle this later
                *pos += 1;
                let right = self.parse_assignment(tokens, pos, shell)?;
                Ok(right)
            }
            _ => Ok(left),
        }
    }
    
    fn parse_conditional(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        let expr = self.parse_logical_or(tokens, pos, shell)?;
        
        if tokens.get(*pos) == Some(&ArithToken::Question) {
            *pos += 1;
            let then_expr = self.parse_expression(tokens, pos, shell)?;
            
            if tokens.get(*pos) == Some(&ArithToken::Colon) {
                *pos += 1;
                let else_expr = self.parse_conditional(tokens, pos, shell)?;
                Ok(if expr != 0 { then_expr } else { else_expr })
            } else {
                Err("Expected ':' in ternary operator".to_string())
            }
        } else {
            Ok(expr)
        }
    }
    
    fn parse_logical_or(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        let mut left = self.parse_logical_and(tokens, pos, shell)?;
        
        while tokens.get(*pos) == Some(&ArithToken::OrOr) {
            *pos += 1;
            let right = self.parse_logical_and(tokens, pos, shell)?;
            left = if left != 0 || right != 0 { 1 } else { 0 };
        }
        
        Ok(left)
    }
    
    fn parse_logical_and(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        let mut left = self.parse_bitwise_or(tokens, pos, shell)?;
        
        while tokens.get(*pos) == Some(&ArithToken::AndAnd) {
            *pos += 1;
            let right = self.parse_bitwise_or(tokens, pos, shell)?;
            left = if left != 0 && right != 0 { 1 } else { 0 };
        }
        
        Ok(left)
    }
    
    fn parse_bitwise_or(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        let mut left = self.parse_bitwise_xor(tokens, pos, shell)?;
        
        while tokens.get(*pos) == Some(&ArithToken::Or) {
            *pos += 1;
            let right = self.parse_bitwise_xor(tokens, pos, shell)?;
            left |= right;
        }
        
        Ok(left)
    }
    
    fn parse_bitwise_xor(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        let mut left = self.parse_bitwise_and(tokens, pos, shell)?;
        
        while tokens.get(*pos) == Some(&ArithToken::Caret) {
            *pos += 1;
            let right = self.parse_bitwise_and(tokens, pos, shell)?;
            left ^= right;
        }
        
        Ok(left)
    }
    
    fn parse_bitwise_and(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        let mut left = self.parse_equality(tokens, pos, shell)?;
        
        while tokens.get(*pos) == Some(&ArithToken::And) {
            *pos += 1;
            let right = self.parse_equality(tokens, pos, shell)?;
            left &= right;
        }
        
        Ok(left)
    }
    
    fn parse_equality(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        let mut left = self.parse_relational(tokens, pos, shell)?;
        
        loop {
            match tokens.get(*pos) {
                Some(ArithToken::Eq) => {
                    *pos += 1;
                    let right = self.parse_relational(tokens, pos, shell)?;
                    left = if left == right { 1 } else { 0 };
                }
                Some(ArithToken::Ne) => {
                    *pos += 1;
                    let right = self.parse_relational(tokens, pos, shell)?;
                    left = if left != right { 1 } else { 0 };
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_relational(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        let mut left = self.parse_shift(tokens, pos, shell)?;
        
        loop {
            match tokens.get(*pos) {
                Some(ArithToken::Lt) => {
                    *pos += 1;
                    let right = self.parse_shift(tokens, pos, shell)?;
                    left = if left < right { 1 } else { 0 };
                }
                Some(ArithToken::Le) => {
                    *pos += 1;
                    let right = self.parse_shift(tokens, pos, shell)?;
                    left = if left <= right { 1 } else { 0 };
                }
                Some(ArithToken::Gt) => {
                    *pos += 1;
                    let right = self.parse_shift(tokens, pos, shell)?;
                    left = if left > right { 1 } else { 0 };
                }
                Some(ArithToken::Ge) => {
                    *pos += 1;
                    let right = self.parse_shift(tokens, pos, shell)?;
                    left = if left >= right { 1 } else { 0 };
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_shift(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        let mut left = self.parse_additive(tokens, pos, shell)?;
        
        loop {
            match tokens.get(*pos) {
                Some(ArithToken::LShift) => {
                    *pos += 1;
                    let right = self.parse_additive(tokens, pos, shell)?;
                    left = left.wrapping_shl(right as u32);
                }
                Some(ArithToken::RShift) => {
                    *pos += 1;
                    let right = self.parse_additive(tokens, pos, shell)?;
                    left = left.wrapping_shr(right as u32);
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_additive(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        let mut left = self.parse_multiplicative(tokens, pos, shell)?;
        
        loop {
            match tokens.get(*pos) {
                Some(ArithToken::Plus) => {
                    *pos += 1;
                    let right = self.parse_multiplicative(tokens, pos, shell)?;
                    left = left.wrapping_add(right);
                }
                Some(ArithToken::Minus) => {
                    *pos += 1;
                    let right = self.parse_multiplicative(tokens, pos, shell)?;
                    left = left.wrapping_sub(right);
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_multiplicative(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        let mut left = self.parse_unary(tokens, pos, shell)?;
        
        loop {
            match tokens.get(*pos) {
                Some(ArithToken::Star) => {
                    *pos += 1;
                    let right = self.parse_unary(tokens, pos, shell)?;
                    left = left.wrapping_mul(right);
                }
                Some(ArithToken::Slash) => {
                    *pos += 1;
                    let right = self.parse_unary(tokens, pos, shell)?;
                    if right == 0 {
                        return Err("Division by zero".to_string());
                    }
                    left = left.wrapping_div(right);
                }
                Some(ArithToken::Percent) => {
                    *pos += 1;
                    let right = self.parse_unary(tokens, pos, shell)?;
                    if right == 0 {
                        return Err("Modulo by zero".to_string());
                    }
                    left = left.wrapping_rem(right);
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_unary(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        match tokens.get(*pos) {
            Some(ArithToken::Plus) => {
                *pos += 1;
                self.parse_unary(tokens, pos, shell)
            }
            Some(ArithToken::Minus) => {
                *pos += 1;
                let expr = self.parse_unary(tokens, pos, shell)?;
                Ok(-expr)
            }
            Some(ArithToken::Tilde) => {
                *pos += 1;
                let expr = self.parse_unary(tokens, pos, shell)?;
                Ok(!expr)
            }
            Some(ArithToken::Not) => {
                *pos += 1;
                let expr = self.parse_unary(tokens, pos, shell)?;
                Ok(if expr == 0 { 1 } else { 0 })
            }
            Some(ArithToken::Inc) => {
                *pos += 1;
                // Pre-increment - need variable
                Err("Pre-increment not yet implemented".to_string())
            }
            Some(ArithToken::Dec) => {
                *pos += 1;
                // Pre-decrement - need variable
                Err("Pre-decrement not yet implemented".to_string())
            }
            _ => self.parse_primary(tokens, pos, shell),
        }
    }
    
    fn parse_primary(&mut self, tokens: &[ArithToken], pos: &mut usize, shell: &Shell) -> Result<i64, String> {
        match tokens.get(*pos) {
            Some(ArithToken::Number(n)) => {
                *pos += 1;
                Ok(*n)
            }
            
            Some(ArithToken::Variable(name)) => {
                *pos += 1;
                // Look up variable value
                if let Some(value) = shell.env_vars.get(name) {
                    value.parse().map_err(|e| format!("Invalid integer value for variable {}: {}", name, e))
                } else {
                    // Unset variables evaluate to 0
                    Ok(0)
                }
            }
            
            Some(ArithToken::LParen) => {
                *pos += 1;
                let expr = self.parse_expression(tokens, pos, shell)?;
                if tokens.get(*pos) == Some(&ArithToken::RParen) {
                    *pos += 1;
                    Ok(expr)
                } else {
                    Err("Expected ')'".to_string())
                }
            }
            
            _ => Err("Expected number, variable, or '('".to_string()),
        }
    }
}

/// Expand arithmetic expression: $((expression))
pub fn expand_arithmetic(shell: &mut Shell, expr: &str) -> Result<String, String> {
    let mut evaluator = ArithmeticEvaluator::new();
    let result = evaluator.evaluate(expr, shell)?;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_arithmetic() {
        let mut shell = Shell::new();
        let mut evaluator = ArithmeticEvaluator::new();
        
        // Test simple expressions
        assert_eq!(evaluator.evaluate("1 + 2", &shell), Ok(3));
        assert_eq!(evaluator.evaluate("5 - 3", &shell), Ok(2));
        assert_eq!(evaluator.evaluate("2 * 3", &shell), Ok(6));
        assert_eq!(evaluator.evaluate("6 / 2", &shell), Ok(3));
        assert_eq!(evaluator.evaluate("7 % 3", &shell), Ok(1));
    }
    
    #[test]
    fn test_precedence() {
        let mut shell = Shell::new();
        let mut evaluator = ArithmeticEvaluator::new();
        
        assert_eq!(evaluator.evaluate("1 + 2 * 3", &shell), Ok(7));  // 1 + (2*3)
        assert_eq!(evaluator.evaluate("(1 + 2) * 3", &shell), Ok(9));
        assert_eq!(evaluator.evaluate("2 * 3 + 4", &shell), Ok(10)); // (2*3) + 4
    }
    
    #[test]
    fn test_bitwise_operations() {
        let mut shell = Shell::new();
        let mut evaluator = ArithmeticEvaluator::new();
        
        assert_eq!(evaluator.evaluate("5 & 3", &shell), Ok(1));  // 0101 & 0011 = 0001
        assert_eq!(evaluator.evaluate("5 | 2", &shell), Ok(7));  // 0101 | 0010 = 0111
        assert_eq!(evaluator.evaluate("5 ^ 3", &shell), Ok(6));  // 0101 ^ 0011 = 0110
        assert_eq!(evaluator.evaluate("~0", &shell), Ok(-1));    // ~0 = -1 (two's complement)
        assert_eq!(evaluator.evaluate("1 << 3", &shell), Ok(8)); // 1 << 3 = 8
        assert_eq!(evaluator.evaluate("8 >> 2", &shell), Ok(2)); // 8 >> 2 = 2
    }
    
    #[test]
    fn test_logical_operations() {
        let mut shell = Shell::new();
        let mut evaluator = ArithmeticEvaluator::new();
        
        assert_eq!(evaluator.evaluate("1 && 1", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("1 && 0", &shell), Ok(0));
        assert_eq!(evaluator.evaluate("0 || 1", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("0 || 0", &shell), Ok(0));
        assert_eq!(evaluator.evaluate("!0", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("!1", &shell), Ok(0));
    }
    
    #[test]
    fn test_comparisons() {
        let mut shell = Shell::new();
        let mut evaluator = ArithmeticEvaluator::new();
        
        assert_eq!(evaluator.evaluate("1 == 1", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("1 != 2", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("1 < 2", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("1 <= 1", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("2 > 1", &shell), Ok(1));
        assert_eq!(evaluator.evaluate("2 >= 2", &shell), Ok(1));
    }
    
    #[test]
    fn test_ternary_operator() {
        let mut shell = Shell::new();
        let mut evaluator = ArithmeticEvaluator::new();
        
        assert_eq!(evaluator.evaluate("1 ? 2 : 3", &shell), Ok(2));
        assert_eq!(evaluator.evaluate("0 ? 2 : 3", &shell), Ok(3));
    }
    
    #[test]
    fn test_number_bases() {
        let mut shell = Shell::new();
        let mut evaluator = ArithmeticEvaluator::new();
        
        assert_eq!(evaluator.evaluate("0x10", &shell), Ok(16));   // hex
        assert_eq!(evaluator.evaluate("010", &shell), Ok(8));     // octal
        assert_eq!(evaluator.evaluate("10", &shell), Ok(10));     // decimal
    }
    
    #[test]
    fn test_division_by_zero() {
        let mut shell = Shell::new();
        let mut evaluator = ArithmeticEvaluator::new();
        
        assert!(evaluator.evaluate("1 / 0", &shell).is_err());
        assert!(evaluator.evaluate("1 % 0", &shell).is_err());
    }
}