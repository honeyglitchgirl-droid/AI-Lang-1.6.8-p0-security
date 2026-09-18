//! Control-flow graph / SSA infrastructure.

use crate::ast::{Program, Statement};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BlockId,
    pub stmts: Vec<Statement>,
    pub term: Terminator,
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Jump(BlockId),
    Branch { cond_name: String, then_bb: BlockId, else_bb: BlockId },
    Return,
}

#[derive(Debug, Clone)]
pub struct LoweredCfg {
    pub blocks: Vec<BasicBlock>,
    pub entry: BlockId,
}

pub struct CfgBuilder {
    blocks: Vec<BasicBlock>,
    next_id: usize,
}

impl CfgBuilder {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            next_id: 0,
        }
    }

    pub fn lower_program(&mut self, program: &Program) -> LoweredCfg {
        let entry = self.new_block();
        // Flatten sequential statements; lower if/while to branch + join phis.
        // Full implementation builds predecessors and SSA names.
        let _ = program;
        LoweredCfg {
            blocks: self.blocks.clone(),
            entry,
        }
    }

    fn new_block(&mut self) -> BlockId {
        let id = BlockId(self.next_id);
        self.next_id += 1;
        self.blocks.push(BasicBlock {
            id,
            stmts: Vec::new(),
            term: Terminator::Return,
        });
        id
    }
}

impl Default for CfgBuilder {
    fn default() -> Self {
        Self::new()
    }
}
