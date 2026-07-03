use socrates_math_algebra::{LinearNormalizer, PolynomialExpression, PolynomialNormalizer};
use socrates_math_compare::EquationComparator;
use socrates_math_core::{
    Context, Declaration, Judgment, MathematicalOutcome, SemanticTerm, TypeId, UnknownReason,
};
use socrates_math_elab::{ElaborationDiagnosticCode, ElaborationOutcome, Elaborator};
use socrates_math_protocol::{
    CompareEquationsResponseDto, CompareMathExpressionsResponseDto,
    CompareNumericAnswerResponseDto, DiagnosticDto, MathDerivationStepDto, MathExpressionDto,
    MathematicalOutcomeKindDto, NormalizeMathExpressionResponseDto, SolutionSetDto,
    SolveLinearEquationResponseDto, TransformMathExpressionResponseDto,
};
use socrates_math_render::LatexRenderer;
use socrates_math_solve::LinearEquationSolver;
use socrates_math_syntax::{DiagnosticCode, ParseOutcome, Parser};

pub struct MathEngine;

impl MathEngine {
    pub fn normalize_math_expression(
        source: &str,
        _input_format: &str,
        variable: &str,
    ) -> NormalizeMathExpressionResponseDto {
        let term = match parse_and_elaborate_expression(source, variable) {
            Ok(term) => term,
            Err(diagnostic) => {
                return NormalizeMathExpressionResponseDto {
                    outcome: MathematicalOutcomeKindDto::Unknown,
                    normalized: None,
                    diagnostics: vec![diagnostic],
                };
            }
        };

        match PolynomialNormalizer::normalize(&term, variable) {
            MathematicalOutcome::Proven(result) => NormalizeMathExpressionResponseDto {
                outcome: MathematicalOutcomeKindDto::Proven,
                normalized: Some(MathExpressionDto {
                    latex: LatexRenderer::polynomial_expression(&result.value.normal_form),
                }),
                diagnostics: Vec::new(),
            },
            MathematicalOutcome::Unknown(unknown) => NormalizeMathExpressionResponseDto {
                outcome: MathematicalOutcomeKindDto::Unknown,
                normalized: None,
                diagnostics: vec![DiagnosticDto {
                    code: unknown_reason_code(&unknown.reason),
                    message: "expression is outside the supported linear rational slice".to_owned(),
                }],
            },
            MathematicalOutcome::Undefined(undefined) => NormalizeMathExpressionResponseDto {
                outcome: MathematicalOutcomeKindDto::Undefined,
                normalized: None,
                diagnostics: vec![DiagnosticDto {
                    code: "Undefined".to_owned(),
                    message: undefined.reason,
                }],
            },
            MathematicalOutcome::Disproven(_) | MathematicalOutcome::Conditional(_) => {
                NormalizeMathExpressionResponseDto {
                    outcome: MathematicalOutcomeKindDto::Unknown,
                    normalized: None,
                    diagnostics: vec![DiagnosticDto {
                        code: "UnsupportedOutcome".to_owned(),
                        message: "normalizer returned an unsupported outcome for this API"
                            .to_owned(),
                    }],
                }
            }
        }
    }

    pub fn compare_math_expressions(
        left_source: &str,
        right_source: &str,
        input_format: &str,
        variable: &str,
    ) -> CompareMathExpressionsResponseDto {
        let left = match normalize_expression_value(left_source, input_format, variable) {
            Ok(value) => value,
            Err(response) => return response,
        };
        let right = match normalize_expression_value(right_source, input_format, variable) {
            Ok(value) => value,
            Err(response) => return response,
        };
        let equal = left.normal_form == right.normal_form;

        CompareMathExpressionsResponseDto {
            outcome: if equal {
                MathematicalOutcomeKindDto::Proven
            } else {
                MathematicalOutcomeKindDto::Disproven
            },
            relation: "expression.equivalent".to_owned(),
            equal: Some(equal),
            left_normalized: Some(MathExpressionDto {
                latex: LatexRenderer::polynomial_expression(&left.normal_form),
            }),
            right_normalized: Some(MathExpressionDto {
                latex: LatexRenderer::polynomial_expression(&right.normal_form),
            }),
            diagnostics: if equal {
                Vec::new()
            } else {
                vec![DiagnosticDto {
                    code: "Expression.NotEquivalent".to_owned(),
                    message: "expressions have different supported normal forms".to_owned(),
                }]
            },
        }
    }

