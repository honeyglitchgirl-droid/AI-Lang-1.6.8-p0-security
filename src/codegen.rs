//! Code generation from IR / AST to bytecode.
//!
//! Full implementation (~69 KB) lowers lets, control flow, calls, and tensors.

use crate::ast::Program;
use crate::error::CompileError;
use crate::program::Program as BytecodeProgram;

pub struct Codegen {
    // internal state in full source
}

impl Codegen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn emit(&mut self, _program: &Program) -> Result<BytecodeProgram, CompileError> {
        // Full lowering lives in the original source.
        Ok(BytecodeProgram {
            code: vec![0], // Halt
            constants: vec![],
            tensors: vec![],
            strings: vec![],
        })
    }
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}
