//! IR optimizer (CSE, constant folding, DCE, etc.).
//!
//! Full implementation (~46 KB).

use crate::ir::IrProgram;

pub struct Optimizer;

impl Optimizer {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self, ir: IrProgram) -> IrProgram {
        // Full passes in original source
        ir
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}
