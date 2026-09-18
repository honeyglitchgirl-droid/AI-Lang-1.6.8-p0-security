//! Symbol table and scope management.

use crate::ast::Type;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub ty: Option<Type>,
    pub slot: usize,
    pub is_grad: bool,
}

#[derive(Debug, Clone)]
pub struct Scope {
    symbols: HashMap<String, Symbol>,
    parent: Option<Box<Scope>>,
    next_slot: usize,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
            next_slot: 0,
        }
    }

    pub fn child(parent: Scope) -> Self {
        let next_slot = parent.next_slot;
        Self {
            symbols: HashMap::new(),
            parent: Some(Box::new(parent)),
            next_slot,
        }
    }

    pub fn define(&mut self, name: impl Into<String>, ty: Option<Type>, is_grad: bool) -> usize {
        let slot = self.next_slot;
        self.next_slot += 1;
        let name = name.into();
        self.symbols.insert(
            name.clone(),
            Symbol {
                name,
                ty,
                slot,
                is_grad,
            },
        );
        slot
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        if let Some(s) = self.symbols.get(name) {
            return Some(s);
        }
        self.parent.as_ref().and_then(|p| p.lookup(name))
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
