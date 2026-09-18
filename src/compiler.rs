//! Top-level compiler facade.
//!
//! Full implementation (~63 KB) orchestrates parse → typecheck → IR → optimize → codegen.

use crate::ast::Program;
use crate::error::CompileError;
use crate::program::Program as BytecodeProgram;

pub struct Compiler {
    // options / state
}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile_program(&mut self, program: &Program) -> Result<BytecodeProgram, CompileError> {
        // Full pipeline in original source.
        let _ = program;
        Ok(BytecodeProgram {
            code: vec![0],
            constants: vec![],
            tensors: vec![],
            strings: vec![],
        })
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
