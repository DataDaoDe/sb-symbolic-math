use crate::{real_function, unit};
use socrates_math_algebra::PolynomialExpression;
use socrates_math_core::ExactRational;
use socrates_math_protocol::{
    ApplyDifferenceQuotientRuleResponseDto, AverageRateResponseDto, DiagnosticDto,
    DifferenceQuotientResponseDto, ExactQuantityDto, ExactValueDto, MathDerivationStepDto,
    MathExpressionDto, MathematicalOutcomeKindDto, RealFunctionSourceDto,
};
use socrates_math_render::LatexRenderer;
use std::cmp::Ordering;
use std::collections::BTreeMap;

const RATE_COMPLETENESS: &str = "exact average rate for the supported real-function theory";
const QUOTIENT_COMPLETENESS: &str =
    "difference quotient for exact-rational polynomial functions of degree at most 32";
const RULES: &[&str] = &[
    "calculus.difference-quotient.construct",
    "algebra.polynomial.increment-substitution.expand",
    "algebra.polynomial.increment.factor",
    "algebra.rational.cancel-nonzero-factor",
];

pub fn average_rate_response(
    source: &RealFunctionSourceDto,
    left_input: &ExactQuantityDto,
    right_input: &ExactQuantityDto,
) -> AverageRateResponseDto {
    let validated = match real_function::validate(source) {
        Ok(value) => value,
        Err((outcome, diagnostic)) => {
            return average_failure(outcome, left_input, right_input, diagnostic);
        }
    };
    let left = match real_function::coerce_input(&validated, left_input) {
        Ok(value) => value,
        Err(diagnostic) => {
            return average_failure(
                MathematicalOutcomeKindDto::Unknown,
                left_input,
                right_input,
                diagnostic,
            );
        }
    };
    let right = match real_function::coerce_input(&validated, right_input) {
        Ok(value) => value,
        Err(diagnostic) => {
            return average_failure(
                MathematicalOutcomeKindDto::Unknown,
                left_input,
                right_input,
                diagnostic,
            );
        }
    };
    if left == right {
        return AverageRateResponseDto {
            outcome: MathematicalOutcomeKindDto::Undefined,
            relation: "function.average-rate".to_owned(),
            value: None,
            left_input: left_input.clone(),
            right_input: right_input.clone(),
            function: Some(validated.dto),
            completeness: RATE_COMPLETENESS.to_owned(),
            diagnostics: vec![diagnostic(
                "Rate.EqualEndpoints",
                "average rate is undefined when the two endpoints are equal",
            )],
        };
    }
    let left_output = real_function::evaluate_input(&validated, left_input);
    let right_output = real_function::evaluate_input(&validated, right_input);
    if left_output.outcome != MathematicalOutcomeKindDto::Proven {
        return evaluated_average_failure(left_output, left_input, right_input, validated.dto);
    }
    if right_output.outcome != MathematicalOutcomeKindDto::Proven {
        return evaluated_average_failure(right_output, left_input, right_input, validated.dto);
    }
    let left_value = unit::exact(
        &left_output
            .output
            .expect("proven evaluation has output")
            .value,
    )
    .expect("engine exact output");
    let right_value = unit::exact(
        &right_output
            .output
            .expect("proven evaluation has output")
            .value,
    )
    .expect("engine exact output");
    let rate = right_value
        .sub(&left_value)
        .div(&right.sub(&left))
        .expect("distinct endpoints have nonzero difference");
    let rate_unit = match unit::quotient(
        validated.dto.output_unit.as_ref(),
        validated.dto.input_unit.as_ref(),
    ) {
        Ok(value) => value,
        Err(diagnostic) => {
            return average_failure(
                MathematicalOutcomeKindDto::Unknown,
                left_input,
                right_input,
                diagnostic,
            );
        }
    };
    AverageRateResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        relation: "function.average-rate".to_owned(),
        value: Some(ExactQuantityDto {
            value: ExactValueDto::from(&rate),
            unit: rate_unit,
        }),
        left_input: left_input.clone(),
        right_input: right_input.clone(),
        function: Some(validated.dto),
        completeness: RATE_COMPLETENESS.to_owned(),
        diagnostics: Vec::new(),
    }
}