    pub fn compare_numeric_answer(
        submitted_source: &str,
        expected_source: &str,
        input_format: &str,
        tolerance: f64,
    ) -> CompareNumericAnswerResponseDto {
        if tolerance < 0.0 {
            return CompareNumericAnswerResponseDto {
                outcome: MathematicalOutcomeKindDto::Undefined,
                relation: "number.within_tolerance".to_owned(),
                equal: None,
                submitted_value: None,
                expected_value: None,
                absolute_error: None,
                tolerance,
                diagnostics: vec![DiagnosticDto {
                    code: "Tolerance.Negative".to_owned(),
                    message: "numeric answer tolerance must be non-negative".to_owned(),
                }],
            };
        }

        let submitted = match normalize_numeric_value(submitted_source, input_format) {
            Ok(value) => value,
            Err(diagnostic) => return numeric_error(tolerance, diagnostic),
        };
        let expected = match normalize_numeric_value(expected_source, input_format) {
            Ok(value) => value,
            Err(diagnostic) => return numeric_error(tolerance, diagnostic),
        };
        let absolute_error = (submitted - expected).abs();
        let equal = absolute_error <= tolerance;

        CompareNumericAnswerResponseDto {
            outcome: if equal {
                MathematicalOutcomeKindDto::Proven
            } else {
                MathematicalOutcomeKindDto::Disproven
            },
            relation: "number.within_tolerance".to_owned(),
            equal: Some(equal),
            submitted_value: Some(submitted),
            expected_value: Some(expected),
            absolute_error: Some(absolute_error),
            tolerance,
            diagnostics: if equal {
                Vec::new()
            } else {
                vec![DiagnosticDto {
                    code: "Number.OutsideTolerance".to_owned(),
                    message: "numeric answer is outside the allowed tolerance".to_owned(),
                }]
            },
        }
    }

    pub fn differentiate_math_expression(
        source: &str,
        input_format: &str,
        variable: &str,
    ) -> TransformMathExpressionResponseDto {
        transform_polynomial_expression(
            source,
            input_format,
            variable,
            "calculus.derivative",
            "calculus.polynomial.derivative.power-rule",
            "Apply the polynomial derivative power rule term-by-term: d/dx a*x^n = a*n*x^(n-1).",
            PolynomialExpression::derivative,
        )
    }

    pub fn integrate_math_expression(
        source: &str,
        input_format: &str,
        variable: &str,
    ) -> TransformMathExpressionResponseDto {
        transform_polynomial_expression(
            source,
            input_format,
            variable,
            "calculus.antiderivative",
            "calculus.polynomial.integral.power-rule",
            "Apply the polynomial antiderivative power rule term-by-term: integral a*x^n dx = a/(n+1)*x^(n+1).",
            PolynomialExpression::antiderivative,
        )
    }

    pub fn solve_linear_equation(source: &str, variable: &str) -> SolveLinearEquationResponseDto {
        let judgment = match parse_and_elaborate_statement(source, variable) {
            Ok(judgment) => judgment,
            Err(diagnostic) => {
                return SolveLinearEquationResponseDto {
                    outcome: MathematicalOutcomeKindDto::Unknown,
                    variable: variable.to_owned(),
                    solution_set: None,
                    solution_set_latex: None,
                    completeness: None,
                    diagnostics: vec![diagnostic],
                };
            }
        };

        match LinearEquationSolver::solve(&judgment, variable) {
            MathematicalOutcome::Proven(result) => SolveLinearEquationResponseDto {
                outcome: MathematicalOutcomeKindDto::Proven,
                variable: result.value.variable.clone(),
                solution_set: Some(SolutionSetDto::from(&result.value.solution_set)),
                solution_set_latex: Some(LatexRenderer::solution_set(
                    &result.value.variable,
                    &result.value.solution_set,
                )),
                completeness: Some(format!("{:?}", result.value.completeness)),
                diagnostics: Vec::new(),
            },
            MathematicalOutcome::Unknown(unknown) => SolveLinearEquationResponseDto {
                outcome: MathematicalOutcomeKindDto::Unknown,
                variable: variable.to_owned(),
                solution_set: None,
                solution_set_latex: None,
                completeness: None,
                diagnostics: vec![DiagnosticDto {
                    code: unknown_reason_code(&unknown.reason),
                    message: "equation is outside the supported linear rational slice".to_owned(),
                }],
            },
            MathematicalOutcome::Undefined(undefined) => SolveLinearEquationResponseDto {
                outcome: MathematicalOutcomeKindDto::Undefined,
                variable: variable.to_owned(),
                solution_set: None,
                solution_set_latex: None,
                completeness: None,
                diagnostics: vec![DiagnosticDto {
                    code: "Undefined".to_owned(),
                    message: undefined.reason,
                }],
            },
            MathematicalOutcome::Disproven(_) | MathematicalOutcome::Conditional(_) => {
                SolveLinearEquationResponseDto {
                    outcome: MathematicalOutcomeKindDto::Unknown,
                    variable: variable.to_owned(),
                    solution_set: None,
                    solution_set_latex: None,
                    completeness: None,
                    diagnostics: vec![DiagnosticDto {
                        code: "UnsupportedOutcome".to_owned(),
                        message: "solver returned an unsupported outcome for this API".to_owned(),
                    }],
                }
            }
        }
    }

