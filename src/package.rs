//! Package / module loading and import resolution.

use crate::error::CompileError;
use std::path::{Path, PathBuf};

pub struct PackageResolver {
    root: PathBuf,
}

impl PackageResolver {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Resolve an import path. Rejects `..` components for containment.
    pub fn resolve(&self, spec: &str) -> Result<PathBuf, CompileError> {
        if spec.contains("..") {
            return Err(CompileError::Other(format!("import path must not contain ..: {spec}")));
        }
        let path = self.root.join(spec);
        if !path.starts_with(&self.root) {
            return Err(CompileError::Other(format!("import escapes root: {spec}")));
        }
        Ok(path)
    }
}
