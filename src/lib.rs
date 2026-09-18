//! AI-Lang library crate.
//!
//! Public surface for embedding the compiler and VM.

pub mod ast;
pub mod autodiff;
pub mod bytecode_verify;
pub mod cfg;
pub mod cfg_driver;
pub mod codegen;
pub mod compiler;
pub mod driver;
pub mod error;
pub mod ide;
pub mod intrinsics;
pub mod ir;
pub mod kernel;
pub mod lexer;
pub mod linker;
pub mod lower;
pub mod module;
pub mod opcodes;
pub mod optimizer;
pub mod package;
pub mod parser;
pub mod program;
pub mod python_api;
pub mod runtime_ad;
pub mod symbol;
pub mod tensor;
pub mod token;
pub mod train;
pub mod typecheck;
pub mod value;
pub mod vm;

pub use compiler::Compiler;
pub use error::{CompileError, VmError};
pub use lexer::lex;
pub use parser::Parser;
pub use program::Program;
pub use vm::Vm;

/// Convenience: compile and run a source string, returning captured stdout.
pub fn run_source(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    let tokens = lex(source)?;
    let program = Parser::new(tokens).parse()?;
    let compiled = Compiler::new().compile_program(&program)?;
    let mut vm = compiled.into_vm();
    let mut buf = Vec::new();
    vm.run(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}
