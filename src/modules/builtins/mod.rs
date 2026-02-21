//! Built-in commands implementation with registry pattern

use std::rc::Rc;
use crate::modules::shell::Shell;

mod registry;
mod cd;
mod pwd;
mod echo;
mod exit;
mod help;
mod r#true;
mod r#false;
mod r#return;
mod test;
mod bracket;

pub use registry::BuiltinRegistry;
pub use cd::CdBuiltin;
pub use pwd::PwdBuiltin;
pub use echo::EchoBuiltin;
pub use exit::ExitBuiltin;
pub use help::HelpBuiltin;
pub use r#true::TrueBuiltin;
pub use r#false::FalseBuiltin;
pub use r#return::ReturnBuiltin;
pub use test::TestBuiltin;
pub use bracket::BracketBuiltin;

/// Trait for builtin commands
pub trait Builtin: Send + Sync {
    /// Get the name of the builtin command
    fn name(&self) -> &'static str;
    
    /// Execute the builtin command
    fn execute(&self, shell: &mut Shell, args: &[String]) -> i32;
    
    /// Execute in pipeline context (default implementation handles regular execution)
    fn execute_in_pipeline(&self, shell: &mut Shell, args: &[String], _is_last: bool) -> i32 {
        // Default implementation: if last in pipeline, print output
        // For most builtins, this is the same as regular execution
        self.execute(shell, args)
    }
}

/// Create and initialize the builtin registry with all builtins
pub fn create_registry() -> Rc<BuiltinRegistry> {
    let mut registry = BuiltinRegistry::new();
    
    // Register all builtin commands
    registry.register(Box::new(CdBuiltin));
    registry.register(Box::new(PwdBuiltin));
    registry.register(Box::new(EchoBuiltin));
    registry.register(Box::new(ExitBuiltin));
    registry.register(Box::new(HelpBuiltin));
    registry.register(Box::new(TrueBuiltin));
    registry.register(Box::new(FalseBuiltin));
    registry.register(Box::new(ReturnBuiltin));
    registry.register(Box::new(TestBuiltin));
    registry.register(Box::new(BracketBuiltin));
    
    Rc::new(registry)
}