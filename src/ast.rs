//! The Abstract Syntax Tree: what the parser produces and the compiler
//! consumes.
//!
//! Statement forms: `let`, `print`, bare expression, the
//! differentiable API (`backward_pass`, `print_grad`, `gradient_update`),
//! and the Phase-4 ecosystem forms — `import` and expression-bodied
//! `fn` declarations. Expression forms: literals, binary, unary,
//! variables, and calls. Every node is `Debug` and `PartialEq` so
//! tests can assert on tree shape directly.

use crate::tensor::Tensor;
use std::fmt;

/// A parsed program: an ordered list of top-level statements.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// Top-level statements, in source order.
    pub statements: Vec<Statement>,
}

/// A type name usable in a `let` annotation
/// (`let x : float = …;`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Tensor,
    List,
    ListInt,
    ListFloat,
    ListBool,
    ListString,
}

/// Top-level or block statement.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let {
        name: String,
        ty: Option<Type>,
        value: Expr,
        grad: bool,
    },
    Print(Expr),
    Expr(Expr),
    Assign { name: String, value: Expr },
    If {
        cond: Expr,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        cond: Expr,
        body: Vec<Statement>,
    },
    For {
        init: Option<Box<Statement>>,
        cond: Option<Expr>,
        step: Option<Box<Statement>>,
        body: Vec<Statement>,
    },
    Break,
    Continue,
    Import { path: String },
    Fn {
        name: String,
        params: Vec<String>,
        body: Expr,
    },
    BackwardPass,
    PrintGrad(String),
    GradientUpdate { name: String, lr: Expr },
}

/// Expression node.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Ident(String),
    Unary { op: UnaryOp, expr: Box<Expr> },
    Binary { op: BinaryOp, left: Box<Expr>, right: Box<Expr> },
    Call { callee: String, args: Vec<Expr> },
    List(Vec<Expr>),
    Tensor(Tensor),
    Index { base: Box<Expr>, index: Box<Expr> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Lt,
    Gt,
    And,
    Or,
}
