use serde::{Deserialize, Serialize};
use socrates_math_core::{
    ExactRational, ExactValueError, Judgment, MathematicalOutcome, Relation, SemanticTerm,
    Undefined, Unknown, UnknownReason, Verified,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    values: BTreeMap<String, ExactRational>,
}

impl Assignment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, name: impl Into<String>, value: ExactRational) -> Self {
        self.values.insert(name.into(), value);
        self
    }

    pub fn get(&self, name: &str) -> Option<&ExactRational> {
        self.values.get(name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExpressionEvaluation {
    ExactValue(ExactRational),
    Residual {
        term: SemanticTerm,
        unresolved_symbols: Vec<String>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StatementTruth {
    True,
    False,
}

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate_expression(
        term: &SemanticTerm,
        assignment: &Assignment,
    ) -> MathematicalOutcome<ExpressionEvaluation> {
        match evaluate_term(term, assignment) {
            Ok(value) => MathematicalOutcome::Proven(Verified {
                value: ExpressionEvaluation::ExactValue(value),
                evidence_id: Some("core.rational.evaluate.exact".to_owned()),
            }),
            Err(EvalError::Unresolved(symbols)) => MathematicalOutcome::Unknown(Unknown {
                reason: if symbols.is_empty() {
                    UnknownReason::NoApplicableMethod
                } else {
                    UnknownReason::InsufficientAssumptions
                },
            }),
            Err(EvalError::Undefined(reason)) => {
                MathematicalOutcome::Undefined(Undefined { reason })
            }
            Err(EvalError::Unsupported) => MathematicalOutcome::Unknown(Unknown {
                reason: UnknownReason::UnsupportedDomain,
            }),
        }
    }

    pub fn partially_evaluate_expression(
        term: &SemanticTerm,
        assignment: &Assignment,
    ) -> MathematicalOutcome<ExpressionEvaluation> {
        match partial_eval(term, assignment) {
            PartialEval::Exact(value) => MathematicalOutcome::Proven(Verified {
                value: ExpressionEvaluation::ExactValue(value),
                evidence_id: Some("core.rational.evaluate.partial".to_owned()),
            }),
            PartialEval::Residual {
                term,
                unresolved_symbols,
            } => MathematicalOutcome::Proven(Verified {
                value: ExpressionEvaluation::Residual {
                    term,
                    unresolved_symbols,
                },
                evidence_id: Some("core.rational.evaluate.partial".to_owned()),
            }),
            PartialEval::Undefined(reason) => MathematicalOutcome::Undefined(Undefined { reason }),
            PartialEval::Unsupported => MathematicalOutcome::Unknown(Unknown {
                reason: UnknownReason::UnsupportedDomain,
            }),
        }
    }

    pub fn evaluate_statement(
        judgment: &Judgment,
        assignment: &Assignment,
    ) -> MathematicalOutcome<StatementTruth> {
        if judgment.relation != Relation::equality() {
            return MathematicalOutcome::Unknown(Unknown {
                reason: UnknownReason::UnsupportedDomain,
            });
        }

        let left = match evaluate_term(&judgment.left, assignment) {
            Ok(value) => value,
            Err(EvalError::Undefined(reason)) => {
                return MathematicalOutcome::Undefined(Undefined { reason });
            }
            Err(EvalError::Unresolved(_)) => {
                return MathematicalOutcome::Unknown(Unknown {
                    reason: UnknownReason::InsufficientAssumptions,
                });
            }
            Err(EvalError::Unsupported) => {
                return MathematicalOutcome::Unknown(Unknown {
                    reason: UnknownReason::UnsupportedDomain,
                });
            }
        };

        let right = match evaluate_term(&judgment.right, assignment) {
            Ok(value) => value,
            Err(EvalError::Undefined(reason)) => {
                return MathematicalOutcome::Undefined(Undefined { reason });
            }
            Err(EvalError::Unresolved(_)) => {
                return MathematicalOutcome::Unknown(Unknown {
                    reason: UnknownReason::InsufficientAssumptions,
                });
            }
            Err(EvalError::Unsupported) => {
                return MathematicalOutcome::Unknown(Unknown {
                    reason: UnknownReason::UnsupportedDomain,
                });
            }
        };

        MathematicalOutcome::Proven(Verified {
            value: if left == right {
                StatementTruth::True
            } else {
                StatementTruth::False
            },
            evidence_id: Some("logic.equal.evaluate.pointwise".to_owned()),
        })
    }
}

#[derive(Debug)]
enum EvalError {
    Undefined(String),
    Unresolved(Vec<String>),
    Unsupported,
}

fn evaluate_term(term: &SemanticTerm, assignment: &Assignment) -> Result<ExactRational, EvalError> {
    match partial_eval(term, assignment) {
        PartialEval::Exact(value) => Ok(value),
        PartialEval::Residual {
            unresolved_symbols, ..
        } => Err(EvalError::Unresolved(unresolved_symbols)),
        PartialEval::Undefined(reason) => Err(EvalError::Undefined(reason)),
        PartialEval::Unsupported => Err(EvalError::Unsupported),
    }
}

enum PartialEval {
    Exact(ExactRational),
    Residual {
        term: SemanticTerm,
        unresolved_symbols: Vec<String>,
    },
    Undefined(String),
    Unsupported,
}

fn partial_eval(term: &SemanticTerm, assignment: &Assignment) -> PartialEval {
    match term {
        SemanticTerm::RationalLiteral(value) => PartialEval::Exact(value.clone()),
        SemanticTerm::LocalVariable { name, .. } => match assignment.get(name) {
            Some(value) => PartialEval::Exact(value.clone()),
            None => PartialEval::Residual {
                term: term.clone(),
                unresolved_symbols: vec![name.clone()],
            },
        },
        SemanticTerm::Apply {
            symbol,
            args,
            type_ref,
        } => {
            let mut evaluated_args = Vec::with_capacity(args.len());
            let mut unresolved = Vec::new();
            let mut all_exact = true;

            for arg in args {
                match partial_eval(arg, assignment) {
                    PartialEval::Exact(value) => {
                        evaluated_args.push(SemanticTerm::RationalLiteral(value));
                    }
                    PartialEval::Residual {
                        term,
                        unresolved_symbols,
                    } => {
                        all_exact = false;
                        evaluated_args.push(term);
                        merge_symbols(&mut unresolved, unresolved_symbols);
                    }
                    PartialEval::Undefined(reason) => return PartialEval::Undefined(reason),
                    PartialEval::Unsupported => return PartialEval::Unsupported,
                }
            }

            if !all_exact {
                return PartialEval::Residual {
                    term: SemanticTerm::Apply {
                        symbol: symbol.clone(),
                        args: evaluated_args,
                        type_ref: type_ref.clone(),
                    },
                    unresolved_symbols: unresolved,
                };
            }

            let values = evaluated_args
                .iter()
                .map(|arg| match arg {
                    SemanticTerm::RationalLiteral(value) => Some(value.clone()),
                    _ => None,
                })
                .collect::<Option<Vec<_>>>();

            let Some(values) = values else {
                return PartialEval::Unsupported;
            };

            match evaluate_rational_operation(symbol.as_str(), &values) {
                Ok(value) => PartialEval::Exact(value),
                Err(ExactValueError::DivisionByZero | ExactValueError::ZeroDenominator) => {
                    PartialEval::Undefined("division by zero".to_owned())
                }
                Err(ExactValueError::InvalidIntegerLiteral(_)) => PartialEval::Unsupported,
            }
        }
    }
}

fn evaluate_rational_operation(
    symbol: &str,
    values: &[ExactRational],
) -> Result<ExactRational, ExactValueError> {
    match (symbol, values) {
        ("core.rational.neg", [value]) => Ok(value.neg()),
        ("core.rational.add", [left, right]) => Ok(left.add(right)),
        ("core.rational.sub", [left, right]) => Ok(left.sub(right)),
        ("core.rational.mul", [left, right]) => Ok(left.mul(right)),
        ("core.rational.div", [left, right]) => left.div(right),
        _ => Err(ExactValueError::InvalidIntegerLiteral(symbol.to_owned())),
    }
}

fn merge_symbols(target: &mut Vec<String>, symbols: Vec<String>) {
    for symbol in symbols {
        if !target.contains(&symbol) {
            target.push(symbol);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use socrates_math_core::{Context, Declaration, TypeId};
    use socrates_math_elab::{ElaborationOutcome, Elaborator};
    use socrates_math_syntax::{ParseOutcome, Parser};

    fn rational_context() -> Context {
        Context::root()
            .with_declaration(Declaration {
                name: "x".to_owned(),
                type_id: TypeId::new("core.rational.rational").unwrap(),
            })
            .with_declaration(Declaration {
                name: "y".to_owned(),
                type_id: TypeId::new("core.rational.rational").unwrap(),
            })
    }

    fn expression(source: &str) -> SemanticTerm {
        let ParseOutcome::Parsed(expression) = Parser::parse_expression(source) else {
            panic!("expected parse success");
        };
        let ElaborationOutcome::Elaborated(term) =
            Elaborator::elaborate_expression(&expression, &rational_context())
        else {
            panic!("expected elaboration success");
        };
        term
    }

    fn statement(source: &str) -> Judgment {
        let ParseOutcome::Parsed(statement) = Parser::parse_statement(source) else {
            panic!("expected parse success");
        };
        let ElaborationOutcome::Elaborated(judgment) =
            Elaborator::elaborate_statement(&statement, &rational_context())
        else {
            panic!("expected elaboration success");
        };
        judgment
    }

    #[test]
    fn evaluates_expression_under_assignment() {
        let assignment = Assignment::new().with("x", ExactRational::integer(3));

        let MathematicalOutcome::Proven(result) =
            Evaluator::evaluate_expression(&expression("x + 2"), &assignment)
        else {
            panic!("expected proven evaluation");
        };

        assert_eq!(
            result.value,
            ExpressionEvaluation::ExactValue(ExactRational::integer(5))
        );
    }

    #[test]
    fn evaluates_rational_arithmetic_exactly() {
        let MathematicalOutcome::Proven(result) = Evaluator::evaluate_expression(
            &expression("\\frac{1}{2} + \\frac{1}{3}"),
            &Assignment::new(),
        ) else {
            panic!("expected proven evaluation");
        };

        assert_eq!(
            result.value,
            ExpressionEvaluation::ExactValue(ExactRational::parse_fraction("5", "6").unwrap())
        );
    }

    #[test]
    fn partially_evaluates_expression() {
        let assignment = Assignment::new().with("x", ExactRational::integer(3));

        let MathematicalOutcome::Proven(result) =
            Evaluator::partially_evaluate_expression(&expression("x + y + 2"), &assignment)
        else {
            panic!("expected proven partial evaluation");
        };

        let ExpressionEvaluation::Residual {
            unresolved_symbols, ..
        } = result.value
        else {
            panic!("expected residual");
        };

        assert_eq!(unresolved_symbols, vec!["y".to_owned()]);
    }

    #[test]
    fn evaluates_true_statement_pointwise() {
        let assignment = Assignment::new().with("x", ExactRational::integer(3));

        let MathematicalOutcome::Proven(result) =
            Evaluator::evaluate_statement(&statement("x + 2 = 5"), &assignment)
        else {
            panic!("expected proven statement evaluation");
        };

        assert_eq!(result.value, StatementTruth::True);
    }

    #[test]
    fn evaluates_false_statement_pointwise() {
        let assignment = Assignment::new().with("x", ExactRational::integer(3));

        let MathematicalOutcome::Proven(result) =
            Evaluator::evaluate_statement(&statement("x + 2 = 6"), &assignment)
        else {
            panic!("expected proven statement evaluation");
        };

        assert_eq!(result.value, StatementTruth::False);
    }

    #[test]
    fn reports_incomplete_statement_evaluation_as_unknown() {
        let assignment = Assignment::new().with("x", ExactRational::integer(3));

        let MathematicalOutcome::Unknown(unknown) =
            Evaluator::evaluate_statement(&statement("x + y = 6"), &assignment)
        else {
            panic!("expected unknown statement evaluation");
        };

        assert_eq!(unknown.reason, UnknownReason::InsufficientAssumptions);
    }
}
