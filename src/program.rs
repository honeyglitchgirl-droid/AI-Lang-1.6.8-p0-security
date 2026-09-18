//! Compiled bytecode program and builder.

use crate::opcodes::OpCode;
use crate::vm::Vm;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Program {
    pub code: Vec<u8>,
    pub constants: Vec<f64>,
    pub tensors: Vec<crate::tensor::Tensor>,
    pub strings: Vec<String>,
}

impl Program {
    pub fn builder() -> ProgramBuilder {
        ProgramBuilder::new()
    }

    pub fn listing(&self) -> String {
        // Simplified disassembler
        let mut out = Vec::new();
        let mut i = 0;
        while i < self.code.len() {
            let byte = self.code[i];
            if let Some(op) = OpCode::decode(byte) {
                out.push(format!("{op:?}"));
                i += 1 + op.operand_size();
            } else {
                out.push(format!("UNKNOWN({byte})"));
                i += 1;
            }
        }
        out.join(", ")
    }

    pub fn into_vm(self) -> Vm {
        Vm::new(self)
    }

    pub fn vm(&self) -> Vm {
        Vm::new(self.clone())
    }
}

pub struct ProgramBuilder {
    code: Vec<u8>,
    constants: Vec<f64>,
    tensors: Vec<crate::tensor::Tensor>,
    strings: Vec<String>,
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            tensors: Vec::new(),
            strings: Vec::new(),
        }
    }

    pub fn push(mut self, v: i64) -> Self {
        self.code.push(OpCode::Push as u8);
        self.code.extend_from_slice(&(v as u64).to_le_bytes());
        self
    }

    pub fn push_f64(mut self, v: f64) -> Self {
        self.code.push(OpCode::PushF64 as u8);
        self.code.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn add(mut self) -> Self {
        self.code.push(OpCode::Add as u8);
        self
    }

    pub fn sub(mut self) -> Self {
        self.code.push(OpCode::Sub as u8);
        self
    }

    pub fn mul(mut self) -> Self {
        self.code.push(OpCode::Mul as u8);
        self
    }

    pub fn print(mut self) -> Self {
        self.code.push(OpCode::Print as u8);
        self
    }

    pub fn halt(mut self) -> Self {
        self.code.push(OpCode::Halt as u8);
        self
    }

    pub fn build(self) -> Program {
        Program {
            code: self.code,
            constants: self.constants,
            tensors: self.tensors,
            strings: self.strings,
        }
    }
}

impl Default for ProgramBuilder {
    fn default() -> Self {
        Self::new()
    }
}