    pub fn compare_equation_solution_sets(
        left_source: &str,
        right_source: &str,
        variable: &str,
    ) -> CompareEquationsResponseDto {
        let left = match parse_and_elaborate_statement(left_source, variable) {
            Ok(judgment) => judgment,
            Err(diagnostic) => return comparison_error(variable, diagnostic),
        };
        let right = match parse_and_elaborate_statement(right_source, variable) {
            Ok(judgment) => judgment,
            Err(diagnostic) => return comparison_error(variable, diagnostic),
        };

        match EquationComparator::compare_solution_sets(&left, &right, variable) {
            MathematicalOutcome::Proven(result) => CompareEquationsResponseDto {
                outcome: MathematicalOutcomeKindDto::Proven,
                relation: result.value.relation,
                equal: Some(true),
                left_solution_set: Some(SolutionSetDto::from(&result.value.left_solution_set)),
                right_solution_set: Some(SolutionSetDto::from(&result.value.right_solution_set)),
                left_solution_set_latex: Some(LatexRenderer::solution_set(
                    &result.value.variable,
                    &result.value.left_solution_set,
                )),
                right_solution_set_latex: Some(LatexRenderer::solution_set(
                    &result.value.variable,
                    &result.value.right_solution_set,
                )),
                diagnostics: Vec::new(),
            },
            MathematicalOutcome::Disproven(disproven) => CompareEquationsResponseDto {
                outcome: MathematicalOutcomeKindDto::Disproven,
                relation: "equation.same_solution_set".to_owned(),
                equal: Some(false),
                left_solution_set: None,
                right_solution_set: None,
                left_solution_set_latex: None,
                right_solution_set_latex: None,
                diagnostics: vec![DiagnosticDto {
                    code: "DifferentSolutionSets".to_owned(),
                    message: disproven.reason,
                }],
            },
            MathematicalOutcome::Unknown(unknown) => CompareEquationsResponseDto {
                outcome: MathematicalOutcomeKindDto::Unknown,
                relation: "equation.same_solution_set".to_owned(),
                equal: None,
                left_solution_set: None,
                right_solution_set: None,
                left_solution_set_latex: None,
                right_solution_set_latex: None,
                diagnostics: vec![DiagnosticDto {
                    code: unknown_reason_code(&unknown.reason),
                    message: "comparison is outside the supported linear rational slice".to_owned(),
                }],
            },
            MathematicalOutcome::Undefined(undefined) => CompareEquationsResponseDto {
                outcome: MathematicalOutcomeKindDto::Undefined,
                relation: "equation.same_solution_set".to_owned(),
                equal: None,
                left_solution_set: None,
                right_solution_set: None,
                left_solution_set_latex: None,
                right_solution_set_latex: None,
                diagnostics: vec![DiagnosticDto {
                    code: "Undefined".to_owned(),
                    message: undefined.reason,
                }],
            },
            MathematicalOutcome::Conditional(_) => CompareEquationsResponseDto {
                outcome: MathematicalOutcomeKindDto::Conditional,
                relation: "equation.same_solution_set".to_owned(),
                equal: None,
                left_solution_set: None,
                right_solution_set: None,
                left_solution_set_latex: None,
                right_solution_set_latex: None,
                diagnostics: Vec::new(),
            },
        }
    }
}

