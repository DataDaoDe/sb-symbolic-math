use serde::{Deserialize, Serialize};
use socrates_math_algebra::{LinearExpression, LinearNormalizer};
use socrates_math_core::{
    ExactRational, Judgment, MathematicalOutcome, Relation, Unknown, UnknownReason, Verified,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SolveResult {
    pub variable: String,
    pub solution_set: SolutionSet,
    pub completeness: CompletenessStatus,
    pub method: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SolutionSet {
    Empty,
    Unique(ExactRational),
    AllRationals,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CompletenessStatus {
    Complete,
    CompleteUnderConditions,
    SoundButPossiblyIncomplete,
    HeuristicCandidates,
    Unknown,
}

pub struct LinearEquationSolver;

impl LinearEquationSolver {
    pub fn solve(judgment: &Judgment, variable: &str) -> MathematicalOutcome<SolveResult> {
        if judgment.relation != Relation::equality() {
            return MathematicalOutcome::Unknown(Unknown {
                reason: UnknownReason::UnsupportedDomain,
            });
        }

        let left = match LinearNormalizer::normalize(&judgment.left, variable) {
            MathematicalOutcome::Proven(result) => result.value.normal_form,
            MathematicalOutcome::Unknown(unknown) => return MathematicalOutcome::Unknown(unknown),
            _ => {
                return MathematicalOutcome::Unknown(Unknown {
                    reason: UnknownReason::UnsupportedDomain,
                });
            }
        };

        let right = match LinearNormalizer::normalize(&judgment.right, variable) {
            MathematicalOutcome::Proven(result) => result.value.normal_form,
            MathematicalOutcome::Unknown(unknown) => return MathematicalOutcome::Unknown(unknown),
            _ => {
                return MathematicalOutcome::Unknown(Unknown {
                    reason: UnknownReason::UnsupportedDomain,
                });
            }
        };

        MathematicalOutcome::Proven(Verified {
            value: solve_normalized(left, right, variable),
            evidence_id: Some("algebra.linear-equation.solve.rational".to_owned()),
        })
    }
}

fn solve_normalized(
    left: LinearExpression,
    right: LinearExpression,
    variable: &str,
) -> SolveResult {
    let coefficient_delta = left.coefficient.sub(&right.coefficient);
    let constant_delta = right.constant.sub(&left.constant);

    let solution_set = if coefficient_delta.is_zero() {
        if left.constant == right.constant {
            SolutionSet::AllRationals
        } else {
            SolutionSet::Empty
        }
    } else {
        SolutionSet::Unique(
            constant_delta
                .div(&coefficient_delta)
                .expect("nonzero coefficient delta is safe to divide by"),
        )
    };

    SolveResult {
        variable: variable.to_owned(),
        solution_set,
        completeness: CompletenessStatus::Complete,
        method: "algebra.linear-equation.solve.rational".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use socrates_math_core::{Context, Declaration, TypeId};
    use socrates_math_elab::{ElaborationOutcome, Elaborator};
    use socrates_math_syntax::{ParseOutcome, Parser};

    fn rational_context() -> Context {
        Context::root().with_declaration(Declaration {
            name: "x".to_owned(),
            type_id: TypeId::new("core.rational.rational").unwrap(),
        })
    }

    fn elaborated_statement(source: &str) -> Judgment {
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
    fn solves_linear_equation_with_integer_solution() {
        let judgment = elaborated_statement("3(x - 2) + 4 = 2x + 9");

        let MathematicalOutcome::Proven(result) = LinearEquationSolver::solve(&judgment, "x")
        else {
            panic!("expected proven solve result");
        };

        assert_eq!(
            result.value.solution_set,
            SolutionSet::Unique(ExactRational::integer(11))
        );
        assert_eq!(result.value.completeness, CompletenessStatus::Complete);
    }

    #[test]
    fn solves_linear_equation_with_rational_solution() {
        let judgment = elaborated_statement("2x = 1");

        let MathematicalOutcome::Proven(result) = LinearEquationSolver::solve(&judgment, "x")
        else {
            panic!("expected proven solve result");
        };

        assert_eq!(
            result.value.solution_set,
            SolutionSet::Unique(ExactRational::parse_fraction("1", "2").unwrap())
        );
    }

    #[test]
    fn solves_linear_equation_with_negative_rational_solution() {
        let judgment = elaborated_statement("3x = -2");

        let MathematicalOutcome::Proven(result) = LinearEquationSolver::solve(&judgment, "x")
        else {
            panic!("expected proven solve result");
        };

        assert_eq!(
            result.value.solution_set,
            SolutionSet::Unique(ExactRational::parse_fraction("-2", "3").unwrap())
        );
    }

    #[test]
    fn returns_empty_solution_set() {
        let judgment = elaborated_statement("2x + 1 = 2x + 3");

        let MathematicalOutcome::Proven(result) = LinearEquationSolver::solve(&judgment, "x")
        else {
            panic!("expected proven solve result");
        };

        assert_eq!(result.value.solution_set, SolutionSet::Empty);
        assert_eq!(result.value.completeness, CompletenessStatus::Complete);
    }

    #[test]
    fn returns_all_rationals_solution_set() {
        let judgment = elaborated_statement("2(x + 1) = 2x + 2");

        let MathematicalOutcome::Proven(result) = LinearEquationSolver::solve(&judgment, "x")
        else {
            panic!("expected proven solve result");
        };

        assert_eq!(result.value.solution_set, SolutionSet::AllRationals);
        assert_eq!(result.value.completeness, CompletenessStatus::Complete);
    }

    #[test]
    fn reports_unsupported_nonlinear_equation_as_unknown() {
        let judgment = elaborated_statement("x x = 2");

        let MathematicalOutcome::Unknown(unknown) = LinearEquationSolver::solve(&judgment, "x")
        else {
            panic!("expected unknown result");
        };

        assert_eq!(unknown.reason, UnknownReason::UnsupportedDomain);
    }
}
