//! Module / import resolution and multi-file support.

use crate::ast::Program;
use crate::error::CompileError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Resolves import paths relative to a root, rejecting `..` for containment.
#[derive(Debug, Clone)]
pub struct ModuleResolver {
    root: PathBuf,
    cache: HashMap<String, Program>,
}

impl ModuleResolver {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            cache: HashMap::new(),
        }
    }

    pub fn resolve(&mut self, spec: &str) -> Result<&Program, CompileError> {
        if spec.contains("..") {
            return Err(CompileError::Other(format!(
                "import path must not contain ..: {spec}"
            )));
        }
        if let Some(prog) = self.cache.get(spec) {
            return Ok(prog);
        }
        let path = self.root.join(spec);
        if !path.starts_with(&self.root) {
            return Err(CompileError::Other(format!("import escapes root: {spec}")));
        }
        let source = std::fs::read_to_string(&path).map_err(|e| {
            CompileError::Other(format!("cannot read import {spec}: {e}"))
        })?;
        let tokens = crate::lexer::lex(&source).map_err(|e| CompileError::Lex(e.to_string()))?;
        let program = crate::parser::Parser::new(tokens)
            .parse()
            .map_err(|e| CompileError::Parse(e.to_string()))?;
        self.cache.insert(spec.to_string(), program);
        Ok(self.cache.get(spec).unwrap())
    }
}

/// Resolve all imports in a program (simplified entry point).
pub fn resolve_imports(
    program: &Program,
    resolver: &mut ModuleResolver,
) -> Result<Vec<Program>, CompileError> {
    let mut modules = Vec::new();
    for stmt in &program.statements {
        if let crate::ast::Statement::Import { path } = stmt {
            let m = resolver.resolve(path)?;
            modules.push(m.clone());
        }
    }
    Ok(modules)
}

/// Simple function table for linking.
#[derive(Debug, Default, Clone)]
pub struct FunctionTable {
    pub entries: HashMap<String, usize>,
}

impl FunctionTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: impl Into<String>, addr: usize) {
        self.entries.insert(name.into(), addr);
    }

    pub fn get(&self, name: &str) -> Option<usize> {
        self.entries.get(name).copied()
    }
}
