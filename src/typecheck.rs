//! Type checker.
//!
//! Full implementation (~44 KB) walks the AST, tracks scopes, and checks shapes.

use crate::ast::Program;
use crate::error::CompileError;

pub struct TypeChecker;

impl TypeChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn check(&mut self, program: &Program) -> Result<(), CompileError> {
        let _ = program;
        // Full checking in original source
        Ok(())
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