pub fn difference_quotient_response(
    source: &RealFunctionSourceDto,
    increment_variable: &str,
) -> DifferenceQuotientResponseDto {
    let derivation = match derive(source, increment_variable) {
        Ok(value) => value,
        Err((outcome, diagnostic)) => {
            return DifferenceQuotientResponseDto {
                outcome,
                relation: "function.difference-quotient".to_owned(),
                increment_variable: increment_variable.to_owned(),
                conditions: Vec::new(),
                initial: None,
                result: None,
                result_unit: None,
                applicable_rules: Vec::new(),
                steps: Vec::new(),
                completeness: QUOTIENT_COMPLETENESS.to_owned(),
                diagnostics: vec![diagnostic],
            };
        }
    };
    DifferenceQuotientResponseDto {
        outcome: MathematicalOutcomeKindDto::Conditional,
        relation: "function.difference-quotient".to_owned(),
        increment_variable: increment_variable.to_owned(),
        conditions: derivation.conditions,
        initial: Some(MathExpressionDto {
            latex: derivation.initial,
        }),
        result: Some(MathExpressionDto {
            latex: derivation.normalized,
        }),
        result_unit: derivation.result_unit,
        applicable_rules: RULES.iter().map(|rule| (*rule).to_owned()).collect(),
        steps: derivation.steps,
        completeness: QUOTIENT_COMPLETENESS.to_owned(),
        diagnostics: Vec::new(),
    }
}

pub fn apply_difference_quotient_rule_response(
    source: &RealFunctionSourceDto,
    increment_variable: &str,
    rule: &str,
) -> ApplyDifferenceQuotientRuleResponseDto {
    let derivation = match derive(source, increment_variable) {
        Ok(value) => value,
        Err((outcome, diagnostic)) => {
            return rule_failure(outcome, increment_variable, rule, diagnostic);
        }
    };
    let Some(index) = RULES.iter().position(|candidate| *candidate == rule) else {
        return rule_failure(
            MathematicalOutcomeKindDto::Unknown,
            increment_variable,
            rule,
            diagnostic("Rule.Unsupported", "unknown difference-quotient rule"),
        );
    };
    let step = derivation.steps[index].clone();
    ApplyDifferenceQuotientRuleResponseDto {
        outcome: MathematicalOutcomeKindDto::Conditional,
        relation: "function.difference-quotient".to_owned(),
        rule: rule.to_owned(),
        conditions: derivation.conditions,
        previous: step
            .input_latex
            .clone()
            .map(|latex| MathExpressionDto { latex }),
        result: step
            .output_latex
            .clone()
            .map(|latex| MathExpressionDto { latex }),
        step: Some(step),
        diagnostics: Vec::new(),
    }
}

struct Derivation {
    initial: String,
    normalized: String,
    conditions: Vec<String>,
    result_unit: Option<socrates_math_protocol::UnitDto>,
    steps: Vec<MathDerivationStepDto>,
}

