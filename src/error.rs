//! Error types for compiler and VM.

use std::fmt;

#[derive(Debug, Clone)]
pub enum CompileError {
    Lex(String),
    Parse(String),
    Type(String),
    Codegen(String),
    Other(String),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::Lex(s) => write!(f, "lex error: {s}"),
            CompileError::Parse(s) => write!(f, "parse error: {s}"),
            CompileError::Type(s) => write!(f, "type error: {s}"),
            CompileError::Codegen(s) => write!(f, "codegen error: {s}"),
            CompileError::Other(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for CompileError {}

#[derive(Debug, Clone)]
pub enum VmError {
    UnknownOpCode { pc: usize, byte: u8 },
    UnexpectedEnd { pc: usize },
    InvalidCallTarget { pc: usize, target: u64 },
    InvalidInstructionBoundary { pc: usize, target: u64 },
    StackUnderflow,
    StackOverflow,
    LocalOutOfBounds,
    ListElementLimit,
    StepBudgetExceeded,
    TapeBudgetExceeded,
    StringPoolOutOfBounds,
    ShapeMismatch,
    Other(String),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for VmError {}
