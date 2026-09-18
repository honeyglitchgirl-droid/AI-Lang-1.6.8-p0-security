//! Bytecode virtual machine.
//!
//! Full implementation (~79 KB in original) contains the interpreter loop,
//! stack/local/list limits, jump validation, and host kernel dispatch.

use crate::bytecode_verify::VerifiedBytecode;
use crate::error::VmError;
use crate::opcodes::OpCode;
use crate::program::Program;
use crate::value::Value;
use std::io::Write;

const STACK_DEPTH_LIMIT: usize = 65_536;
const LOCAL_COUNT_LIMIT: usize = 65_536;
const STEP_BUDGET: u64 = 10_000_000;

#[derive(Debug)]
pub struct Vm {
    code: Vec<u8>,
    stack: Vec<Value>,
    locals: Vec<Value>,
    pc: usize,
    steps: u64,
}

#[derive(Debug)]
pub struct RunResult {
    pub steps: u64,
    pub stack: Vec<Value>,
    pub locals: Vec<Value>,
}

impl Vm {
    pub fn new(program: Program) -> Self {
        Self {
            code: program.code,
            stack: Vec::new(),
            locals: vec![Value::Nil; 256],
            pc: 0,
            steps: 0,
        }
    }

    pub fn run(&mut self, stdout: &mut dyn Write) -> Result<RunResult, VmError> {
        // Verify once at entry
        let verified = VerifiedBytecode::verify(self.code.clone())?;
        self.code = verified.code;

        while self.pc < self.code.len() {
            if self.steps >= STEP_BUDGET {
                return Err(VmError::StepBudgetExceeded);
            }
            self.steps += 1;

            let byte = self.code[self.pc];
            let op = OpCode::decode(byte).ok_or(VmError::UnknownOpCode {
                pc: self.pc,
                byte,
            })?;
            self.pc += 1;

            match op {
                OpCode::Halt => break,
                OpCode::Print => {
                    if let Some(v) = self.stack.pop() {
                        writeln!(stdout, "{v}").ok();
                    } else {
                        return Err(VmError::StackUnderflow);
                    }
                }
                OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div => {
                    // Full arithmetic handling in original source
                    let _ = op;
                }
                _ => {
                    // Remaining ops implemented in the original 79 KB source
                }
            }
        }

        Ok(RunResult {
            steps: self.steps,
            stack: self.stack.clone(),
            locals: self.locals.clone(),
        })
    }
}