fn derive(
    source: &RealFunctionSourceDto,
    increment: &str,
) -> Result<Derivation, (MathematicalOutcomeKindDto, DiagnosticDto)> {
    let validated = real_function::validate(source)?;
    if increment.trim().is_empty() || increment == validated.dto.input.symbol {
        return Err((
            MathematicalOutcomeKindDto::Unknown,
            diagnostic(
                "DifferenceQuotient.InvalidIncrementBinding",
                "increment variable must be explicit and distinct from the input variable",
            ),
        ));
    }
    if !is_one(&validated.formula.denominator) {
        return Err((
            MathematicalOutcomeKindDto::Unknown,
            diagnostic(
                "DifferenceQuotient.UnsupportedFunction",
                "normalized difference quotients currently require a polynomial function",
            ),
        ));
    }
    let degree = validated
        .formula
        .numerator
        .coefficients
        .keys()
        .next_back()
        .copied()
        .unwrap_or(0);
    if degree > 32 {
        return Err((
            MathematicalOutcomeKindDto::Unknown,
            diagnostic(
                "DifferenceQuotient.ResourceLimit",
                "polynomial degree exceeds the complete difference-quotient slice",
            ),
        ));
    }
    let variable = &validated.dto.input.symbol;
    let shifted = PolynomialExpression {
        variable: format!("\\left({variable} + {increment}\\right)"),
        coefficients: validated.formula.numerator.coefficients.clone(),
    };
    let original = &validated.formula.numerator;
    let initial = format!(
        "\\frac{{{} - \\left({}\\right)}}{{{increment}}}",
        LatexRenderer::polynomial_expression(&shifted),
        LatexRenderer::polynomial_expression(original)
    );
    let expanded_terms = quotient_terms(original, false)?;
    let normalized_terms = quotient_terms(original, true)?;
    let expanded_numerator = render_bivariate(&expanded_terms, variable, increment);
    let normalized = render_bivariate(&normalized_terms, variable, increment);
    let expanded = format!("\\frac{{{expanded_numerator}}}{{{increment}}}");
    let factored = format!("\\frac{{{increment}\\left({normalized}\\right)}}{{{increment}}}");
    let condition = format!("{increment} != 0");
    let states = [
        (
            LatexRenderer::polynomial_expression(original),
            initial.clone(),
            "calculus.difference-quotient.construct",
            "Substitute an increment into the function and form the difference quotient.",
        ),
        (
            initial.clone(),
            expanded.clone(),
            "algebra.polynomial.increment-substitution.expand",
            "Expand the polynomial increment substitution exactly.",
        ),
        (
            expanded.clone(),
            factored.clone(),
            "algebra.polynomial.increment.factor",
            "Factor the common nonzero increment from the numerator.",
        ),
        (
            factored,
            normalized.clone(),
            "algebra.rational.cancel-nonzero-factor",
            "Cancel the increment using the retained nonzero condition.",
        ),
    ];
    let steps = states
        .into_iter()
        .map(|(input, output, rule, reason)| MathDerivationStepDto {
            rule: rule.to_owned(),
            reason: reason.to_owned(),
            target: None,
            input_latex: Some(input),
            output_latex: Some(output),
        })
        .collect();
    let result_unit = unit::quotient(
        validated.dto.output_unit.as_ref(),
        validated.dto.input_unit.as_ref(),
    )
    .map_err(|diagnostic| (MathematicalOutcomeKindDto::Unknown, diagnostic))?;
    Ok(Derivation {
        initial,
        normalized,
        conditions: vec![condition],
        result_unit,
        steps,
    })
}

fn quotient_terms(
    polynomial: &PolynomialExpression,
    cancel_increment: bool,
) -> Result<BTreeMap<(u32, u32), ExactRational>, (MathematicalOutcomeKindDto, DiagnosticDto)> {
    let mut terms = BTreeMap::new();
    for (degree, coefficient) in &polynomial.coefficients {
        for k in 1..=*degree {
            let choose = binomial(*degree, k).ok_or_else(|| {
                (
                    MathematicalOutcomeKindDto::Unknown,
                    diagnostic(
                        "DifferenceQuotient.ResourceLimit",
                        "binomial coefficient exceeds the exact bounded implementation",
                    ),
                )
            })?;
            let choose = i64::try_from(choose).map_err(|_| {
                (
                    MathematicalOutcomeKindDto::Unknown,
                    diagnostic(
                        "DifferenceQuotient.ResourceLimit",
                        "binomial coefficient exceeds the exact bounded implementation",
                    ),
                )
            })?;
            let h_degree = if cancel_increment { k - 1 } else { k };
            let key = (degree - k, h_degree);
            let value = coefficient.mul(&ExactRational::integer(choose));
            let previous = terms
                .remove(&key)
                .unwrap_or_else(|| ExactRational::integer(0));
            terms.insert(key, previous.add(&value));
        }
    }
    Ok(terms)
}

