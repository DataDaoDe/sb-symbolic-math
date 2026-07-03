use serde::{Deserialize, Serialize};
use socrates_math_syntax::Span;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ElaborationDiagnostic {
    pub code: ElaborationDiagnosticCode,
    pub span: Option<Span>,
    pub summary: String,
}

impl ElaborationDiagnostic {
    pub fn new(
        code: ElaborationDiagnosticCode,
        span: Option<Span>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            code,
            span,
            summary: summary.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ElaborationDiagnosticCode {
    ExpectedStatement,
    TypeMismatch,
    UnknownSymbol,
    UnsupportedSemanticForm,
}
