use crate::cst::{
    BinaryOperator, ConcreteExpression, ConcreteExpressionKind, ConcreteStatement,
    ConcreteStatementKind, GroupDelimiterSpans, MultiplicationKind, Span, UnaryOperator,
};
use crate::diagnostic::{Diagnostic, DiagnosticCode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseOutcome<T> {
    Parsed(T),
    Incomplete(Diagnostic),
    Rejected(Diagnostic),
}

pub struct Parser<'source> {
    source: &'source str,
    cursor: usize,
}

impl<'source> Parser<'source> {
    pub fn parse_statement(source: &'source str) -> ParseOutcome<ConcreteStatement> {
        let mut parser = Self { source, cursor: 0 };
        parser.parse_statement_inner()
    }

    pub fn parse_expression(source: &'source str) -> ParseOutcome<ConcreteExpression> {
        let mut parser = Self { source, cursor: 0 };
        let expression = match parser.parse_expression_inner() {
            Ok(expression) => expression,
            Err(error) => return error,
        };

        parser.skip_whitespace();

        if parser.is_at_end() {
            ParseOutcome::Parsed(expression)
        } else {
            ParseOutcome::Rejected(Diagnostic::error(
                DiagnosticCode::TrailingInput,
                parser.current_span(),
                "unexpected input after expression",
            ))
        }
    }

    fn parse_statement_inner(&mut self) -> ParseOutcome<ConcreteStatement> {
        let left = match self.parse_expression_inner() {
            Ok(expression) => expression,
            Err(error) => return error.map_diagnostic(),
        };

        self.skip_whitespace();

        let equals_span = if self.consume_char('=') {
            Span::new(self.cursor - 1, self.cursor)
        } else {
            return ParseOutcome::Rejected(Diagnostic::error(
                DiagnosticCode::ExpectedEquals,
                self.current_span(),
                "expected '=' in statement",
            ));
        };

        let right = match self.parse_expression_inner() {
            Ok(expression) => expression,
            Err(error) => return error.map_diagnostic(),
        };

        self.skip_whitespace();

        if !self.is_at_end() {
            return ParseOutcome::Rejected(Diagnostic::error(
                DiagnosticCode::TrailingInput,
                self.current_span(),
                "unexpected input after statement",
            ));
        }

        let span = left.span.join(right.span);

        ParseOutcome::Parsed(ConcreteStatement {
            kind: ConcreteStatementKind::Equality {
                left,
                equals_span,
                right,
            },
            span,
        })
    }

    fn parse_expression_inner(
        &mut self,
    ) -> Result<ConcreteExpression, ParseOutcome<ConcreteExpression>> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<ConcreteExpression, ParseOutcome<ConcreteExpression>> {
        let mut left = self.parse_multiplicative()?;

        loop {
            self.skip_whitespace();

            let Some(operator) = self.peek_char() else {
                return Ok(left);
            };

            let binary_operator = match operator {
                '+' => BinaryOperator::Add,
                '-' => BinaryOperator::Subtract,
                _ => return Ok(left),
            };

            let operator_start = self.cursor;
            self.cursor += operator.len_utf8();
            let operator_span = Span::new(operator_start, self.cursor);
            let right = self.parse_multiplicative()?;
            let span = left.span.join(right.span);

            left = ConcreteExpression {
                kind: ConcreteExpressionKind::Binary {
                    operator: binary_operator,
                    operator_span,
                    left: Box::new(left),
                    right: Box::new(right),
                    multiplication_kind: None,
                },
                span,
            };
        }
    }

    fn parse_multiplicative(
        &mut self,
    ) -> Result<ConcreteExpression, ParseOutcome<ConcreteExpression>> {
        let mut left = self.parse_unary()?;

        loop {
            self.skip_whitespace();

            let Some(kind) = self.next_multiplication_kind() else {
                return Ok(left);
            };

            let operator_span = match kind {
                MultiplicationKind::ExplicitDot => {
                    let start = self.cursor;
                    self.cursor += "\\cdot".len();
                    Span::new(start, self.cursor)
                }
                MultiplicationKind::Juxtaposition => Span::new(left.span.end, self.cursor),
            };

            let right = self.parse_unary()?;
            let span = left.span.join(right.span);

            left = ConcreteExpression {
                kind: ConcreteExpressionKind::Binary {
                    operator: BinaryOperator::Multiply,
                    operator_span,
                    left: Box::new(left),
                    right: Box::new(right),
                    multiplication_kind: Some(kind),
                },
                span,
            };
        }
    }