fn binomial(n: u32, k: u32) -> Option<u64> {
    let k = k.min(n - k);
    let mut value = 1_u64;
    for i in 0..k {
        value = value.checked_mul(u64::from(n - i))? / u64::from(i + 1);
    }
    Some(value)
}

fn render_bivariate(
    terms: &BTreeMap<(u32, u32), ExactRational>,
    variable: &str,
    increment: &str,
) -> String {
    let mut rendered = String::new();
    for ((x_degree, h_degree), coefficient) in terms.iter().rev() {
        if coefficient.is_zero() {
            continue;
        }
        let negative = coefficient.cmp_exact(&ExactRational::integer(0)) == Ordering::Less;
        let magnitude = if negative {
            coefficient.neg()
        } else {
            coefficient.clone()
        };
        let mut factors = String::new();
        append_power(&mut factors, variable, *x_degree);
        append_power(&mut factors, increment, *h_degree);
        let term = if factors.is_empty() {
            LatexRenderer::exact_rational(&magnitude)
        } else if magnitude == ExactRational::integer(1) {
            factors
        } else {
            format!("{}{factors}", LatexRenderer::exact_rational(&magnitude))
        };
        if rendered.is_empty() {
            if negative {
                rendered.push('-');
            }
            rendered.push_str(&term);
        } else if negative {
            rendered.push_str(" - ");
            rendered.push_str(&term);
        } else {
            rendered.push_str(" + ");
            rendered.push_str(&term);
        }
    }
    if rendered.is_empty() {
        "0".to_owned()
    } else {
        rendered
    }
}

fn append_power(rendered: &mut String, variable: &str, degree: u32) {
    match degree {
        0 => {}
        1 => rendered.push_str(variable),
        _ => rendered.push_str(&format!("{variable}^{{{degree}}}")),
    }
}

fn is_one(polynomial: &PolynomialExpression) -> bool {
    polynomial.coefficients.len() == 1
        && polynomial.coefficients.get(&0) == Some(&ExactRational::integer(1))
}

fn average_failure(
    outcome: MathematicalOutcomeKindDto,
    left: &ExactQuantityDto,
    right: &ExactQuantityDto,
    diagnostic: DiagnosticDto,
) -> AverageRateResponseDto {
    AverageRateResponseDto {
        outcome,
        relation: "function.average-rate".to_owned(),
        value: None,
        left_input: left.clone(),
        right_input: right.clone(),
        function: None,
        completeness: RATE_COMPLETENESS.to_owned(),
        diagnostics: vec![diagnostic],
    }
}

fn evaluated_average_failure(
    evaluated: real_function::EvaluatedInput,
    left: &ExactQuantityDto,
    right: &ExactQuantityDto,
    function: socrates_math_protocol::RealFunctionDto,
) -> AverageRateResponseDto {
    AverageRateResponseDto {
        outcome: evaluated.outcome,
        relation: "function.average-rate".to_owned(),
        value: None,
        left_input: left.clone(),
        right_input: right.clone(),
        function: Some(function),
        completeness: RATE_COMPLETENESS.to_owned(),
        diagnostics: evaluated.diagnostics,
    }
}

fn rule_failure(
    outcome: MathematicalOutcomeKindDto,
    increment: &str,
    rule: &str,
    diagnostic: DiagnosticDto,
) -> ApplyDifferenceQuotientRuleResponseDto {
    ApplyDifferenceQuotientRuleResponseDto {
        outcome,
        relation: "function.difference-quotient".to_owned(),
        rule: rule.to_owned(),
        conditions: if increment.is_empty() {
            Vec::new()
        } else {
            vec![format!("{increment} != 0")]
        },
        previous: None,
        result: None,
        step: None,
        diagnostics: vec![diagnostic],
    }
}