fn transform_polynomial_expression(
    source: &str,
    input_format: &str,
    variable: &str,
    relation: &str,
    rule: &str,
    reason: &str,
    transform: fn(&PolynomialExpression) -> PolynomialExpression,
) -> TransformMathExpressionResponseDto {
    let input = match normalize_expression_value(source, input_format, variable) {
        Ok(value) => value,
        Err(response) => {
            return TransformMathExpressionResponseDto {
                outcome: response.outcome,
                relation: relation.to_owned(),
                result: None,
                steps: Vec::new(),
                diagnostics: response.diagnostics,
            };
        }
    };
    let result = transform(&input.normal_form);
    let input_latex = LatexRenderer::polynomial_expression(&input.normal_form);
    let output_latex = LatexRenderer::polynomial_expression(&result);

    TransformMathExpressionResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        relation: relation.to_owned(),
        result: Some(MathExpressionDto {
            latex: output_latex.clone(),
        }),
        steps: vec![MathDerivationStepDto {
            rule: rule.to_owned(),
            reason: reason.to_owned(),
            input_latex: Some(input_latex),
            output_latex: Some(output_latex),
        }],
        diagnostics: Vec::new(),
    }
}

fn normalize_numeric_value(source: &str, input_format: &str) -> Result<f64, DiagnosticDto> {
    let variable = "x";
    let term = parse_and_elaborate_expression(source, variable)?;

    let normal_form = match LinearNormalizer::normalize(&term, variable) {
        MathematicalOutcome::Proven(result) => result.value.normal_form,
        MathematicalOutcome::Unknown(unknown) => {
            return Err(DiagnosticDto {
                code: unknown_reason_code(&unknown.reason),
                message: "numeric answer is outside the supported rational slice".to_owned(),
            });
        }
        _ => {
            return Err(DiagnosticDto {
                code: "UnsupportedOutcome".to_owned(),
                message: "numeric answer normalization returned an unsupported outcome".to_owned(),
            });
        }
    };

    if !input_format.eq_ignore_ascii_case("latex") {
        return Err(DiagnosticDto {
            code: "Input.UnsupportedFormat".to_owned(),
            message: "only latex input is currently supported".to_owned(),
        });
    }

    let Some(value) = normal_form.as_constant() else {
        return Err(DiagnosticDto {
            code: "Number.ExpectedConstant".to_owned(),
            message: "numeric answers must normalize to a constant".to_owned(),
        });
    };

    exact_rational_to_f64(value)
}

fn exact_rational_to_f64(value: &socrates_math_core::ExactRational) -> Result<f64, DiagnosticDto> {
    let numerator = value
        .numerator()
        .to_string()
        .parse::<f64>()
        .map_err(|_| DiagnosticDto {
            code: "Number.OutOfRange".to_owned(),
            message: "numeric answer is too large to compare as f64".to_owned(),
        })?;
    let denominator = value
        .denominator()
        .to_string()
        .parse::<f64>()
        .map_err(|_| DiagnosticDto {
            code: "Number.OutOfRange".to_owned(),
            message: "numeric answer is too large to compare as f64".to_owned(),
        })?;

    Ok(numerator / denominator)
}

fn numeric_error(tolerance: f64, diagnostic: DiagnosticDto) -> CompareNumericAnswerResponseDto {
    CompareNumericAnswerResponseDto {
        outcome: MathematicalOutcomeKindDto::Unknown,
        relation: "number.within_tolerance".to_owned(),
        equal: None,
        submitted_value: None,
        expected_value: None,
        absolute_error: None,
        tolerance,
        diagnostics: vec![diagnostic],
    }
}

struct NormalizedExpressionValue {
    normal_form: PolynomialExpression,
}

fn normalize_expression_value(
    source: &str,
    input_format: &str,
    variable: &str,
) -> Result<NormalizedExpressionValue, CompareMathExpressionsResponseDto> {
    let response = MathEngine::normalize_math_expression(source, input_format, variable);

    if response.outcome != MathematicalOutcomeKindDto::Proven {
        return Err(CompareMathExpressionsResponseDto {
            outcome: response.outcome,
            relation: "expression.equivalent".to_owned(),
            equal: None,
            left_normalized: None,
            right_normalized: None,
            diagnostics: response.diagnostics,
        });
    }

    let term = parse_and_elaborate_expression(source, variable).map_err(|diagnostic| {
        CompareMathExpressionsResponseDto {
            outcome: MathematicalOutcomeKindDto::Unknown,
            relation: "expression.equivalent".to_owned(),
            equal: None,
            left_normalized: None,
            right_normalized: None,
            diagnostics: vec![diagnostic],
        }
    })?;

    match PolynomialNormalizer::normalize(&term, variable) {
        MathematicalOutcome::Proven(result) => Ok(NormalizedExpressionValue {
            normal_form: result.value.normal_form,
        }),
        _ => Err(CompareMathExpressionsResponseDto {
            outcome: MathematicalOutcomeKindDto::Unknown,
            relation: "expression.equivalent".to_owned(),
            equal: None,
            left_normalized: None,
            right_normalized: None,
            diagnostics: response.diagnostics,
        }),
    }
}

