use crate::cst::Span;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: Severity,
    pub span: Option<Span>,
    pub summary: String,
}

impl Diagnostic {
    pub fn error(code: DiagnosticCode, span: Option<Span>, summary: impl Into<String>) -> Self {
        Self {
            code,
            severity: Severity::Error,
            span,
            summary: summary.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DiagnosticCode {
    ExpectedExpression,
    ExpectedEquals,
    ExpectedIntegerLiteral,
    ExpectedRightBrace,
    ExpectedRightParenthesis,
    TrailingInput,
    UnsupportedCommand,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
}
