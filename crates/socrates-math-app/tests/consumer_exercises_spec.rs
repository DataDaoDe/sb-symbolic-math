use socrates_math_app::MathEngine;
use socrates_math_protocol::{
    CompareMathExpressionsResponseDto, CompareNumericAnswerResponseDto,
    CompareSetExpressionsResponseDto, EvaluateFiniteRelationPredicateResponseDto,
    EvaluateSetCardinalityResponseDto, EvaluateSetStatementResponseDto, MathematicalOutcomeKindDto,
    RuleApplicabilityStatusDto, RuleTargetDto,
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
    assert_eq!(result.steps.len(), 4);
    assert_eq!(result.steps[0].rule, "calculus.derivative.sum-rule");
    assert_eq!(result.steps[0].target, Some(RuleTargetDto::Whole));
    assert_eq!(
        result.steps[0].input_latex.as_deref(),
        Some("\\frac{d}{dx}\\left(x^{3} + 2x\\right)")
    );
    assert_eq!(
        result.steps[1].rule,
        "calculus.polynomial.derivative.power-rule"
    );
    assert_eq!(
        result.steps[1].target,
        Some(RuleTargetDto::PolynomialTerm { degree: 3 })
    );
    assert_eq!(result.steps[1].input_latex.as_deref(), Some("x^{3}"));
    assert_eq!(result.steps[1].output_latex.as_deref(), Some("3x^{2}"));
    assert_eq!(
        result.steps[2].rule,
        "calculus.derivative.constant-multiple-rule"
    );
    assert_eq!(
        result.steps[2].target,
        Some(RuleTargetDto::PolynomialTerm { degree: 1 })
    );
    assert_eq!(
        result.steps[2].input_latex.as_deref(),
        Some("\\frac{d}{dx}\\left(2x\\right)")
    );
    assert_eq!(
        result.steps[2].output_latex.as_deref(),
        Some("2\\frac{d}{dx}\\left(x\\right)")
    );
    assert_eq!(
        result.steps[3].rule,
        "calculus.polynomial.derivative.power-rule"
    );
    assert_eq!(
        result.steps[3].target,
        Some(RuleTargetDto::PolynomialTerm { degree: 1 })
    );
    assert_eq!(result.steps[3].input_latex.as_deref(), Some("2x"));
    assert_eq!(result.steps[3].output_latex.as_deref(), Some("2"));
}

#[test]
fn rule_api_lists_legal_derivative_moves_for_selected_polynomial_term() {
    let result = MathEngine::list_applicable_math_expression_rules(
        "x^3 + 2x",
        "latex",
        "x",
        Some(RuleTargetDto::PolynomialTerm { degree: 3 }),
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.rules.len(), 2);
    assert_eq!(
        result.rules[0].rule,
        "calculus.polynomial.derivative.power-rule"
    );
    assert_eq!(
        result.rules[0].status,
        RuleApplicabilityStatusDto::Applicable
    );
    assert_eq!(
        result.rules[0].target,
        Some(RuleTargetDto::PolynomialTerm { degree: 3 })
    );
    assert_eq!(result.rules[0].relation, "calculus.derivative");
    assert_eq!(
        result.rules[1].rule,
        "calculus.polynomial.integral.power-rule"
    );
    assert_eq!(
        result.rules[1].status,
        RuleApplicabilityStatusDto::Applicable
    );
    assert_eq!(
        result.rules[1].target,
        Some(RuleTargetDto::PolynomialTerm { degree: 3 })
    );
    assert_eq!(result.rules[1].relation, "calculus.antiderivative");
}

#[test]
fn rule_api_lists_sum_rules_for_whole_polynomial_sum() {
    let result = MathEngine::list_applicable_math_expression_rules(
        "x^3 + 2x",
        "latex",
        "x",
        Some(RuleTargetDto::Whole),
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.rules.len(), 2);
    assert_eq!(result.rules[0].rule, "calculus.derivative.sum-rule");
    assert_eq!(
        result.rules[0].status,
        RuleApplicabilityStatusDto::Applicable
    );
    assert_eq!(result.rules[0].target, Some(RuleTargetDto::Whole));
    assert_eq!(result.rules[1].rule, "calculus.integral.sum-rule");
    assert_eq!(
        result.rules[1].status,
        RuleApplicabilityStatusDto::Applicable
    );
    assert_eq!(result.rules[1].target, Some(RuleTargetDto::Whole));
}

