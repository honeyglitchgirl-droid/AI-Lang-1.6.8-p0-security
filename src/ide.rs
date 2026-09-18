//! IDE / LSP support helpers (diagnostics, completions, hover).

use crate::ast::Program;
use crate::error::CompileError;
use crate::lexer::lex;
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub line: usize,
    pub col: usize,
    pub severity: Severity,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Run parse + basic checks and return diagnostics for an editor.
pub fn diagnose(source: &str) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    match lex(source) {
        Ok(tokens) => match Parser::new(tokens).parse() {
            Ok(_prog) => {}
            Err(e) => diags.push(Diagnostic {
                line: 1,
                col: 1,
                severity: Severity::Error,
                message: e.to_string(),
            }),
        },
        Err(e) => diags.push(Diagnostic {
            line: e.line,
            col: e.col,
            severity: Severity::Error,
            message: e.message,
        }),
    }
    diags
}

/// Keyword + intrinsic completion list.
pub fn completions(_prefix: &str) -> Vec<&'static str> {
    vec![
        "let", "print", "if", "else", "while", "for", "break", "continue",
        "import", "grad", "dual", "primal", "tangent",
        "list_len", "list_push", "aether", "damp", "pulse", "softmax_naive",
        "backward_pass", "gradient_update",
    ]
}

/// Simple hover text for known keywords.
pub fn hover(word: &str) -> Option<&'static str> {
    match word {
        "let" => Some("let name = expr;  — bind a value"),
        "print" => Some("print expr;  — write to stdout"),
        "dual" => Some("dual(x)  — forward-mode dual number"),
        "grad" => Some("let w = 0.0 grad;  — mark parameter for reverse AD"),
        _ => None,
    }
}
