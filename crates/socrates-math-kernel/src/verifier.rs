use crate::proof::{ProofNode, ProofRule, RationalOperation, VerificationError};
use socrates_math_core::{ExactRational, Judgment, Relation, SemanticTerm};

pub struct VerificationKernel;

impl VerificationKernel {
    pub fn verify(node: &ProofNode) -> Result<(), VerificationError> {
        match &node.rule {
            ProofRule::EqualityReflexive => verify_reflexive(&node.conclusion),
            ProofRule::EqualitySymmetric { premise } => verify_symmetric(premise, &node.conclusion),
            ProofRule::EqualityTransitive {
                left_premise,
                right_premise,
            } => verify_transitive(left_premise, right_premise, &node.conclusion),
            ProofRule::RationalArithmetic {
                operation,
                left,
                right,
            } => verify_rational_arithmetic(*operation, left, right, &node.conclusion),
        }
    }
}

fn verify_reflexive(conclusion: &Judgment) -> Result<(), VerificationError> {
    ensure_equality(conclusion)?;

    if conclusion.left == conclusion.right {
        Ok(())
    } else {
        Err(VerificationError::ConclusionDoesNotMatchRule {
            rule_id: ProofRule::EqualityReflexive.id(),
        })
    }
}

fn verify_symmetric(premise: &Judgment, conclusion: &Judgment) -> Result<(), VerificationError> {
    ensure_equality(premise)?;
    ensure_equality(conclusion)?;

    if conclusion.left == premise.right && conclusion.right == premise.left {
        Ok(())
    } else {
        Err(VerificationError::ConclusionDoesNotMatchRule {
            rule_id: ProofRule::EqualitySymmetric {
                premise: premise.clone(),
            }
            .id(),
        })
    }
}

fn verify_transitive(
    left_premise: &Judgment,
    right_premise: &Judgment,
    conclusion: &Judgment,
) -> Result<(), VerificationError> {
    ensure_equality(left_premise)?;
    ensure_equality(right_premise)?;
    ensure_equality(conclusion)?;

    if left_premise.right != right_premise.left {
        return Err(VerificationError::IncompatibleTransitivePremises);
    }

    if conclusion.left == left_premise.left && conclusion.right == right_premise.right {
        Ok(())
    } else {
        Err(VerificationError::ConclusionDoesNotMatchRule {
            rule_id: ProofRule::EqualityTransitive {
                left_premise: left_premise.clone(),
                right_premise: right_premise.clone(),
            }
            .id(),
        })
    }
}

fn verify_rational_arithmetic(
    operation: RationalOperation,
    left: &ExactRational,
    right: &ExactRational,
    conclusion: &Judgment,
) -> Result<(), VerificationError> {
    ensure_equality(conclusion)?;

    let result = match operation {
        RationalOperation::Add => left.add(right),
        RationalOperation::Subtract => left.sub(right),
        RationalOperation::Multiply => left.mul(right),
        RationalOperation::Divide => left
            .div(right)
            .map_err(|_| VerificationError::DivisionByZero)?,
    };

    let expected_left = match operation {
        RationalOperation::Add => arithmetic_term("core.rational.add", left, right),
        RationalOperation::Subtract => arithmetic_term("core.rational.sub", left, right),
        RationalOperation::Multiply => arithmetic_term("core.rational.mul", left, right),
        RationalOperation::Divide => arithmetic_term("core.rational.div", left, right),
    };
    let expected_right = SemanticTerm::rational(result);
    let expected = Judgment {
        left: expected_left,
        relation: Relation::equality(),
        right: expected_right,
    };

    if *conclusion == expected {
        Ok(())
    } else {
        Err(VerificationError::ConclusionDoesNotMatchRule {
            rule_id: ProofRule::RationalArithmetic {
                operation,
                left: left.clone(),
                right: right.clone(),
            }
            .id(),
        })
    }
}

fn arithmetic_term(symbol: &str, left: &ExactRational, right: &ExactRational) -> SemanticTerm {
    SemanticTerm::apply(
        socrates_math_core::SymbolId::new(symbol).expect("static symbol id is valid"),
        vec![
            SemanticTerm::rational(left.clone()),
            SemanticTerm::rational(right.clone()),
        ],
        socrates_math_core::TypeRef::rational(),
    )
}