    fn parse_unary(&mut self) -> Result<ConcreteExpression, ParseOutcome<ConcreteExpression>> {
        self.skip_whitespace();

        if self.consume_char('-') {
            let operator_span = Span::new(self.cursor - 1, self.cursor);
            let operand = self.parse_unary()?;
            let span = operator_span.join(operand.span);

            return Ok(ConcreteExpression {
                kind: ConcreteExpressionKind::Unary {
                    operator: UnaryOperator::Negate,
                    operator_span,
                    operand: Box::new(operand),
                },
                span,
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<ConcreteExpression, ParseOutcome<ConcreteExpression>> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Err(ParseOutcome::Incomplete(Diagnostic::error(
                DiagnosticCode::ExpectedExpression,
                self.current_span(),
                "expected expression",
            )));
        }

        if self.starts_with("\\frac") {
            return self.parse_fraction();
        }

        if self.starts_with("\\") {
            return Err(ParseOutcome::Rejected(Diagnostic::error(
                DiagnosticCode::UnsupportedCommand,
                self.command_span(),
                "unsupported LaTeX command",
            )));
        }

        if self.peek_char() == Some('(') {
            return self.parse_group();
        }

        if self.peek_char().is_some_and(|ch| ch.is_ascii_digit()) {
            return Ok(self.parse_integer());
        }

        if self.peek_char().is_some_and(is_identifier_start) {
            return Ok(self.parse_identifier());
        }

        Err(ParseOutcome::Rejected(Diagnostic::error(
            DiagnosticCode::ExpectedExpression,
            self.current_span(),
            "expected expression",
        )))
    }

    fn parse_fraction(&mut self) -> Result<ConcreteExpression, ParseOutcome<ConcreteExpression>> {
        let start = self.cursor;
        self.cursor += "\\frac".len();
        let command_span = Span::new(start, self.cursor);
        let (numerator, numerator_span) = self.parse_braced_integer()?;
        let (denominator, denominator_span) = self.parse_braced_integer()?;
        let span = Span::new(start, denominator_span.end + 1);

        Ok(ConcreteExpression {
            kind: ConcreteExpressionKind::RationalLiteral {
                numerator,
                numerator_span,
                denominator,
                denominator_span,
                command_span,
            },
            span,
        })
    }

    fn parse_braced_integer(&mut self) -> Result<(String, Span), ParseOutcome<ConcreteExpression>> {
        self.skip_whitespace();

        if !self.consume_char('{') {
            return Err(ParseOutcome::Rejected(Diagnostic::error(
                DiagnosticCode::ExpectedRightBrace,
                self.current_span(),
                "expected '{'",
            )));
        }

        self.skip_whitespace();
        let integer_start = self.cursor;

        if self.peek_char() == Some('-') {
            self.cursor += 1;
        }

        let digit_start = self.cursor;
        while self.peek_char().is_some_and(|ch| ch.is_ascii_digit()) {
            self.cursor += 1;
        }

        if self.cursor == digit_start {
            return Err(ParseOutcome::Rejected(Diagnostic::error(
                DiagnosticCode::ExpectedIntegerLiteral,
                Some(Span::new(integer_start, self.cursor)),
                "expected integer literal",
            )));
        }

        let integer_end = self.cursor;
        let text = self.source[integer_start..integer_end].to_owned();
        self.skip_whitespace();

        if !self.consume_char('}') {
            return Err(ParseOutcome::Incomplete(Diagnostic::error(
                DiagnosticCode::ExpectedRightBrace,
                self.current_span(),
                "expected '}'",
            )));
        }

        Ok((text, Span::new(integer_start, integer_end)))
    }

    fn parse_group(&mut self) -> Result<ConcreteExpression, ParseOutcome<ConcreteExpression>> {
        let open = Span::new(self.cursor, self.cursor + 1);
        self.cursor += 1;
        let inner = self.parse_expression_inner()?;
        self.skip_whitespace();

        if !self.consume_char(')') {
            return Err(ParseOutcome::Incomplete(Diagnostic::error(
                DiagnosticCode::ExpectedRightParenthesis,
                Some(open),
                "expected ')'",
            )));
        }

        let close = Span::new(self.cursor - 1, self.cursor);
        let span = Span::new(open.start, close.end);

        Ok(ConcreteExpression {
            kind: ConcreteExpressionKind::Group {
                delimiters: GroupDelimiterSpans { open, close },
                inner: Box::new(inner),
            },
            span,
        })
    }

    fn parse_integer(&mut self) -> ConcreteExpression {
        let start = self.cursor;

        while self.peek_char().is_some_and(|ch| ch.is_ascii_digit()) {
            self.cursor += 1;
        }

        let text = self.source[start..self.cursor].to_owned();

        ConcreteExpression {
            kind: ConcreteExpressionKind::IntegerLiteral { text },
            span: Span::new(start, self.cursor),
        }
    }

    fn parse_identifier(&mut self) -> ConcreteExpression {
        let start = self.cursor;

        while self.peek_char().is_some_and(is_identifier_continue) {
            self.cursor += 1;
        }

        let text = self.source[start..self.cursor].to_owned();

        ConcreteExpression {
            kind: ConcreteExpressionKind::Identifier { text },
            span: Span::new(start, self.cursor),
        }
    }

    fn next_multiplication_kind(&self) -> Option<MultiplicationKind> {
        if self.starts_with("\\cdot") {
            return Some(MultiplicationKind::ExplicitDot);
        }

        if self.peek_char().is_some_and(starts_primary) {
            return Some(MultiplicationKind::Juxtaposition);
        }

        if self.starts_with("\\frac") {
            return Some(MultiplicationKind::Juxtaposition);
        }

        None
    }

    fn skip_whitespace(&mut self) {
        while self.peek_char().is_some_and(char::is_whitespace) {
            self.cursor += self.peek_char().expect("peeked char exists").len_utf8();
        }
    }

    fn consume_char(&mut self, expected: char) -> bool {
        if self.peek_char() == Some(expected) {
            self.cursor += expected.len_utf8();
            true
        } else {
            false
        }
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.source.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.source[self.cursor..].chars().next()
    }

    fn starts_with(&self, text: &str) -> bool {
        self.source[self.cursor..].starts_with(text)
    }

    fn current_span(&self) -> Option<Span> {
        if self.cursor <= self.source.len() {
            Some(Span::new(self.cursor, self.cursor))
        } else {
            None
        }
    }

    fn command_span(&self) -> Option<Span> {
        if !self.starts_with("\\") {
            return self.current_span();
        }

        let start = self.cursor;
        let mut end = self.cursor + 1;

        for ch in self.source[end..].chars() {
            if !ch.is_ascii_alphabetic() {
                break;
            }
            end += ch.len_utf8();
        }

        Some(Span::new(start, end))
    }
}

impl<T> ParseOutcome<T> {
    fn map_diagnostic<U>(self) -> ParseOutcome<U> {
        match self {
            Self::Parsed(_) => unreachable!("successful parse outcomes are not diagnostics"),
            Self::Incomplete(diagnostic) => ParseOutcome::Incomplete(diagnostic),
            Self::Rejected(diagnostic) => ParseOutcome::Rejected(diagnostic),
        }
    }
}

fn starts_primary(ch: char) -> bool {
    ch == '(' || ch.is_ascii_digit() || is_identifier_start(ch)
}

fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cst::{ConcreteExpressionKind, ConcreteStatementKind};

    #[test]
    fn parses_linear_equation_with_spans() {
        let ParseOutcome::Parsed(statement) = Parser::parse_statement("3(x-2)+4 = 2x+9") else {
            panic!("expected parse success");
        };

        assert_eq!(statement.span, Span::new(0, 15));

        let ConcreteStatementKind::Equality {
            left,
            equals_span,
            right,
        } = statement.kind;

        assert_eq!(equals_span, Span::new(9, 10));
        assert_eq!(left.span, Span::new(0, 8));
        assert_eq!(right.span, Span::new(11, 15));
    }

    #[test]
    fn records_implicit_multiplication() {
        let ParseOutcome::Parsed(expression) = Parser::parse_expression("3(x-2)") else {
            panic!("expected parse success");
        };

        let ConcreteExpressionKind::Binary {
            operator,
            multiplication_kind,
            ..
        } = expression.kind
        else {
            panic!("expected binary expression");
        };

        assert_eq!(operator, BinaryOperator::Multiply);
        assert_eq!(multiplication_kind, Some(MultiplicationKind::Juxtaposition));
    }

    #[test]
    fn records_explicit_multiplication() {
        let ParseOutcome::Parsed(expression) = Parser::parse_expression("3\\cdot y") else {
            panic!("expected parse success");
        };

        let ConcreteExpressionKind::Binary {
            operator_span,
            multiplication_kind,
            ..
        } = expression.kind
        else {
            panic!("expected binary expression");
        };

        assert_eq!(operator_span, Span::new(1, 6));
        assert_eq!(multiplication_kind, Some(MultiplicationKind::ExplicitDot));
    }

    #[test]
    fn preserves_explicit_grouping() {
        let ParseOutcome::Parsed(expression) = Parser::parse_expression("3((x-2)+4)") else {
            panic!("expected parse success");
        };

        let ConcreteExpressionKind::Binary { right, .. } = expression.kind else {
            panic!("expected binary multiplication");
        };

        let ConcreteExpressionKind::Group { inner, delimiters } = right.kind else {
            panic!("expected outer group");
        };

        assert_eq!(delimiters.open, Span::new(1, 2));
        assert_eq!(delimiters.close, Span::new(9, 10));

        let ConcreteExpressionKind::Binary { left, .. } = inner.kind else {
            panic!("expected grouped addition");
        };

        assert!(matches!(left.kind, ConcreteExpressionKind::Group { .. }));
    }

    #[test]
    fn parses_rational_literal_with_operand_spans() {
        let ParseOutcome::Parsed(statement) = Parser::parse_statement("\\frac{1}{2}x + 3 = 4")
        else {
            panic!("expected parse success");
        };

        let ConcreteStatementKind::Equality { left, .. } = statement.kind;
        let ConcreteExpressionKind::Binary { left, .. } = left.kind else {
            panic!("expected left addition");
        };
        let ConcreteExpressionKind::Binary { left, .. } = left.kind else {
            panic!("expected implicit multiplication");
        };
        let ConcreteExpressionKind::RationalLiteral {
            numerator,
            numerator_span,
            denominator,
            denominator_span,
            ..
        } = left.kind
        else {
            panic!("expected rational literal");
        };

        assert_eq!(numerator, "1");
        assert_eq!(numerator_span, Span::new(6, 7));
        assert_eq!(denominator, "2");
        assert_eq!(denominator_span, Span::new(9, 10));
    }

    #[test]
    fn rejects_unsupported_command() {
        let ParseOutcome::Rejected(diagnostic) = Parser::parse_statement("\\sqrt{x} = 2") else {
            panic!("expected rejection");
        };

        assert_eq!(diagnostic.code, DiagnosticCode::UnsupportedCommand);
        assert_eq!(diagnostic.span, Some(Span::new(0, 5)));
    }

    #[test]
    fn reports_unclosed_delimiter_as_incomplete() {
        let ParseOutcome::Incomplete(diagnostic) = Parser::parse_statement("3(x-2 + 4") else {
            panic!("expected incomplete parse");
        };

        assert_eq!(diagnostic.code, DiagnosticCode::ExpectedRightParenthesis);
        assert_eq!(diagnostic.span, Some(Span::new(1, 2)));
    }

    #[test]
    fn rejects_malformed_fraction() {
        let ParseOutcome::Rejected(diagnostic) = Parser::parse_expression("\\frac{1}{}") else {
            panic!("expected rejection");
        };

        assert_eq!(diagnostic.code, DiagnosticCode::ExpectedIntegerLiteral);
    }
}
