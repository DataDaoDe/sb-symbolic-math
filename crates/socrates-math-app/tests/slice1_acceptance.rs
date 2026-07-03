use socrates_math_app::MathEngine;
use socrates_math_protocol::{ExactValueDto, MathematicalOutcomeKindDto, SolutionSetDto};

fn integer(value: &str) -> ExactValueDto {
    ExactValueDto::Integer {
        value: value.to_owned(),
    }
}

fn rational(numerator: &str, denominator: &str) -> ExactValueDto {
    ExactValueDto::Rational {
        numerator: numerator.to_owned(),
        denominator: denominator.to_owned(),
    }
}

#[test]
fn solves_the_first_documented_linear_equation_slice() {
    let result = MathEngine::solve_linear_equation("3(x - 2) + 4 = 2x + 9", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(
        result.solution_set,
        Some(SolutionSetDto::Unique {
            value: integer("11")
        })
    );
    assert_eq!(result.completeness.as_deref(), Some("Complete"));
    assert_eq!(result.solution_set_latex.as_deref(), Some("x = 11"));
    assert!(result.diagnostics.is_empty());
}

#[test]
fn solves_equations_with_exact_rational_answers() {
    let result = MathEngine::solve_linear_equation("2x = 1", "x");

    assert_eq!(
        result.solution_set,
        Some(SolutionSetDto::Unique {
            value: rational("1", "2")
        })
    );
    assert_eq!(
        result.solution_set_latex.as_deref(),
        Some("x = \\frac{1}{2}")
    );
}

#[test]
fn distinguishes_empty_and_universal_solution_sets() {
    let empty = MathEngine::solve_linear_equation("2x + 1 = 2x + 3", "x");
    let all = MathEngine::solve_linear_equation("2(x + 1) = 2x + 2", "x");

    assert_eq!(empty.solution_set, Some(SolutionSetDto::Empty));
    assert_eq!(all.solution_set, Some(SolutionSetDto::AllRationals));
    assert_eq!(empty.solution_set_latex.as_deref(), Some("\\varnothing"));
    assert_eq!(
        all.solution_set_latex.as_deref(),
        Some("x \\in \\mathbb{Q}")
    );
}

#[test]
fn compares_saved_answers_by_solution_set() {
    let result = MathEngine::compare_equation_solution_sets("x + 1 = 3", "x = 2", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "equation.same_solution_set");
    assert_eq!(result.equal, Some(true));
    assert_eq!(
        result.left_solution_set,
        Some(SolutionSetDto::Unique {
            value: integer("2")
        })
    );
    assert_eq!(
        result.right_solution_set,
        Some(SolutionSetDto::Unique {
            value: integer("2")
        })
    );
    assert_eq!(result.left_solution_set_latex.as_deref(), Some("x = 2"));
    assert_eq!(result.right_solution_set_latex.as_deref(), Some("x = 2"));
}

#[test]
fn disproves_different_solution_sets() {
    let result = MathEngine::compare_equation_solution_sets("x + 1 = 3", "x = 3", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Disproven);
    assert_eq!(result.equal, Some(false));
    assert_eq!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("DifferentSolutionSets")
    );
}

#[test]
fn reports_unsupported_math_as_unknown_not_false() {
    let result = MathEngine::solve_linear_equation("x x = 2", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
    assert_eq!(result.solution_set, None);
    assert_ne!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("DifferentSolutionSets")
    );
}

#[test]
fn returns_structured_parse_diagnostics() {
    let result = MathEngine::solve_linear_equation("3(x - 2 = 5", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
    assert_eq!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("Parse.ExpectedRightParenthesis")
    );
}
