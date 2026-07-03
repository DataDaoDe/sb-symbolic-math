use crate::diagnostic::{ElaborationDiagnostic, ElaborationDiagnosticCode};
use socrates_math_core::{
    Context, ExactRational, Judgment, Relation, SemanticTerm, SymbolId, TypeRef,
};
use socrates_math_syntax::{
    BinaryOperator, ConcreteExpression, ConcreteExpressionKind, ConcreteStatement,
    ConcreteStatementKind, Span, UnaryOperator,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpectedKind {
    Expression,
    Statement,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ElaborationOutcome<T> {
    Elaborated(T),
    Rejected(ElaborationDiagnostic),
}

pub struct Elaborator;

impl Elaborator {
    pub fn elaborate_expression(
        expression: &ConcreteExpression,
        context: &Context,
    ) -> ElaborationOutcome<SemanticTerm> {
        match elaborate_expression(expression, context) {
            Ok(term) => ElaborationOutcome::Elaborated(term),
            Err(diagnostic) => ElaborationOutcome::Rejected(diagnostic),
        }
    }

    pub fn elaborate_statement(
        statement: &ConcreteStatement,
        context: &Context,
    ) -> ElaborationOutcome<Judgment> {
        match elaborate_statement(statement, context) {
            Ok(judgment) => ElaborationOutcome::Elaborated(judgment),
            Err(diagnostic) => ElaborationOutcome::Rejected(diagnostic),
        }
    }

    pub fn elaborate_with_expected_kind(
        statement: &ConcreteStatement,
        context: &Context,
        expected_kind: ExpectedKind,
    ) -> ElaborationOutcome<ElaboratedObject> {
        match expected_kind {
            ExpectedKind::Statement => match Self::elaborate_statement(statement, context) {
                ElaborationOutcome::Elaborated(judgment) => {
                    ElaborationOutcome::Elaborated(ElaboratedObject::Statement(judgment))
                }
                ElaborationOutcome::Rejected(diagnostic) => {
                    ElaborationOutcome::Rejected(diagnostic)
                }
            },
            ExpectedKind::Expression => ElaborationOutcome::Rejected(ElaborationDiagnostic::new(
                ElaborationDiagnosticCode::TypeMismatch,
                Some(statement.span),
                "statement syntax cannot be elaborated where an expression is expected",
            )),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ElaboratedObject {
    Expression(SemanticTerm),
    Statement(Judgment),
}

fn elaborate_statement(
    statement: &ConcreteStatement,
    context: &Context,
) -> Result<Judgment, ElaborationDiagnostic> {
    match &statement.kind {
        ConcreteStatementKind::Equality { left, right, .. } => {
            let left = elaborate_expression(left, context)?;
            let right = elaborate_expression(right, context)?;

            ensure_type(&left, &TypeRef::rational(), Some(statement.span))?;
            ensure_type(&right, &TypeRef::rational(), Some(statement.span))?;

            Ok(Judgment {
                left,
                relation: Relation::equality(),
                right,
            })
        }
    }
}

fn elaborate_expression(
    expression: &ConcreteExpression,
    context: &Context,
) -> Result<SemanticTerm, ElaborationDiagnostic> {
    match &expression.kind {
        ConcreteExpressionKind::IntegerLiteral { text } => {
            let rational = ExactRational::parse_integer(text).map_err(|_| {
                ElaborationDiagnostic::new(
                    ElaborationDiagnosticCode::UnsupportedSemanticForm,
                    Some(expression.span),
                    "invalid integer literal",
                )
            })?;
            Ok(SemanticTerm::rational(rational))
        }
        ConcreteExpressionKind::RationalLiteral {
            numerator,
            denominator,
            ..
        } => {
            let rational = ExactRational::parse_fraction(numerator, denominator).map_err(|_| {
                ElaborationDiagnostic::new(
                    ElaborationDiagnosticCode::UnsupportedSemanticForm,
                    Some(expression.span),
                    "invalid rational literal",
                )
            })?;
            Ok(SemanticTerm::rational(rational))
        }
        ConcreteExpressionKind::Identifier { text } => {
            let declaration = context.find_declaration(text).ok_or_else(|| {
                ElaborationDiagnostic::new(
                    ElaborationDiagnosticCode::UnknownSymbol,
                    Some(expression.span),
                    format!("unknown symbol '{text}'"),
                )
            })?;

            Ok(SemanticTerm::variable(
                text.clone(),
                TypeRef {
                    id: declaration.type_id.clone(),
                },
            ))
        }
        ConcreteExpressionKind::Unary {
            operator: UnaryOperator::Negate,
            operand,
            ..
        } => {
            let operand = elaborate_expression(operand, context)?;
            ensure_type(&operand, &TypeRef::rational(), Some(expression.span))?;
            Ok(SemanticTerm::apply(
                symbol("core.rational.neg"),
                vec![operand],
                TypeRef::rational(),
            ))
        }
        ConcreteExpressionKind::Binary {
            operator,
            left,
            right,
            ..
        } => {
            let left = elaborate_expression(left, context)?;
            let right = elaborate_expression(right, context)?;
            ensure_type(&left, &TypeRef::rational(), Some(expression.span))?;
            ensure_type(&right, &TypeRef::rational(), Some(expression.span))?;

            let symbol = match operator {
                BinaryOperator::Add => symbol("core.rational.add"),
                BinaryOperator::Subtract => symbol("core.rational.sub"),
                BinaryOperator::Multiply => symbol("core.rational.mul"),
                BinaryOperator::Power => symbol("core.rational.pow"),
            };

            Ok(SemanticTerm::apply(
                symbol,
                vec![left, right],
                TypeRef::rational(),
            ))
        }
        ConcreteExpressionKind::Group { inner, .. } => elaborate_expression(inner, context),
    }
}

fn ensure_type(
    term: &SemanticTerm,
    expected: &TypeRef,
    span: Option<Span>,
) -> Result<(), ElaborationDiagnostic> {
    let actual = term_type(term);

    if &actual == expected {
        Ok(())
    } else {
        Err(ElaborationDiagnostic::new(
            ElaborationDiagnosticCode::TypeMismatch,
            span,
            format!(
                "expected type '{}', found '{}'",
                expected.id.as_str(),
                actual.id.as_str()
            ),
        ))
    }
}

fn term_type(term: &SemanticTerm) -> TypeRef {
    match term {
        SemanticTerm::RationalLiteral(_) => TypeRef::rational(),
        SemanticTerm::LocalVariable { type_ref, .. } | SemanticTerm::Apply { type_ref, .. } => {
            type_ref.clone()
        }
    }
}

fn symbol(id: &str) -> SymbolId {
    SymbolId::new(id).expect("static symbol id is valid")
}

#[cfg(test)]
mod tests {
    use super::*;
    use socrates_math_core::{Declaration, TypeId};
    use socrates_math_syntax::{ParseOutcome, Parser};

    fn rational_context() -> Context {
        Context::root().with_declaration(Declaration {
            name: "x".to_owned(),
            type_id: TypeId::new("core.rational.rational").unwrap(),
        })
    }

    #[test]
    fn elaborates_linear_equation_as_typed_equality() {
        let ParseOutcome::Parsed(statement) = Parser::parse_statement("3(x-2)+4 = 2x+9") else {
            panic!("expected parse success");
        };

        let ElaborationOutcome::Elaborated(judgment) =
            Elaborator::elaborate_statement(&statement, &rational_context())
        else {
            panic!("expected elaboration success");
        };

        assert_eq!(judgment.relation, Relation::equality());
        assert!(matches!(judgment.left, SemanticTerm::Apply { .. }));
        assert!(matches!(judgment.right, SemanticTerm::Apply { .. }));
    }

    #[test]
    fn elaborates_fraction_as_exact_rational() {
        let ParseOutcome::Parsed(expression) = Parser::parse_expression("\\frac{1}{2}") else {
            panic!("expected parse success");
        };

        let ElaborationOutcome::Elaborated(term) =
            Elaborator::elaborate_expression(&expression, &rational_context())
        else {
            panic!("expected elaboration success");
        };

        assert_eq!(
            term,
            SemanticTerm::rational(ExactRational::parse_fraction("1", "2").unwrap())
        );
    }

    #[test]
    fn rejects_unknown_symbol() {
        let ParseOutcome::Parsed(expression) = Parser::parse_expression("y + 1") else {
            panic!("expected parse success");
        };

        let ElaborationOutcome::Rejected(diagnostic) =
            Elaborator::elaborate_expression(&expression, &rational_context())
        else {
            panic!("expected unknown symbol rejection");
        };

        assert_eq!(diagnostic.code, ElaborationDiagnosticCode::UnknownSymbol);
        assert_eq!(diagnostic.span, Some(Span::new(0, 1)));
    }

    #[test]
    fn rejects_statement_where_expression_expected() {
        let ParseOutcome::Parsed(statement) = Parser::parse_statement("x = 2") else {
            panic!("expected parse success");
        };

        let ElaborationOutcome::Rejected(diagnostic) = Elaborator::elaborate_with_expected_kind(
            &statement,
            &rational_context(),
            ExpectedKind::Expression,
        ) else {
            panic!("expected type mismatch");
        };

        assert_eq!(diagnostic.code, ElaborationDiagnosticCode::TypeMismatch);
    }

    #[test]
    fn uses_theory_qualified_operation_symbols() {
        let ParseOutcome::Parsed(expression) = Parser::parse_expression("x + 1") else {
            panic!("expected parse success");
        };

        let ElaborationOutcome::Elaborated(term) =
            Elaborator::elaborate_expression(&expression, &rational_context())
        else {
            panic!("expected elaboration success");
        };

        let SemanticTerm::Apply { symbol, .. } = term else {
            panic!("expected application");
        };

        assert_eq!(symbol.as_str(), "core.rational.add");
    }

    #[test]
    fn elaborates_power_with_theory_qualified_symbol() {
        let ParseOutcome::Parsed(expression) = Parser::parse_expression("x^3") else {
            panic!("expected parse success");
        };

        let ElaborationOutcome::Elaborated(term) =
            Elaborator::elaborate_expression(&expression, &rational_context())
        else {
            panic!("expected elaboration success");
        };

        let SemanticTerm::Apply { symbol, .. } = term else {
            panic!("expected application");
        };

        assert_eq!(symbol.as_str(), "core.rational.pow");
    }
}