fn parse_and_elaborate_statement(source: &str, variable: &str) -> Result<Judgment, DiagnosticDto> {
    let statement = match Parser::parse_statement(source) {
        ParseOutcome::Parsed(statement) => statement,
        ParseOutcome::Incomplete(diagnostic) | ParseOutcome::Rejected(diagnostic) => {
            return Err(DiagnosticDto {
                code: parse_code(diagnostic.code),
                message: diagnostic.summary,
            });
        }
    };

    let context = rational_context(variable);

    match Elaborator::elaborate_statement(&statement, &context) {
        ElaborationOutcome::Elaborated(judgment) => Ok(judgment),
        ElaborationOutcome::Rejected(diagnostic) => Err(DiagnosticDto {
            code: elaboration_code(diagnostic.code),
            message: diagnostic.summary,
        }),
    }
}

fn parse_and_elaborate_expression(
    source: &str,
    variable: &str,
) -> Result<SemanticTerm, DiagnosticDto> {
    let expression = match Parser::parse_expression(source) {
        ParseOutcome::Parsed(expression) => expression,
        ParseOutcome::Incomplete(diagnostic) | ParseOutcome::Rejected(diagnostic) => {
            return Err(DiagnosticDto {
                code: parse_code(diagnostic.code),
                message: diagnostic.summary,
            });
        }
    };

    let context = rational_context(variable);

    match Elaborator::elaborate_expression(&expression, &context) {
        ElaborationOutcome::Elaborated(term) => Ok(term),
        ElaborationOutcome::Rejected(diagnostic) => Err(DiagnosticDto {
            code: elaboration_code(diagnostic.code),
            message: diagnostic.summary,
        }),
    }
}

fn rational_context(variable: &str) -> Context {
    Context::root().with_declaration(Declaration {
        name: variable.to_owned(),
        type_id: TypeId::new("core.rational.rational").expect("static type id is valid"),
    })
}

fn comparison_error(variable: &str, diagnostic: DiagnosticDto) -> CompareEquationsResponseDto {
    let _ = variable;
    CompareEquationsResponseDto {
        outcome: MathematicalOutcomeKindDto::Unknown,
        relation: "equation.same_solution_set".to_owned(),
        equal: None,
        left_solution_set: None,
        right_solution_set: None,
        left_solution_set_latex: None,
        right_solution_set_latex: None,
        diagnostics: vec![diagnostic],
    }
}

fn parse_code(code: DiagnosticCode) -> String {
    format!("Parse.{code:?}")
}

fn elaboration_code(code: ElaborationDiagnosticCode) -> String {
    format!("Elaboration.{code:?}")
}

fn unknown_reason_code(reason: &UnknownReason) -> String {
    format!("Unknown.{reason:?}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use socrates_math_protocol::{ExactValueDto, SolutionSetDto};

    #[test]
    fn solves_linear_equation_through_app_facade() {
        let result = MathEngine::solve_linear_equation("3(x - 2) + 4 = 2x + 9", "x");

        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
        assert_eq!(
            result.solution_set,
            Some(SolutionSetDto::Unique {
                value: ExactValueDto::Integer {
                    value: "11".to_owned()
                }
            })
        );
    }

    #[test]
    fn compares_equivalent_equations_through_app_facade() {
        let result = MathEngine::compare_equation_solution_sets("x + 1 = 3", "x = 2", "x");

        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
        assert_eq!(result.equal, Some(true));
    }

    #[test]
    fn reports_unknown_for_unsupported_equation_through_app_facade() {
        let result = MathEngine::solve_linear_equation("x x = 2", "x");

        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
        assert_eq!(result.solution_set, None);
    }
}
