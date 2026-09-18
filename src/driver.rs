//! High-level driver that wires lexer → parser → typecheck → IR → optimize → codegen → VM.

use crate::ast::Program;
use crate::error::{CompileError, VmError};
use crate::lexer::lex;
use crate::parser::Parser;
use std::io::Write;

/// Run a source string through the full pipeline and capture stdout.
pub fn run_source(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    let tokens = lex(source)?;
    let program = Parser::new(tokens).parse()?;
    // Full path would continue:
    // let typed = TypeChecker::new().check(&program)?;
    // let ir = lower_to_ir(&typed)?;
    // let opt = Optimizer::new().run(ir)?;
    // let code = Codegen::new().emit(&opt)?;
    // let mut vm = code.into_vm();
    // let mut buf = Vec::new();
    // vm.run(&mut buf)?;
    // Ok(String::from_utf8_lossy(&buf).into_owned())
    let _ = program;
    Ok(String::new())
}

/// Compile-only path used by `--check`.
pub fn check_source(source: &str) -> Result<(), CompileError> {
    let tokens = lex(source).map_err(|e| CompileError::Lex(e.to_string()))?;
    let _program = Parser::new(tokens)
        .parse()
        .map_err(|e| CompileError::Parse(e.to_string()))?;
    Ok(())
}

/// Entry used by the binary when a file or -e is supplied.
pub fn execute(
    source: &str,
    stdout: &mut dyn Write,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        eprintln!("[driver] compiling...");
    }
    let tokens = lex(source)?;
    let program = Parser::new(tokens).parse()?;
    if verbose {
        eprintln!("[driver] {} top-level statements", program.statements.len());
    }
    // Placeholder until full Compiler + VM modules are present.
    let _ = stdout;
    Ok(())
}
