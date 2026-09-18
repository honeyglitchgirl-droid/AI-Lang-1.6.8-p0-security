//! Intermediate representation (graph / SSA-ish).
//!
//! Full implementation (~72 KB) contains NodeId, IrProgram, builders,
//! and visitation helpers used by optimizer and codegen.

use crate::ast::{BinaryOp, UnaryOp};
use crate::tensor::Tensor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

#[derive(Debug, Clone)]
pub enum IrNode {
    ConstInt(i64),
    ConstFloat(f64),
    ConstTensor(Tensor),
    LoadLocal(usize),
    Binary {
        op: BinaryOp,
        left: NodeId,
        right: NodeId,
    },
    Unary {
        op: UnaryOp,
        expr: NodeId,
    },
    Call {
        name: String,
        args: Vec<NodeId>,
    },
    // Additional variants in full source
}

#[derive(Debug, Clone, Default)]
pub struct IrProgram {
    pub nodes: Vec<IrNode>,
    pub roots: Vec<NodeId>,
}

impl IrProgram {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, node: IrNode) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(node);
        id
    }

    pub fn node(&self, id: NodeId) -> Option<&IrNode> {
        self.nodes.get(id.0)
    }
}
