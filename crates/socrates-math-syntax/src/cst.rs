use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn join(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConcreteStatement {
    pub kind: ConcreteStatementKind,
    pub span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ConcreteStatementKind {
    Equality {
        left: ConcreteExpression,
        equals_span: Span,
        right: ConcreteExpression,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConcreteExpression {
    pub kind: ConcreteExpressionKind,
    pub span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ConcreteExpressionKind {
    IntegerLiteral {
        text: String,
    },
    RationalLiteral {
        numerator: String,
        numerator_span: Span,
        denominator: String,
        denominator_span: Span,
        command_span: Span,
    },
    Identifier {
        text: String,
    },
    Unary {
        operator: UnaryOperator,
        operator_span: Span,
        operand: Box<ConcreteExpression>,
    },
    Binary {
        operator: BinaryOperator,
        operator_span: Span,
        left: Box<ConcreteExpression>,
        right: Box<ConcreteExpression>,
        multiplication_kind: Option<MultiplicationKind>,
    },
    Group {
        delimiters: GroupDelimiterSpans,
        inner: Box<ConcreteExpression>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Negate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Power,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MultiplicationKind {
    ExplicitDot,
    Juxtaposition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GroupDelimiterSpans {
    pub open: Span,
    pub close: Span,
}