fn ensure_equality(judgment: &Judgment) -> Result<(), VerificationError> {
    if judgment.relation == Relation::equality() {
        Ok(())
    } else {
        Err(VerificationError::UnsupportedRelation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proof::{ProofNodeId, RationalOperation};
    use socrates_math_core::ExactRational;

    trait ProofTermHelpers {
        fn rational_judgment(
            operation: RationalOperation,
            left: ExactRational,
            right: ExactRational,
            result: ExactRational,
        ) -> Judgment;
    }

    impl ProofTermHelpers for ExactRational {
        fn rational_judgment(
            operation: RationalOperation,
            left: ExactRational,
            right: ExactRational,
            result: ExactRational,
        ) -> Judgment {
            let symbol = match operation {
                RationalOperation::Add => "core.rational.add",
                RationalOperation::Subtract => "core.rational.sub",
                RationalOperation::Multiply => "core.rational.mul",
                RationalOperation::Divide => "core.rational.div",
            };

            Judgment {
                left: arithmetic_term(symbol, &left, &right),
                relation: Relation::equality(),
                right: SemanticTerm::rational(result),
            }
        }
    }

    #[test]
    fn verifies_exact_rational_addition() {
        let half = ExactRational::parse_fraction("1", "2").unwrap();
        let third = ExactRational::parse_fraction("1", "3").unwrap();
        let five_sixths = ExactRational::parse_fraction("5", "6").unwrap();
        let node = ProofNode {
            id: ProofNodeId("p1".to_owned()),
            rule: ProofRule::RationalArithmetic {
                operation: RationalOperation::Add,
                left: half.clone(),
                right: third.clone(),
            },
            conclusion: ExactRational::rational_judgment(
                RationalOperation::Add,
                half,
                third,
                five_sixths,
            ),
        };

        assert_eq!(VerificationKernel::verify(&node), Ok(()));
    }

    #[test]
    fn rejects_tampered_rational_addition_conclusion() {
        let half = ExactRational::parse_fraction("1", "2").unwrap();
        let third = ExactRational::parse_fraction("1", "3").unwrap();
        let wrong = ExactRational::parse_fraction("7", "6").unwrap();
        let node = ProofNode {
            id: ProofNodeId("p1".to_owned()),
            rule: ProofRule::RationalArithmetic {
                operation: RationalOperation::Add,
                left: half.clone(),
                right: third.clone(),
            },
            conclusion: ExactRational::rational_judgment(
                RationalOperation::Add,
                half,
                third,
                wrong,
            ),
        };

        assert!(matches!(
            VerificationKernel::verify(&node),
            Err(VerificationError::ConclusionDoesNotMatchRule { .. })
        ));
    }

    #[test]
    fn rejects_division_by_zero() {
        let one = ExactRational::integer(1);
        let zero = ExactRational::integer(0);
        let node = ProofNode {
            id: ProofNodeId("p1".to_owned()),
            rule: ProofRule::RationalArithmetic {
                operation: RationalOperation::Divide,
                left: one.clone(),
                right: zero.clone(),
            },
            conclusion: ExactRational::rational_judgment(
                RationalOperation::Divide,
                one,
                zero,
                ExactRational::integer(0),
            ),
        };

        assert_eq!(
            VerificationKernel::verify(&node),
            Err(VerificationError::DivisionByZero)
        );
    }

    #[test]
    fn verifies_equality_transitivity() {
        let a = SemanticTerm::variable("a", socrates_math_core::TypeRef::rational());
        let b = SemanticTerm::variable("b", socrates_math_core::TypeRef::rational());
        let c = SemanticTerm::variable("c", socrates_math_core::TypeRef::rational());
        let left = Judgment {
            left: a.clone(),
            relation: Relation::equality(),
            right: b.clone(),
        };
        let right = Judgment {
            left: b,
            relation: Relation::equality(),
            right: c.clone(),
        };
        let node = ProofNode {
            id: ProofNodeId("p2".to_owned()),
            rule: ProofRule::EqualityTransitive {
                left_premise: left.clone(),
                right_premise: right.clone(),
            },
            conclusion: Judgment {
                left: a,
                relation: Relation::equality(),
                right: c,
            },
        };

        assert_eq!(VerificationKernel::verify(&node), Ok(()));
    }
}