fn diagnostic(code: &str, message: &str) -> DiagnosticDto {
    DiagnosticDto {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use socrates_math_protocol::{
        ExplicitFunctionDefinitionSourceDto, RealFunctionInputSourceDto,
        RealFunctionOutputSourceDto, UnitDimensionDto, UnitDto,
    };

    fn source(expression: &str) -> RealFunctionSourceDto {
        RealFunctionSourceDto {
            schema: "socrates.real-function".to_owned(),
            version: 1,
            input: RealFunctionInputSourceDto {
                variable: "x".to_owned(),
                label: None,
                unit: None,
            },
            output: RealFunctionOutputSourceDto {
                label: None,
                unit: None,
            },
            definition: ExplicitFunctionDefinitionSourceDto {
                kind: "explicit".to_owned(),
                expression: expression.to_owned(),
                input_format: "latex".to_owned(),
            },
            declared_domain: None,
            declared_codomain: None,
            parameters: Vec::new(),
            assumptions: Vec::new(),
        }
    }

    fn integer(value: &str) -> ExactQuantityDto {
        ExactQuantityDto {
            value: ExactValueDto::Integer {
                value: value.to_owned(),
            },
            unit: None,
        }
    }

    fn length_unit(symbol: &str) -> UnitDto {
        UnitDto {
            schema: "socrates.unit".to_owned(),
            version: 1,
            dimensions: vec![UnitDimensionDto {
                base: "length".to_owned(),
                exponent: ExactValueDto::Integer {
                    value: "1".to_owned(),
                },
            }],
            scale_to_canonical: ExactValueDto::Integer {
                value: "1".to_owned(),
            },
            symbol: symbol.to_owned(),
        }
    }

    #[test]
    fn computes_exact_average_rate_and_rejects_equal_endpoints() {
        let exact = average_rate_response(&source("x^2"), &integer("1"), &integer("3"));
        assert_eq!(exact.outcome, MathematicalOutcomeKindDto::Proven);
        assert_eq!(
            exact.value.expect("rate").value,
            ExactValueDto::Integer {
                value: "4".to_owned()
            }
        );
        let undefined = average_rate_response(&source("x^2"), &integer("1"), &integer("1"));
        assert_eq!(undefined.outcome, MathematicalOutcomeKindDto::Undefined);
    }

    #[test]
    fn composes_output_over_input_units_for_rates() {
        let mut function = source("x^2");
        function.input.unit = Some(length_unit("m"));
        function.output.unit = Some(length_unit("m"));
        let rate = average_rate_response(
            &function,
            &ExactQuantityDto {
                value: integer("1").value,
                unit: Some(length_unit("m")),
            },
            &ExactQuantityDto {
                value: integer("3").value,
                unit: Some(length_unit("m")),
            },
        );
        let unit = rate.value.expect("rate").unit.expect("dimensionless unit");
        assert!(unit.dimensions.is_empty());
        assert_eq!(unit.symbol, "m/m");
    }

    #[test]
    fn preserves_nonzero_condition_and_replays_every_automated_step() {
        let function = source("x^2");
        let derived = difference_quotient_response(&function, "h");
        assert_eq!(derived.outcome, MathematicalOutcomeKindDto::Conditional);
        assert_eq!(derived.conditions, vec!["h != 0"]);
        assert_eq!(
            derived.result.as_ref().map(|value| value.latex.as_str()),
            Some("2x + h")
        );
        for step in &derived.steps {
            let replayed = apply_difference_quotient_rule_response(&function, "h", &step.rule);
            assert_eq!(replayed.conditions, vec!["h != 0"]);
            assert_eq!(replayed.step.as_ref(), Some(step));
            assert_eq!(
                replayed.previous.as_ref().map(|value| &value.latex),
                step.input_latex.as_ref()
            );
            assert_eq!(
                replayed.result.as_ref().map(|value| &value.latex),
                step.output_latex.as_ref()
            );
        }
    }

    #[test]
    fn refuses_to_normalize_a_rational_difference_quotient_outside_the_slice() {
        let result = difference_quotient_response(&source("1/x"), "h");
        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
        assert!(result.result.is_none());
    }
}
