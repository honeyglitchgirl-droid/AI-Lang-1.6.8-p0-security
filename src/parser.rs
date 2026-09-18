//! Recursive-descent / Pratt parser.
//!
//! Full implementation (~52 KB) handles expressions, statements, lists, tensors, control flow.

use crate::ast::{Program, Statement};
use crate::error::CompileError;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, CompileError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if matches!(self.peek(), Token::Eof) {
                break;
            }
            // Full statement parsing in original source
            statements.push(Statement::Expr(crate::ast::Expr::Int(0))); // placeholder
            break;
        }
        Ok(Program { statements })
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof) || self.pos >= self.tokens.len()
    }
}
