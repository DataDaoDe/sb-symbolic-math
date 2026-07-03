use socrates_math_app::MathEngine;
use socrates_math_protocol::{
    CompareMathExpressionsResponseDto, CompareNumericAnswerResponseDto, MathematicalOutcomeKindDto,
};

struct MathExpressionAnswerKey<'a> {
    variable: &'a str,
    accepted_expressions: &'a [&'a str],
}

struct MathExpressionGrade {
    correct: bool,
    normalized_latex: Option<String>,
    matched_expression: Option<String>,
}

fn grade_math_expression_answer(
    answer_key: &MathExpressionAnswerKey,
    submitted_latex: &str,
) -> MathExpressionGrade {
    let normalized =
        MathEngine::normalize_math_expression(submitted_latex, "latex", answer_key.variable);

    let Some(normalized_expression) = normalized.normalized else {
        return MathExpressionGrade {
            correct: false,
            normalized_latex: None,
            matched_expression: None,
        };
    };

    for accepted_expression in answer_key.accepted_expressions {
        let comparison = MathEngine::compare_math_expressions(
            accepted_expression,
            submitted_latex,
            "latex",
            answer_key.variable,
        );

        if comparison.outcome == MathematicalOutcomeKindDto::Proven
            && comparison.equal == Some(true)
        {
            return MathExpressionGrade {
                correct: true,
                normalized_latex: Some(normalized_expression.latex),
                matched_expression: Some((*accepted_expression).to_owned()),
            };
        }
    }

    MathExpressionGrade {
        correct: false,
        normalized_latex: Some(normalized_expression.latex),
        matched_expression: None,
    }
}

#[test]
fn math_expression_normalizes_latex_and_accepts_equivalent_forms() {
    let answer_key = MathExpressionAnswerKey {
        variable: "x",
        accepted_expressions: &["3x - 2"],
    };

    let grade = grade_math_expression_answer(&answer_key, "3(x - 2) + 4");

    assert!(grade.correct);
    assert_eq!(grade.normalized_latex.as_deref(), Some("3x - 2"));
    assert_eq!(grade.matched_expression.as_deref(), Some("3x - 2"));
}

#[test]
fn math_expression_rejects_a_different_expression() {
    let answer_key = MathExpressionAnswerKey {
        variable: "x",
        accepted_expressions: &["3x - 2"],
    };

    let grade = grade_math_expression_answer(&answer_key, "3x + 2");

    assert!(!grade.correct);
    assert_eq!(grade.normalized_latex.as_deref(), Some("3x + 2"));
    assert_eq!(grade.matched_expression, None);
}

#[test]
fn math_expression_reports_unsupported_domains_as_unknown() {
    let result = MathEngine::normalize_math_expression("x^-1", "latex", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
    assert_eq!(result.normalized, None);
    assert_eq!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("Unknown.UnsupportedDomain")
    );
}

#[test]
fn math_expression_normalizes_polynomial_powers() {
    let result = MathEngine::normalize_math_expression("x^3 + x^3", "latex", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "2x^{3}");
}

#[test]
fn math_expression_compares_equivalent_polynomials() {
    let result = MathEngine::compare_math_expressions("(x + 1)(x - 1)", "x^2 - 1", "latex", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.equal, Some(true));
    assert_eq!(result.left_normalized.unwrap().latex, "x^{2} - 1");
    assert_eq!(result.right_normalized.unwrap().latex, "x^{2} - 1");
}

#[test]
fn calculus_derivative_returns_result_and_power_rule_step() {
    let result = MathEngine::differentiate_math_expression("x^3 + 2x", "latex", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.derivative");
    assert_eq!(result.result.unwrap().latex, "3x^{2} + 2");
    assert_eq!(result.steps.len(), 1);
    assert_eq!(
        result.steps[0].rule,
        "calculus.polynomial.derivative.power-rule"
    );
    assert_eq!(result.steps[0].input_latex.as_deref(), Some("x^{3} + 2x"));
    assert_eq!(result.steps[0].output_latex.as_deref(), Some("3x^{2} + 2"));
}

#[test]
fn calculus_integral_returns_one_antiderivative_and_power_rule_step() {
    let result = MathEngine::integrate_math_expression("x^3", "latex", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.antiderivative");
    assert_eq!(result.result.unwrap().latex, "\\frac{1}{4}x^{4}");
    assert_eq!(result.steps.len(), 1);
    assert_eq!(
        result.steps[0].rule,
        "calculus.polynomial.integral.power-rule"
    );
    assert_eq!(result.steps[0].input_latex.as_deref(), Some("x^{3}"));
    assert_eq!(
        result.steps[0].output_latex.as_deref(),
        Some("\\frac{1}{4}x^{4}")
    );
}

#[test]
fn math_equation_compares_solution_sets_not_surface_expression_forms() {
    let result = MathEngine::compare_equation_solution_sets("x + 1 = 3", "2x = 4", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.equal, Some(true));
    assert_eq!(result.left_solution_set_latex.as_deref(), Some("x = 2"));
    assert_eq!(result.right_solution_set_latex.as_deref(), Some("x = 2"));
}

#[test]
fn numeric_answer_accepts_values_inside_absolute_tolerance() {
    let result =
        MathEngine::compare_numeric_answer("\\frac{333}{1000}", "\\frac{1}{3}", "latex", 0.001);

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.equal, Some(true));
    assert!(result.absolute_error.unwrap() <= result.tolerance);
}

#[test]
fn numeric_answer_rejects_values_outside_absolute_tolerance() {
    let result =
        MathEngine::compare_numeric_answer("\\frac{333}{1000}", "\\frac{1}{3}", "latex", 0.0001);

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Disproven);
    assert_eq!(result.equal, Some(false));
    assert_eq!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("Number.OutsideTolerance")
    );
}

#[test]
fn numeric_answer_requires_a_constant_value() {
    let result = MathEngine::compare_numeric_answer("x + 1", "2", "latex", 0.001);

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
    assert_eq!(result.equal, None);
    assert_eq!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("Number.ExpectedConstant")
    );
}

#[test]
fn direct_expression_comparison_returns_both_normal_forms() {
    let result: CompareMathExpressionsResponseDto =
        MathEngine::compare_math_expressions("2(x + 1)", "2x + 2", "latex", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.equal, Some(true));
    assert_eq!(result.left_normalized.unwrap().latex, "2x + 2");
    assert_eq!(result.right_normalized.unwrap().latex, "2x + 2");
}

#[test]
fn direct_numeric_comparison_exposes_values_for_feedback() {
    let result: CompareNumericAnswerResponseDto =
        MathEngine::compare_numeric_answer("\\frac{1}{2}", "\\frac{2}{4}", "latex", 0.0);

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.equal, Some(true));
    assert_eq!(result.submitted_value, Some(0.5));
    assert_eq!(result.expected_value, Some(0.5));
    assert_eq!(result.absolute_error, Some(0.0));
}
