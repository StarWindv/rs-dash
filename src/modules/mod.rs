//! rs-dash modules

pub mod arithmetic;
pub mod builtins;
pub mod control;
pub mod expansion;
pub mod functions;
pub mod grammar;
pub mod param_expand;
pub mod parser;
pub mod pipeline;
pub mod process_substitution;
pub mod redirection;
pub mod shell;
pub mod subshell;
pub mod tokens;

#[cfg(test)]
mod control_tests;