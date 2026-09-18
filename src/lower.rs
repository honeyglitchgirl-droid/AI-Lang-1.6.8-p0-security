//! Lowering from AST / IR to bytecode.

use crate::ast::{Expr, Program, Statement};
use crate::error::CompileError;
use crate::program::Program as BytecodeProgram;

pub fn lower_source(source: &str) -> Result<BytecodeProgram, CompileError> {
    // Full pipeline is in compiler.rs; this is a convenience helper used by tests.
    let tokens = crate::lexer::lex(source).map_err(|e| CompileError::Lex(e.to_string()))?;
    let program = crate::parser::Parser::new(tokens).parse().map_err(|e| CompileError::Parse(e.to_string()))?;
    crate::compiler::Compiler::new().compile_program(&program)
}

pub fn listing(source: &str) -> String {
    lower_source(source)
        .map(|p| p.listing())
        .unwrap_or_else(|e| format!("error: {e}"))
}
