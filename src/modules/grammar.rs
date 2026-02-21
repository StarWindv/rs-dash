//! Grammar definitions for shell parsing

use crate::modules::tokens::Token;

/// Abstract syntax tree nodes
#[derive(Debug, Clone)]
pub enum ASTNode {
    /// Simple command: name and arguments
    SimpleCommand {
        name: String,
        args: Vec<String>,
        redirects: Vec<Redirect>,
    },
    
    /// Pipeline: command1 | command2 | ...
    Pipeline {
        commands: Vec<ASTNode>,
        negated: bool,  // ! prefix
    },
    
    /// List: command1 ; command2
    List {
        commands: Vec<ASTNode>,
        separators: Vec<char>,  // ;, &, &&, ||
    },
    
    /// If statement: if condition; then commands; fi
    IfStatement {
        condition: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        elif_branches: Vec<(ASTNode, ASTNode)>,
        else_branch: Option<Box<ASTNode>>,
    },
    
    /// For loop: for var in words; do commands; done
    ForLoop {
        variable: String,
        wordlist: Vec<String>,
        body: Box<ASTNode>,
    },
    
    /// While loop: while condition; do commands; done
    WhileLoop {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    
    /// Until loop: until condition; do commands; done
    UntilLoop {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    
    /// Case statement: case word in pattern) commands;; esac
    CaseStatement {
        word: String,
        patterns: Vec<(String, ASTNode)>,
    },
    
    /// Function definition: name() compound-command
    FunctionDefinition {
        name: String,
        body: Box<ASTNode>,
    },
    
    /// Subshell: (command)
    Subshell(Box<ASTNode>),
    
    /// Command grouping: { command; }
    CommandGroup(Box<ASTNode>),
    
    /// Empty command
    Empty,
}

/// Redirection types
#[derive(Debug, Clone)]
pub enum Redirect {
    /// Input redirection: < file
    Input {
        fd: i32,  // file descriptor, 0 for stdin
        filename: String,
        here_doc: Option<String>,  // for here-documents
    },
    
    /// Output redirection: > file
    Output {
        fd: i32,  // file descriptor, 1 for stdout
        filename: String,
        append: bool,  // >> for append
        clobber: bool, // >| for clobber
    },
    
    /// Duplicate file descriptor: >&2
    Dup {
        fd: i32,      // file descriptor to redirect
        target_fd: i32, // file descriptor to duplicate
    },
    
    /// Here-document: << EOF
    HereDocument {
        fd: i32,
        content: String,
        strip_tabs: bool,  // <<- for stripping leading tabs
    },
    
    /// Here-string: <<< string
    HereString {
        fd: i32,
        content: String,
    },
}

impl Redirect {
    pub fn new_input(fd: i32, filename: String) -> Self {
        Self::Input { fd, filename, here_doc: None }
    }
    
    pub fn new_output(fd: i32, filename: String, append: bool) -> Self {
        Self::Output { fd, filename, append, clobber: false }
    }
    
    pub fn new_dup(fd: i32, target_fd: i32) -> Self {
        Self::Dup { fd, target_fd }
    }
    
    pub fn new_heredoc(fd: i32, content: String, strip_tabs: bool) -> Self {
        Self::HereDocument { fd, content, strip_tabs }
    }
    
    pub fn new_herestring(fd: i32, content: String) -> Self {
        Self::HereString { fd, content }
    }
}

/// Grammar production rules
pub struct Grammar {
    // This will be expanded to include parsing rules
}

impl Grammar {
    pub fn new() -> Self {
        Self {}
    }
    
    /// Check if a token is a reserved word
    pub fn is_reserved_word(token: &Token) -> bool {
        use crate::modules::tokens::TokenType::*;
        
        match &token.token_type {
            If | Then | Else | Elif | Fi |
            Do | Done | Case | Esac |
            While | Until | For | In |
            Select | Bang | Time | Function => true,
            _ => false,
        }
    }
    
    /// Check if a token is a control operator
    pub fn is_control_operator(token: &Token) -> bool {
        use crate::modules::tokens::TokenType::*;
        
        match &token.token_type {
            Semicolon | Ampersand | Pipe | AndIf | OrIf | DSemi => true,
            _ => false,
        }
    }
    
    /// Check if a token is a redirection operator
    pub fn is_redirection_operator(token: &Token) -> bool {
        use crate::modules::tokens::TokenType::*;
        
        match &token.token_type {
            Less | Great | DLess | DGreat | LessAnd |
            GreatAnd | LessGreat | DLessDash | Clobber |
            RedirectIn | RedirectOut | RedirectAppend | RedirectHere => true,
            _ => false,
        }
    }
}