#[test]
fn rule_api_applies_derivative_sum_rule_to_whole_polynomial_sum() {
    let result = MathEngine::apply_math_expression_rule(
        "x^3 + 2x",
        "latex",
        "x",
        "calculus.derivative.sum-rule",
        Some(RuleTargetDto::Whole),
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.derivative");
    assert_eq!(
        result.previous.unwrap().latex,
        "\\frac{d}{dx}\\left(x^{3} + 2x\\right)"
    );
    assert_eq!(
        result.result.unwrap().latex,
        "\\frac{d}{dx}\\left(x^{3}\\right) + \\frac{d}{dx}\\left(2x\\right)"
    );

    let step = result.step.unwrap();
    assert_eq!(step.rule, "calculus.derivative.sum-rule");
    assert_eq!(step.target, Some(RuleTargetDto::Whole));
}

#[test]
fn rule_api_applies_integral_sum_rule_to_whole_polynomial_sum() {
    let result = MathEngine::apply_math_expression_rule(
        "x^3 + 2x",
        "latex",
        "x",
        "calculus.integral.sum-rule",
        Some(RuleTargetDto::Whole),
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.antiderivative");
    assert_eq!(result.previous.unwrap().latex, "\\int x^{3} + 2x\\,dx");
    assert_eq!(
        result.result.unwrap().latex,
        "\\int x^{3}\\,dx + \\int 2x\\,dx"
    );

    let step = result.step.unwrap();
    assert_eq!(step.rule, "calculus.integral.sum-rule");
    assert_eq!(step.target, Some(RuleTargetDto::Whole));
}

#[test]
fn rule_api_lists_constant_multiple_rules_for_selected_term_with_coefficient() {
    let result = MathEngine::list_applicable_math_expression_rules(
        "x^3 + 2x",
        "latex",
        "x",
        Some(RuleTargetDto::PolynomialTerm { degree: 1 }),
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.rules.len(), 4);
    assert_eq!(
        result.rules[0].rule,
        "calculus.derivative.constant-multiple-rule"
    );
    assert_eq!(
        result.rules[0].status,
        RuleApplicabilityStatusDto::Applicable
    );
    assert_eq!(
        result.rules[1].rule,
        "calculus.integral.constant-multiple-rule"
    );
    assert_eq!(
        result.rules[1].status,
        RuleApplicabilityStatusDto::Applicable
    );
}

#[test]
fn rule_api_applies_derivative_constant_multiple_rule_to_selected_term() {
    let result = MathEngine::apply_math_expression_rule(
        "x^3 + 2x",
        "latex",
        "x",
        "calculus.derivative.constant-multiple-rule",
        Some(RuleTargetDto::PolynomialTerm { degree: 1 }),
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.derivative");
    assert_eq!(
        result.previous.unwrap().latex,
        "\\frac{d}{dx}\\left(2x\\right)"
    );
    assert_eq!(
        result.result.unwrap().latex,
        "2\\frac{d}{dx}\\left(x\\right)"
    );

    let step = result.step.unwrap();
    assert_eq!(step.rule, "calculus.derivative.constant-multiple-rule");
    assert_eq!(
        step.target,
        Some(RuleTargetDto::PolynomialTerm { degree: 1 })
    );
}

#[test]
fn rule_api_applies_integral_constant_multiple_rule_to_selected_term() {
    let result = MathEngine::apply_math_expression_rule(
        "x^3 + 2x",
        "latex",
        "x",
        "calculus.integral.constant-multiple-rule",
        Some(RuleTargetDto::PolynomialTerm { degree: 1 }),
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.antiderivative");
    assert_eq!(result.previous.unwrap().latex, "\\int 2x\\,dx");
    assert_eq!(result.result.unwrap().latex, "2\\int x\\,dx");

    let step = result.step.unwrap();
    assert_eq!(step.rule, "calculus.integral.constant-multiple-rule");
    assert_eq!(
        step.target,
        Some(RuleTargetDto::PolynomialTerm { degree: 1 })
    );
}

#[test]
fn rule_api_applies_power_rule_to_selected_polynomial_term() {
    let result = MathEngine::apply_math_expression_rule(
        "x^3 + 2x",
        "latex",
        "x",
        "calculus.polynomial.derivative.power-rule",
        Some(RuleTargetDto::PolynomialTerm { degree: 3 }),
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.derivative");
    assert_eq!(result.previous.unwrap().latex, "x^{3}");
    assert_eq!(result.result.unwrap().latex, "3x^{2}");

    let step = result.step.unwrap();
    assert_eq!(step.rule, "calculus.polynomial.derivative.power-rule");
    assert_eq!(
        step.target,
        Some(RuleTargetDto::PolynomialTerm { degree: 3 })
    );
    assert_eq!(step.input_latex.as_deref(), Some("x^{3}"));
    assert_eq!(step.output_latex.as_deref(), Some("3x^{2}"));
}

#[test]
fn rule_api_auto_selects_the_only_polynomial_term_when_target_is_omitted() {
    let result = MathEngine::apply_math_expression_rule(
        "x^3",
        "latex",
        "x",
        "calculus.polynomial.derivative.power-rule",
        None,
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.derivative");
    assert_eq!(result.previous.unwrap().latex, "x^{3}");
    assert_eq!(result.result.unwrap().latex, "3x^{2}");
    assert_eq!(
        result.step.unwrap().target,
        Some(RuleTargetDto::PolynomialTerm { degree: 3 })
    );
}

#[test]
fn rule_api_lists_rules_for_whole_single_term_expression() {
    let result = MathEngine::list_applicable_math_expression_rules(
        "x^3",
        "latex",
        "x",
        Some(RuleTargetDto::Whole),
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.rules.len(), 2);
    assert_eq!(
        result.rules[0].target,
        Some(RuleTargetDto::PolynomialTerm { degree: 3 })
    );
    assert_eq!(
        result.rules[1].target,
        Some(RuleTargetDto::PolynomialTerm { degree: 3 })
    );
}

#[test]
fn rule_api_applies_antiderivative_power_rule_to_selected_polynomial_term() {
    let result = MathEngine::apply_math_expression_rule(
        "x^3 + 2x",
        "latex",
        "x",
        "calculus.polynomial.integral.power-rule",
        Some(RuleTargetDto::PolynomialTerm { degree: 3 }),
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.antiderivative");
    assert_eq!(result.previous.unwrap().latex, "x^{3}");
    assert_eq!(result.result.unwrap().latex, "\\frac{1}{4}x^{4}");

    let step = result.step.unwrap();
    assert_eq!(step.rule, "calculus.polynomial.integral.power-rule");
    assert_eq!(
        step.target,
        Some(RuleTargetDto::PolynomialTerm { degree: 3 })
    );
    assert_eq!(step.input_latex.as_deref(), Some("x^{3}"));
    assert_eq!(step.output_latex.as_deref(), Some("\\frac{1}{4}x^{4}"));
}

#[test]
fn rule_api_lists_rational_power_rules_for_whole_monomial() {
    let result = MathEngine::list_applicable_math_expression_rules("x^-1", "latex", "x", None);

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.rules.len(), 2);
    assert_eq!(
        result.rules[0].rule,
        "calculus.power.derivative.rational-rule"
    );
    assert_eq!(
        result.rules[0].status,
        RuleApplicabilityStatusDto::Applicable
    );
    assert_eq!(result.rules[0].target, Some(RuleTargetDto::Whole));
    assert_eq!(
        result.rules[1].rule,
        "calculus.power.integral.rational-rule"
    );
    assert_eq!(
        result.rules[1].status,
        RuleApplicabilityStatusDto::NotApplicable
    );
    assert_eq!(
        result.rules[1].required_conditions,
        vec!["exponent != -1".to_owned()]
    );
}

#[test]
fn rule_api_applies_rational_derivative_power_rule_to_whole_monomial() {
    let result = MathEngine::apply_math_expression_rule(
        "x^-1",
        "latex",
        "x",
        "calculus.power.derivative.rational-rule",
        None,
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.derivative");
    assert_eq!(result.previous.unwrap().latex, "x^{-1}");
    assert_eq!(result.result.unwrap().latex, "-x^{-2}");

    let step = result.step.unwrap();
    assert_eq!(step.rule, "calculus.power.derivative.rational-rule");
    assert_eq!(step.target, Some(RuleTargetDto::Whole));
}

#[test]
fn rule_api_applies_rational_integral_power_rule_to_whole_monomial() {
    let result = MathEngine::apply_math_expression_rule(
        "x^{\\frac{1}{2}}",
        "latex",
        "x",
        "calculus.power.integral.rational-rule",
        None,
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.antiderivative");
    assert_eq!(result.previous.unwrap().latex, "x^{\\frac{1}{2}}");
    assert_eq!(result.result.unwrap().latex, "\\frac{2}{3}x^{\\frac{3}{2}}");

    let step = result.step.unwrap();
    assert_eq!(step.rule, "calculus.power.integral.rational-rule");
    assert_eq!(step.target, Some(RuleTargetDto::Whole));
}

#[test]
fn rule_api_rejects_rational_integral_power_rule_at_negative_one() {
    let result = MathEngine::apply_math_expression_rule(
        "x^-1",
        "latex",
        "x",
        "calculus.power.integral.rational-rule",
        None,
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
    assert_eq!(result.relation, "calculus.antiderivative");
    assert_eq!(result.previous.unwrap().latex, "x^{-1}");
    assert_eq!(result.result, None);
    assert_eq!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("Rule.NotApplicable")
    );
}

#[test]
fn rule_api_requires_a_target_when_rule_application_is_ambiguous() {
    let result = MathEngine::apply_math_expression_rule(
        "x^3 + 2x",
        "latex",
        "x",
        "calculus.polynomial.derivative.power-rule",
        None,
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
    assert_eq!(result.result, None);
    assert_eq!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("Rule.AmbiguousTarget")
    );
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
fn calculus_integral_shows_sum_and_constant_multiple_rules() {
    let result = MathEngine::integrate_math_expression("x^3 + 2x", "latex", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.antiderivative");
    assert_eq!(result.result.unwrap().latex, "\\frac{1}{4}x^{4} + x^{2}");
    assert_eq!(result.steps.len(), 4);
    assert_eq!(result.steps[0].rule, "calculus.integral.sum-rule");
    assert_eq!(result.steps[0].target, Some(RuleTargetDto::Whole));
    assert_eq!(
        result.steps[0].input_latex.as_deref(),
        Some("\\int x^{3} + 2x\\,dx")
    );
    assert_eq!(
        result.steps[1].rule,
        "calculus.polynomial.integral.power-rule"
    );
    assert_eq!(
        result.steps[1].target,
        Some(RuleTargetDto::PolynomialTerm { degree: 3 })
    );
    assert_eq!(result.steps[1].input_latex.as_deref(), Some("x^{3}"));
    assert_eq!(
        result.steps[1].output_latex.as_deref(),
        Some("\\frac{1}{4}x^{4}")
    );
    assert_eq!(
        result.steps[2].rule,
        "calculus.integral.constant-multiple-rule"
    );
    assert_eq!(
        result.steps[2].target,
        Some(RuleTargetDto::PolynomialTerm { degree: 1 })
    );
    assert_eq!(
        result.steps[2].input_latex.as_deref(),
        Some("\\int 2x\\,dx")
    );
    assert_eq!(
        result.steps[2].output_latex.as_deref(),
        Some("2\\int x\\,dx")
    );
    assert_eq!(
        result.steps[3].rule,
        "calculus.polynomial.integral.power-rule"
    );
    assert_eq!(
        result.steps[3].target,
        Some(RuleTargetDto::PolynomialTerm { degree: 1 })
    );
    assert_eq!(result.steps[3].input_latex.as_deref(), Some("2x"));
    assert_eq!(result.steps[3].output_latex.as_deref(), Some("x^{2}"));
}

#[test]
fn calculus_derivative_supports_negative_integer_power_rule() {
    let result = MathEngine::differentiate_math_expression("x^-1", "latex", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.derivative");
    assert_eq!(result.result.unwrap().latex, "-x^{-2}");
    assert_eq!(result.steps.len(), 1);
    assert_eq!(
        result.steps[0].rule,
        "calculus.power.derivative.rational-rule"
    );
    assert_eq!(result.steps[0].input_latex.as_deref(), Some("x^{-1}"));
    assert_eq!(result.steps[0].output_latex.as_deref(), Some("-x^{-2}"));
}

#[test]
fn calculus_derivative_supports_rational_power_rule() {
    let result = MathEngine::differentiate_math_expression("x^{\\frac{1}{2}}", "latex", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.derivative");
    assert_eq!(
        result.result.unwrap().latex,
        "\\frac{1}{2}x^{-\\frac{1}{2}}"
    );
    assert_eq!(result.steps.len(), 1);
    assert_eq!(
        result.steps[0].rule,
        "calculus.power.derivative.rational-rule"
    );
    assert_eq!(
        result.steps[0].input_latex.as_deref(),
        Some("x^{\\frac{1}{2}}")
    );
    assert_eq!(
        result.steps[0].output_latex.as_deref(),
        Some("\\frac{1}{2}x^{-\\frac{1}{2}}")
    );
}

#[test]
fn calculus_integral_supports_rational_power_rule_except_negative_one() {
    let result = MathEngine::integrate_math_expression("x^{\\frac{1}{2}}", "latex", "x");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "calculus.antiderivative");
    assert_eq!(result.result.unwrap().latex, "\\frac{2}{3}x^{\\frac{3}{2}}");
    assert_eq!(result.steps.len(), 1);
    assert_eq!(
        result.steps[0].rule,
        "calculus.power.integral.rational-rule"
    );
    assert_eq!(
        result.steps[0].input_latex.as_deref(),
        Some("x^{\\frac{1}{2}}")
    );
    assert_eq!(
        result.steps[0].output_latex.as_deref(),
        Some("\\frac{2}{3}x^{\\frac{3}{2}}")
    );

    let reciprocal = MathEngine::integrate_math_expression("x^-1", "latex", "x");
    assert_eq!(reciprocal.outcome, MathematicalOutcomeKindDto::Unknown);
    assert_eq!(
        reciprocal
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("Unknown.UnsupportedDomain")
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

#[test]
fn set_literal_normalization_removes_duplicates_and_sorts_elements() {
    let result = MathEngine::normalize_set_expression("\\{3, 1, 2, 2\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "\\{1,2,3\\}");
}

#[test]
fn set_expression_compares_finite_sets_by_extensional_equality() {
    let result: CompareSetExpressionsResponseDto =
        MathEngine::compare_set_expressions("\\{3,1,2,2\\}", "\\{1,2,3\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "set.extensional_equal");
    assert_eq!(result.equal, Some(true));
    assert_eq!(result.left_normalized.unwrap().latex, "\\{1,2,3\\}");
    assert_eq!(result.right_normalized.unwrap().latex, "\\{1,2,3\\}");
}

#[test]
fn set_expression_rejects_different_finite_sets() {
    let result = MathEngine::compare_set_expressions("\\{1,2,3\\}", "\\{1,2,4\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Disproven);
    assert_eq!(result.equal, Some(false));
    assert_eq!(result.left_normalized.unwrap().latex, "\\{1,2,3\\}");
    assert_eq!(result.right_normalized.unwrap().latex, "\\{1,2,4\\}");
}

#[test]
fn set_expression_simplifies_finite_union() {
    let result = MathEngine::normalize_set_expression("\\{1,2\\} \\cup \\{2,3\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "\\{1,2,3\\}");
}

#[test]
fn set_expression_simplifies_finite_intersection() {
    let result = MathEngine::normalize_set_expression("\\{1,2,3\\} \\cap \\{2,3,4\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "\\{2,3\\}");
}

#[test]
fn set_expression_simplifies_finite_difference() {
    let result = MathEngine::normalize_set_expression("\\{1,2,3\\} \\setminus \\{2,4\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "\\{1,3\\}");
}

#[test]
fn set_expression_renders_empty_operation_result_canonically() {
    let result = MathEngine::normalize_set_expression("\\{1,2\\} \\cap \\{3,4\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "\\varnothing");
}

#[test]
fn set_expression_compares_operation_result_to_stored_answer() {
    let result =
        MathEngine::compare_set_expressions("\\{1,2\\} \\cup \\{2,3\\}", "\\{3,1,2\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.equal, Some(true));
    assert_eq!(result.left_normalized.unwrap().latex, "\\{1,2,3\\}");
    assert_eq!(result.right_normalized.unwrap().latex, "\\{1,2,3\\}");
}

#[test]
fn set_expression_respects_intersection_precedence_over_union() {
    let result =
        MathEngine::normalize_set_expression("\\{1\\} \\cup \\{2,3\\} \\cap \\{3,4\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "\\{1,3\\}");
}

#[test]
fn set_expression_normalizes_rosters_of_ordered_pairs() {
    let result = MathEngine::normalize_set_expression("\\{(2,b),(1,a),(1,b)\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "\\{(1,a),(1,b),(2,b)\\}");
}

#[test]
fn set_expression_lists_a_finite_cartesian_product() {
    let result = MathEngine::normalize_set_expression("\\{1,2\\} \\times \\{a,b\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(
        result.normalized.unwrap().latex,
        "\\{(1,a),(1,b),(2,a),(2,b)\\}"
    );
}

#[test]
fn set_expression_compares_cartesian_product_to_stored_answer() {
    let result = MathEngine::compare_set_expressions(
        "\\{1,2\\} \\times \\{a,b\\}",
        "\\{(2,b),(1,a),(2,a),(1,b)\\}",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.equal, Some(true));
    assert_eq!(
        result.left_normalized.unwrap().latex,
        "\\{(1,a),(1,b),(2,a),(2,b)\\}"
    );
}

#[test]
fn set_cardinality_counts_cartesian_product_elements() {
    let result = MathEngine::evaluate_set_cardinality("\\{1,2\\} \\times \\{a,b,c\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.cardinality, Some(6));
}

#[test]
fn set_statement_evaluates_ordered_pair_membership() {
    let result =
        MathEngine::evaluate_set_statement("(1,a) \\in \\{1,2\\} \\times \\{a,b\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(true));
    assert_eq!(
        result.normalized.unwrap().latex,
        "(1,a) \\in \\{(1,a),(1,b),(2,a),(2,b)\\}"
    );
}

#[test]
fn finite_relation_predicate_accepts_a_relation_from_domain_to_codomain() {
    let result: EvaluateFiniteRelationPredicateResponseDto =
        MathEngine::evaluate_relation_from("\\{(1,a),(2,b)\\}", "\\{1,2\\}", "\\{a,b\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "relation.from");
    assert_eq!(result.truth, Some(true));
    assert_eq!(
        result.normalized_relation.unwrap().latex,
        "\\{(1,a),(2,b)\\}"
    );
    assert_eq!(result.normalized_domain.unwrap().latex, "\\{1,2\\}");
    assert_eq!(result.normalized_codomain.unwrap().latex, "\\{a,b\\}");
}

#[test]
fn finite_relation_predicate_rejects_pairs_outside_codomain() {
    let result =
        MathEngine::evaluate_relation_from("\\{(1,a),(2,c)\\}", "\\{1,2\\}", "\\{a,b\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(false));
}

#[test]
fn finite_function_predicate_accepts_exactly_one_output_per_domain_element() {
    let result =
        MathEngine::evaluate_function_from("\\{(1,a),(2,a)\\}", "\\{1,2\\}", "\\{a,b\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "function.from");
    assert_eq!(result.truth, Some(true));
}

#[test]
fn finite_function_predicate_rejects_missing_domain_input() {
    let result =
        MathEngine::evaluate_function_from("\\{(1,a)\\}", "\\{1,2\\}", "\\{a,b\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(false));
}

#[test]
fn finite_function_predicate_rejects_conflicting_outputs() {
    let result = MathEngine::evaluate_function_from(
        "\\{(1,a),(1,b),(2,a)\\}",
        "\\{1,2\\}",
        "\\{a,b\\}",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(false));
}

#[test]
fn finite_relation_predicate_requires_ordered_pairs() {
    let result =
        MathEngine::evaluate_relation_from("\\{1,(2,b)\\}", "\\{1,2\\}", "\\{a,b\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
    assert_eq!(result.truth, None);
    assert_eq!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("Relation.ExpectedOrderedPairs")
    );
}

#[test]
fn finite_relation_property_detects_reflexive_relation() {
    let result = MathEngine::evaluate_relation_property(
        "\\{(1,1),(1,2),(2,1),(2,2)\\}",
        "\\{1,2\\}",
        "reflexive",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "relation.reflexive");
    assert_eq!(result.truth, Some(true));
    assert_eq!(
        result.normalized_relation.unwrap().latex,
        "\\{(1,1),(1,2),(2,1),(2,2)\\}"
    );
}

#[test]
fn finite_relation_property_rejects_non_reflexive_relation() {
    let result = MathEngine::evaluate_relation_property(
        "\\{(1,1),(1,2)\\}",
        "\\{1,2\\}",
        "reflexive",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(false));
}

#[test]
fn finite_relation_property_detects_symmetric_relation() {
    let result = MathEngine::evaluate_relation_property(
        "\\{(1,1),(1,2),(2,1)\\}",
        "\\{1,2\\}",
        "symmetric",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "relation.symmetric");
    assert_eq!(result.truth, Some(true));
}

#[test]
fn finite_relation_property_rejects_non_symmetric_relation() {
    let result = MathEngine::evaluate_relation_property(
        "\\{(1,1),(1,2)\\}",
        "\\{1,2\\}",
        "symmetric",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(false));
}

#[test]
fn finite_relation_property_detects_antisymmetric_relation() {
    let result = MathEngine::evaluate_relation_property(
        "\\{(1,1),(1,2),(2,2)\\}",
        "\\{1,2\\}",
        "antisymmetric",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "relation.antisymmetric");
    assert_eq!(result.truth, Some(true));
}

#[test]
fn finite_relation_property_rejects_non_antisymmetric_relation() {
    let result = MathEngine::evaluate_relation_property(
        "\\{(1,2),(2,1)\\}",
        "\\{1,2\\}",
        "antisymmetric",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(false));
}

#[test]
fn finite_relation_property_detects_transitive_relation() {
    let result = MathEngine::evaluate_relation_property(
        "\\{(1,1),(1,2),(1,3),(2,2),(2,3),(3,3)\\}",
        "\\{1,2,3\\}",
        "transitive",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "relation.transitive");
    assert_eq!(result.truth, Some(true));
}

#[test]
fn finite_relation_property_rejects_non_transitive_relation() {
    let result = MathEngine::evaluate_relation_property(
        "\\{(1,2),(2,3)\\}",
        "\\{1,2,3\\}",
        "transitive",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(false));
}

#[test]
fn finite_relation_property_rejects_pairs_outside_declared_set() {
    let result = MathEngine::evaluate_relation_property(
        "\\{(1,1),(2,2),(3,3)\\}",
        "\\{1,2\\}",
        "reflexive",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(false));
}

#[test]
fn finite_relation_property_reports_unsupported_property() {
    let result =
        MathEngine::evaluate_relation_property("\\{(1,1)\\}", "\\{1\\}", "connected", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
    assert_eq!(result.truth, None);
    assert_eq!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("Relation.UnsupportedProperty")
    );
}

#[test]
fn set_expression_lists_the_power_set_of_a_two_element_set() {
    let result = MathEngine::normalize_set_expression("\\mathcal{P}(\\{a,b\\})", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(
        result.normalized.unwrap().latex,
        "\\{\\varnothing,\\{a\\},\\{a,b\\},\\{b\\}\\}"
    );
}

#[test]
fn set_expression_compares_power_set_to_stored_roster_answer() {
    let result = MathEngine::compare_set_expressions(
        "\\mathcal{P}(\\{a,b\\})",
        "\\{\\{b\\},\\varnothing,\\{a,b\\},\\{a\\}\\}",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.equal, Some(true));
    assert_eq!(
        result.left_normalized.unwrap().latex,
        "\\{\\varnothing,\\{a\\},\\{a,b\\},\\{b\\}\\}"
    );
}

#[test]
fn set_builder_filters_even_numbers_from_a_finite_domain() {
    let result = MathEngine::normalize_set_expression(
        "\\{x \\in \\{1,2,3,4\\} \\mid x \\text{ is even}\\}",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "\\{2,4\\}");
}

#[test]
fn set_builder_supports_numeric_inequality_predicates() {
    let result =
        MathEngine::normalize_set_expression("\\{x \\in \\{1,2,3,4\\} \\mid x > 2\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "\\{3,4\\}");
}

#[test]
fn set_builder_supports_divisibility_predicates() {
    let result = MathEngine::normalize_set_expression(
        "\\{x \\in \\{1,2,3,4,5,6\\} \\mid 3 \\mid x\\}",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "\\{3,6\\}");
}

#[test]
fn set_builder_supports_membership_predicates() {
    let result = MathEngine::normalize_set_expression(
        "\\{x \\in \\{a,b,c\\} \\mid x \\in \\{b,c,d\\}\\}",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "\\{b,c\\}");
}

#[test]
fn set_builder_compares_against_a_stored_roster_answer() {
    let result = MathEngine::compare_set_expressions(
        "\\{x \\in \\{1,2,3,4\\} \\mid x \\text{ is even}\\}",
        "\\{4,2\\}",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.equal, Some(true));
    assert_eq!(result.left_normalized.unwrap().latex, "\\{2,4\\}");
    assert_eq!(result.right_normalized.unwrap().latex, "\\{2,4\\}");
}

#[test]
fn set_builder_reports_unsupported_predicates_as_unknown() {
    let result = MathEngine::normalize_set_expression(
        "\\{x \\in \\{1,2,3\\} \\mid x \\text{ is prime}\\}",
        "latex",
    );

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
    assert_eq!(result.normalized, None);
    assert_eq!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("Set.UnsupportedBuilderPredicate")
    );
}

#[test]
fn set_cardinality_counts_unique_elements_after_normalization() {
    let result: EvaluateSetCardinalityResponseDto =
        MathEngine::evaluate_set_cardinality("\\{a,b,b,c\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "set.cardinality");
    assert_eq!(result.cardinality, Some(3));
    assert_eq!(result.cardinality_latex.as_deref(), Some("3"));
    assert_eq!(result.normalized_set.unwrap().latex, "\\{a,b,c\\}");
}

#[test]
fn set_cardinality_counts_operation_results() {
    let result = MathEngine::evaluate_set_cardinality("\\{1,2\\} \\cup \\{2,3\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.cardinality, Some(3));
    assert_eq!(result.normalized_set.unwrap().latex, "\\{1,2,3\\}");
}

#[test]
fn set_cardinality_counts_power_set_elements() {
    let result = MathEngine::evaluate_set_cardinality("\\mathcal{P}(\\{a,b\\})", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.cardinality, Some(4));
    assert_eq!(
        result.normalized_set.unwrap().latex,
        "\\{\\varnothing,\\{a\\},\\{a,b\\},\\{b\\}\\}"
    );
}

#[test]
fn set_literal_normalization_preserves_singleton_as_a_set_element() {
    let result = MathEngine::normalize_set_expression("\\{2, \\{2\\}\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.normalized.unwrap().latex, "\\{2,\\{2\\}\\}");
}

#[test]
fn set_literal_normalization_renders_empty_set_canonically() {
    let empty_roster = MathEngine::normalize_set_expression("\\{\\}", "latex");
    let empty_symbol = MathEngine::normalize_set_expression("\\varnothing", "latex");

    assert_eq!(empty_roster.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(empty_roster.normalized.unwrap().latex, "\\varnothing");
    assert_eq!(empty_symbol.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(empty_symbol.normalized.unwrap().latex, "\\varnothing");
}

#[test]
fn set_literal_normalization_reports_malformed_roster_sets_as_unknown() {
    let result = MathEngine::normalize_set_expression("\\{1,2", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
    assert_eq!(result.normalized, None);
    assert_eq!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("Set.ExpectedSeparatorOrClose")
    );
}

#[test]
fn set_statement_evaluates_membership_in_a_finite_roster_set() {
    let result: EvaluateSetStatementResponseDto =
        MathEngine::evaluate_set_statement("2 \\in \\{1,2,3\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.relation, "logic.truth");
    assert_eq!(result.truth, Some(true));
    assert_eq!(result.normalized.unwrap().latex, "2 \\in \\{1,2,3\\}");
}

#[test]
fn set_statement_evaluates_non_membership_in_a_finite_roster_set() {
    let result = MathEngine::evaluate_set_statement("4 \\notin \\{1,2,3\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(true));
    assert_eq!(result.normalized.unwrap().latex, "4 \\notin \\{1,2,3\\}");
}

#[test]
fn set_statement_distinguishes_element_from_singleton_set() {
    let element = MathEngine::evaluate_set_statement("2 \\in \\{1,2,3\\}", "latex");
    let singleton = MathEngine::evaluate_set_statement("\\{2\\} \\in \\{1,2,3\\}", "latex");

    assert_eq!(element.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(element.truth, Some(true));
    assert_eq!(singleton.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(singleton.truth, Some(false));
    assert_eq!(
        singleton.normalized.unwrap().latex,
        "\\{2\\} \\in \\{1,2,3\\}"
    );
}

#[test]
fn set_statement_evaluates_subset_inclusion() {
    let result = MathEngine::evaluate_set_statement("\\{1,2\\} \\subseteq \\{1,2,3\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(true));
    assert_eq!(
        result.normalized.unwrap().latex,
        "\\{1,2\\} \\subseteq \\{1,2,3\\}"
    );
}

#[test]
fn set_statement_evaluates_failed_subset_inclusion_as_false() {
    let result = MathEngine::evaluate_set_statement("\\{1,4\\} \\subseteq \\{1,2,3\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(false));
    assert_eq!(
        result.normalized.unwrap().latex,
        "\\{1,4\\} \\subseteq \\{1,2,3\\}"
    );
}

#[test]
fn set_statement_evaluates_not_subset_inclusion() {
    let result = MathEngine::evaluate_set_statement("\\{1,4\\} \\nsubseteq \\{1,2,3\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
    assert_eq!(result.truth, Some(true));
    assert_eq!(
        result.normalized.unwrap().latex,
        "\\{1,4\\} \\nsubseteq \\{1,2,3\\}"
    );
}

#[test]
fn set_statement_reports_missing_operator_as_unknown() {
    let result = MathEngine::evaluate_set_statement("\\{1,2\\} \\{1,2,3\\}", "latex");

    assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
    assert_eq!(result.truth, None);
    assert_eq!(
        result
            .diagnostics
            .first()
            .map(|diagnostic| diagnostic.code.as_str()),
        Some("Set.ExpectedStatementOperator")
    );
}
