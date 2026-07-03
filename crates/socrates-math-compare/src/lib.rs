use serde::{Deserialize, Serialize};
use socrates_math_core::{
    Disproven, Judgment, MathematicalOutcome, Unknown, UnknownReason, Verified,
};
use socrates_math_solve::{LinearEquationSolver, SolutionSet};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SolutionSetComparison {
    pub variable: String,
    pub left_solution_set: SolutionSet,
    pub right_solution_set: SolutionSet,
    pub relation: String,
    pub complete: bool,
}

pub struct EquationComparator;

impl EquationComparator {
    pub fn compare_solution_sets(
        left: &Judgment,
        right: &Judgment,
        variable: &str,
    ) -> MathematicalOutcome<SolutionSetComparison> {
        let left_solve = match LinearEquationSolver::solve(left, variable) {
            MathematicalOutcome::Proven(result) => result.value,
            MathematicalOutcome::Unknown(unknown) => return MathematicalOutcome::Unknown(unknown),
            _ => {
                return MathematicalOutcome::Unknown(Unknown {
                    reason: UnknownReason::UnsupportedDomain,
                });
            }
        };

        let right_solve = match LinearEquationSolver::solve(right, variable) {
            MathematicalOutcome::Proven(result) => result.value,
            MathematicalOutcome::Unknown(unknown) => return MathematicalOutcome::Unknown(unknown),
            _ => {
                return MathematicalOutcome::Unknown(Unknown {
                    reason: UnknownReason::UnsupportedDomain,
                });
            }
        };

        let comparison = SolutionSetComparison {
            variable: variable.to_owned(),
            left_solution_set: left_solve.solution_set.clone(),
            right_solution_set: right_solve.solution_set.clone(),
            relation: "equation.same_solution_set".to_owned(),
            complete: true,
        };

        if left_solve.solution_set == right_solve.solution_set {
            MathematicalOutcome::Proven(Verified {
                value: comparison,
                evidence_id: Some("equation.same_solution_set.linear_rational".to_owned()),
            })
        } else {
            MathematicalOutcome::Disproven(Disproven {
                reason: format!(
                    "solution sets differ: left is {:?}, right is {:?}",
                    left_solve.solution_set, right_solve.solution_set
                ),
                evidence_id: Some("equation.same_solution_set.linear_rational.refute".to_owned()),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use socrates_math_core::{Context, Declaration, ExactRational, TypeId};
    use socrates_math_elab::{ElaborationOutcome, Elaborator};
    use socrates_math_syntax::{ParseOutcome, Parser};

    fn rational_context() -> Context {
        Context::root().with_declaration(Declaration {
            name: "x".to_owned(),
            type_id: TypeId::new("core.rational.rational").unwrap(),
        })
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
    fn proves_equivalent_solution_sets() {
        let left = statement("x + 1 = 3");
        let right = statement("x = 2");

        let MathematicalOutcome::Proven(result) =
            EquationComparator::compare_solution_sets(&left, &right, "x")
        else {
            panic!("expected proven comparison");
        };

        assert_eq!(
            result.value.left_solution_set,
            SolutionSet::Unique(ExactRational::integer(2))
        );
        assert_eq!(
            result.value.right_solution_set,
            SolutionSet::Unique(ExactRational::integer(2))
        );
        assert!(result.value.complete);
    }

    #[test]
    fn disproves_different_solution_sets() {
        let left = statement("x + 1 = 3");
        let right = statement("x = 3");

        assert!(matches!(
            EquationComparator::compare_solution_sets(&left, &right, "x"),
            MathematicalOutcome::Disproven(_)
        ));
    }
}
