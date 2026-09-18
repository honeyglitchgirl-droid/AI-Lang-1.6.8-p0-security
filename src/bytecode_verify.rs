//! Pre-execution bytecode verification.

use crate::error::VmError;
use crate::opcodes::OpCode;

#[derive(Debug, Clone)]
pub struct VerifiedBytecode {
    pub code: Vec<u8>,
    pub boundaries: Vec<usize>,
}

impl VerifiedBytecode {
    pub fn verify(code: Vec<u8>) -> Result<Self, VmError> {
        let mut boundaries = Vec::new();
        let mut control_targets: Vec<(usize, u64)> = Vec::new();
        let mut i = 0usize;
        while i < code.len() {
            boundaries.push(i);
            let byte = code[i];
            let op = OpCode::decode(byte).ok_or(VmError::UnknownOpCode { pc: i, byte })?;
            let start = i;
            i += 1;
            let extra = op.operand_size();
            if i + extra > code.len() {
                return Err(VmError::UnexpectedEnd { pc: i });
            }
            if extra == 8 && matches!(op, OpCode::Jump | OpCode::JumpIfFalse | OpCode::Call) {
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&code[i..i + 8]);
                let target = u64::from_le_bytes(buf);
                control_targets.push((start, target));
            }
            i += extra;
        }
        let boundary_set: std::collections::HashSet<usize> = boundaries.iter().copied().collect();
        for (pc, target) in control_targets {
            let t = target as usize;
            if target > usize::MAX as u64 || t >= code.len() {
                return Err(VmError::InvalidCallTarget { pc, target });
            }
            if !boundary_set.contains(&t) {
                return Err(VmError::InvalidInstructionBoundary { pc, target });
            }
        }
        Ok(Self { code, boundaries })
    }
}
