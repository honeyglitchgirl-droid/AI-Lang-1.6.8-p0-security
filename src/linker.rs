//! Linker: resolves call targets and builds the final executable image.

use crate::error::CompileError;
use crate::opcodes::OpCode;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LinkedProgram {
    pub code: Vec<u8>,
    pub entry: usize,
    pub symbols: HashMap<String, usize>,
}

pub struct Linker {
    code: Vec<u8>,
    symbols: HashMap<String, usize>,
    pending_calls: Vec<(usize, String)>, // (offset_of_operand, name)
}

impl Linker {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            symbols: HashMap::new(),
            pending_calls: Vec::new(),
        }
    }

    pub fn define(&mut self, name: impl Into<String>, addr: usize) {
        self.symbols.insert(name.into(), addr);
    }

    pub fn emit_op(&mut self, op: OpCode) {
        self.code.push(op as u8);
    }

    pub fn emit_u64(&mut self, v: u64) {
        self.code.extend_from_slice(&v.to_le_bytes());
    }

    pub fn emit_call(&mut self, name: &str) {
        self.emit_op(OpCode::Call);
        let offset = self.code.len();
        self.emit_u64(0); // placeholder
        self.pending_calls.push((offset, name.to_string()));
    }

    pub fn finish(mut self) -> Result<LinkedProgram, CompileError> {
        for (offset, name) in &self.pending_calls {
            let addr = self.symbols.get(name).ok_or_else(|| {
                CompileError::Other(format!("undefined function: {name}"))
            })?;
            let bytes = (*addr as u64).to_le_bytes();
            self.code[*offset..*offset + 8].copy_from_slice(&bytes);
        }
        Ok(LinkedProgram {
            code: self.code,
            entry: 0,
            symbols: self.symbols,
        })
    }
}

impl Default for Linker {
    fn default() -> Self {
        Self::new()
    }
}
