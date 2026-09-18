//! Experimental runtime reverse-mode AD (tape).
//!
//! Compile-time / dual forward-mode is the supported path.
//! This module provides a soft-budget tape that fails closed.

use crate::error::VmError;

const TAPE_OP_LIMIT: usize = 1_000_000;

#[derive(Debug, Clone)]
enum TapeOp {
    Add,
    Sub,
    Mul,
    Div,
    // additional ops in full source
}

#[derive(Debug, Default)]
pub struct Tape {
    ops: Vec<TapeOp>,
    budget: usize,
}

impl Tape {
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            budget: TAPE_OP_LIMIT,
        }
    }

    pub fn record(&mut self, op: TapeOp) -> Result<(), VmError> {
        if self.ops.len() >= self.budget {
            return Err(VmError::TapeBudgetExceeded);
        }
        self.ops.push(op);
        Ok(())
    }

    pub fn clear(&mut self) {
        self.ops.clear();
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }
}

/// Run a backward pass over the recorded tape (placeholder).
pub fn backward_pass(_tape: &Tape) -> Result<(), VmError> {
    // Full implementation walks the tape in reverse and accumulates gradients.
    Ok(())
}
