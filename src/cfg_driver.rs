//! CFG driver / lower_program_to_cfg entry points.

use crate::ast::Program;
use crate::cfg::{CfgBuilder, LoweredCfg};

pub fn lower_program_to_cfg(program: &Program) -> LoweredCfg {
    let mut builder = CfgBuilder::new();
    // Flatten sequential statements into basic blocks with phis for mutations.
    // Implementation details in cfg.rs.
    builder.lower_program(program)
}
