use socrates_math_algebra::{LinearNormalizer, PolynomialExpression, PolynomialNormalizer};
use socrates_math_compare::EquationComparator;
use socrates_math_core::{
    Context, Declaration, ExactRational, Judgment, MathematicalOutcome, SemanticTerm, TypeId,
    UnknownReason,
};
use socrates_math_elab::{ElaborationDiagnosticCode, ElaborationOutcome, Elaborator};
use socrates_math_protocol::{
    ApplicableRuleDto, ApplyRuleResponseDto, CompareEquationsResponseDto,
    CompareMathExpressionsResponseDto, CompareNumericAnswerResponseDto,
    CompareSetExpressionsResponseDto, DiagnosticDto, EvaluateFiniteRelationPredicateResponseDto,
    EvaluateSetCardinalityResponseDto, EvaluateSetStatementResponseDto,
    ListApplicableRulesResponseDto, MathDerivationStepDto, MathExpressionDto,
    MathematicalOutcomeKindDto, NormalizeMathExpressionResponseDto,
    NormalizeSetExpressionResponseDto, RuleApplicabilityStatusDto, RuleTargetDto, SetBindingDto,
    SetExpressionDto, SetStatementDto, SolutionSetDto, SolveLinearEquationResponseDto,
    TransformMathExpressionResponseDto,
};
use socrates_math_render::LatexRenderer;
use socrates_math_solve::LinearEquationSolver;
use socrates_math_syntax::{DiagnosticCode, ParseOutcome, Parser};
use std::collections::{BTreeMap, BTreeSet};

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

    pub fn normalize_set_expression(
        source: &str,
        input_format: &str,
    ) -> NormalizeSetExpressionResponseDto {
        match normalize_finite_set_expression(source, input_format) {
            Ok(normalized) => NormalizeSetExpressionResponseDto {
                outcome: MathematicalOutcomeKindDto::Proven,
                normalized: Some(SetExpressionDto {
                    latex: normalized.to_latex(),
                }),
                diagnostics: Vec::new(),
            },
            Err(diagnostic) => NormalizeSetExpressionResponseDto {
                outcome: MathematicalOutcomeKindDto::Unknown,
                normalized: None,
                diagnostics: vec![diagnostic],
            },
        }
    }

    pub fn compare_set_expressions(
        left_source: &str,
        right_source: &str,
        input_format: &str,
    ) -> CompareSetExpressionsResponseDto {
        let left = match normalize_finite_set_expression(left_source, input_format) {
            Ok(value) => value,
            Err(diagnostic) => {
                return unknown_set_comparison(vec![diagnostic]);
            }
        };
        let right = match normalize_finite_set_expression(right_source, input_format) {
            Ok(value) => value,
            Err(diagnostic) => {
                return unknown_set_comparison(vec![diagnostic]);
            }
        };
        let equal = left == right;

        CompareSetExpressionsResponseDto {
            outcome: if equal {
                MathematicalOutcomeKindDto::Proven
            } else {
                MathematicalOutcomeKindDto::Disproven
            },
            relation: "set.extensional_equal".to_owned(),
            equal: Some(equal),
            left_normalized: Some(SetExpressionDto {
                latex: left.to_latex(),
            }),
            right_normalized: Some(SetExpressionDto {
                latex: right.to_latex(),
            }),
            diagnostics: if equal {
                Vec::new()
            } else {
                vec![DiagnosticDto {
                    code: "Set.NotExtensionallyEqual".to_owned(),
                    message: "sets have different normalized elements".to_owned(),
                }]
            },
        }
    }

    pub fn compare_set_expressions_in_context(
        left_source: &str,
        right_source: &str,
        universe_source: &str,
        bindings: &[SetBindingDto],
        input_format: &str,
    ) -> CompareSetExpressionsResponseDto {
        let context = match finite_set_context(universe_source, bindings, input_format) {
            Ok(value) => value,
            Err(diagnostic) => return unknown_contextual_set_comparison(vec![diagnostic]),
        };
        let left = match normalize_finite_set_expression_with_context(
            left_source,
            input_format,
            Some(&context),
        ) {
            Ok(value) => value,
            Err(diagnostic) => return unknown_contextual_set_comparison(vec![diagnostic]),
        };
        let right = match normalize_finite_set_expression_with_context(
            right_source,
            input_format,
            Some(&context),
        ) {
            Ok(value) => value,
            Err(diagnostic) => return unknown_contextual_set_comparison(vec![diagnostic]),
        };
        let equal = left == right;

        CompareSetExpressionsResponseDto {
            outcome: if equal {
                MathematicalOutcomeKindDto::Proven
            } else {
                MathematicalOutcomeKindDto::Disproven
            },
            relation: "set.extensional_equal.in_context".to_owned(),
            equal: Some(equal),
            left_normalized: Some(SetExpressionDto {
                latex: left.to_latex(),
            }),
            right_normalized: Some(SetExpressionDto {
                latex: right.to_latex(),
            }),
            diagnostics: if equal {
                Vec::new()
            } else {
                vec![DiagnosticDto {
                    code: "Set.NotExtensionallyEqualInContext".to_owned(),
                    message: "sets have different normalized elements in the declared universe"
                        .to_owned(),
                }]
            },
        }
    }

    pub fn evaluate_set_statement(
        source: &str,
        input_format: &str,
    ) -> EvaluateSetStatementResponseDto {
        match evaluate_finite_set_statement(source, input_format) {
            Ok(result) => EvaluateSetStatementResponseDto {
                outcome: MathematicalOutcomeKindDto::Proven,
                relation: "logic.truth".to_owned(),
                truth: Some(result.truth),
                normalized: Some(SetStatementDto {
                    latex: result.normalized_latex,
                }),
                diagnostics: Vec::new(),
            },
            Err(diagnostic) => EvaluateSetStatementResponseDto {
                outcome: MathematicalOutcomeKindDto::Unknown,
                relation: "logic.truth".to_owned(),
                truth: None,
                normalized: None,
                diagnostics: vec![diagnostic],
            },
        }
    }

    pub fn evaluate_set_cardinality(
        source: &str,
        input_format: &str,
    ) -> EvaluateSetCardinalityResponseDto {
        match normalize_finite_set_expression(source, input_format) {
            Ok(set) => {
                let cardinality = set.elements.len() as u64;

                EvaluateSetCardinalityResponseDto {
                    outcome: MathematicalOutcomeKindDto::Proven,
                    relation: "set.cardinality".to_owned(),
                    cardinality: Some(cardinality),
                    cardinality_latex: Some(cardinality.to_string()),
                    normalized_set: Some(SetExpressionDto {
                        latex: set.to_latex(),
                    }),
                    diagnostics: Vec::new(),
                }
            }
            Err(diagnostic) => EvaluateSetCardinalityResponseDto {
                outcome: MathematicalOutcomeKindDto::Unknown,
                relation: "set.cardinality".to_owned(),
                cardinality: None,
                cardinality_latex: None,
                normalized_set: None,
                diagnostics: vec![diagnostic],
            },
        }
    }

    pub fn evaluate_relation_from(
        relation_source: &str,
        domain_source: &str,
        codomain_source: &str,
        input_format: &str,
    ) -> EvaluateFiniteRelationPredicateResponseDto {
        evaluate_finite_relation_predicate(
            relation_source,
            domain_source,
            codomain_source,
            input_format,
            "relation.from",
            finite_relation_is_from,
        )
    }

    pub fn evaluate_function_from(
        relation_source: &str,
        domain_source: &str,
        codomain_source: &str,
        input_format: &str,
    ) -> EvaluateFiniteRelationPredicateResponseDto {
        evaluate_finite_relation_predicate(
            relation_source,
            domain_source,
            codomain_source,
            input_format,
            "function.from",
            finite_relation_is_function_from,
        )
    }

    pub fn evaluate_relation_property(
        relation_source: &str,
        set_source: &str,
        property: &str,
        input_format: &str,
    ) -> EvaluateFiniteRelationPredicateResponseDto {
        let relation_name = match property {
            "reflexive" => "relation.reflexive",
            "symmetric" => "relation.symmetric",
            "antisymmetric" => "relation.antisymmetric",
            "transitive" => "relation.transitive",
            _ => {
                return EvaluateFiniteRelationPredicateResponseDto {
                    outcome: MathematicalOutcomeKindDto::Unknown,
                    relation: "relation.property".to_owned(),
                    truth: None,
                    normalized_relation: None,
                    normalized_domain: None,
                    normalized_codomain: None,
                    diagnostics: vec![DiagnosticDto {
                        code: "Relation.UnsupportedProperty".to_owned(),
                        message: "supported relation properties are reflexive, symmetric, antisymmetric, and transitive"
                            .to_owned(),
                    }],
                };
            }
        };

        evaluate_finite_relation_predicate(
            relation_source,
            set_source,
            set_source,
            input_format,
            relation_name,
            match property {
                "reflexive" => finite_relation_is_reflexive,
                "symmetric" => finite_relation_is_symmetric,
                "antisymmetric" => finite_relation_is_antisymmetric,
                "transitive" => finite_relation_is_transitive,
                _ => unreachable!("unsupported relation properties returned above"),
            },
        )
    }

    pub fn evaluate_relation_domain(
        relation_source: &str,
        input_format: &str,
    ) -> NormalizeSetExpressionResponseDto {
        evaluate_finite_relation_set_operation(
            relation_source,
            input_format,
            finite_relation_domain,
        )
    }

    pub fn evaluate_relation_range(
        relation_source: &str,
        input_format: &str,
    ) -> NormalizeSetExpressionResponseDto {
        evaluate_finite_relation_set_operation(relation_source, input_format, finite_relation_range)
    }

    pub fn evaluate_relation_inverse(
        relation_source: &str,
        input_format: &str,
    ) -> NormalizeSetExpressionResponseDto {
        evaluate_finite_relation_set_operation(
            relation_source,
            input_format,
            finite_relation_inverse,
        )
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
        let polynomial_response = transform_polynomial_expression(
            source,
            input_format,
            variable,
            "calculus.derivative",
            PolynomialExpression::derivative,
            derivative_steps,
        );

        if polynomial_response.outcome != MathematicalOutcomeKindDto::Unknown {
            return polynomial_response;
        }

        transform_rational_power_monomial(
            source,
            input_format,
            variable,
            CalculusOperation::Derivative,
        )
        .unwrap_or(polynomial_response)
    }

    pub fn integrate_math_expression(
        source: &str,
        input_format: &str,
        variable: &str,
    ) -> TransformMathExpressionResponseDto {
        let polynomial_response = transform_polynomial_expression(
            source,
            input_format,
            variable,
            "calculus.antiderivative",
            PolynomialExpression::antiderivative,
            antiderivative_steps,
        );

        if polynomial_response.outcome != MathematicalOutcomeKindDto::Unknown {
            return polynomial_response;
        }

        transform_rational_power_monomial(
            source,
            input_format,
            variable,
            CalculusOperation::Antiderivative,
        )
        .unwrap_or(polynomial_response)
    }

    pub fn list_applicable_math_expression_rules(
        source: &str,
        input_format: &str,
        variable: &str,
        target: Option<RuleTargetDto>,
    ) -> ListApplicableRulesResponseDto {
        let input = match normalize_expression_value(source, input_format, variable) {
            Ok(value) => value,
            Err(response) => {
                if let Some(response) =
                    list_rational_power_monomial_rules(source, input_format, variable, target)
                {
                    return response;
                }

                return ListApplicableRulesResponseDto {
                    outcome: response.outcome,
                    rules: Vec::new(),
                    diagnostics: response.diagnostics,
                };
            }
        };

        let rules = match target {
            Some(RuleTargetDto::PolynomialTerm { degree }) => {
                rules_for_polynomial_term(&input.normal_form, degree)
            }
            Some(RuleTargetDto::Whole) => match unique_polynomial_term_degree(&input.normal_form) {
                Some(degree) => rules_for_polynomial_term(&input.normal_form, degree),
                None => whole_polynomial_rules(&input.normal_form),
            },
            None => input
                .normal_form
                .coefficients
                .keys()
                .rev()
                .flat_map(|degree| rules_for_polynomial_term(&input.normal_form, *degree))
                .collect(),
        };

        ListApplicableRulesResponseDto {
            outcome: MathematicalOutcomeKindDto::Proven,
            rules,
            diagnostics: Vec::new(),
        }
    }

    pub fn apply_math_expression_rule(
        source: &str,
        input_format: &str,
        variable: &str,
        rule: &str,
        target: Option<RuleTargetDto>,
    ) -> ApplyRuleResponseDto {
        let input = match normalize_expression_value(source, input_format, variable) {
            Ok(value) => value,
            Err(response) => {
                if let Some(response) =
                    apply_rational_power_monomial_rule(source, input_format, variable, rule, target)
                {
                    return response;
                }

                return ApplyRuleResponseDto {
                    outcome: response.outcome,
                    relation: "rule.application".to_owned(),
                    previous: None,
                    result: None,
                    step: None,
                    diagnostics: response.diagnostics,
                };
            }
        };

        if matches!(target, Some(RuleTargetDto::Whole))
            && let Some(response) = apply_whole_polynomial_rule(&input.normal_form, rule)
        {
            return response;
        }

        let degree = match resolve_polynomial_rule_target(&input.normal_form, target) {
            Ok(degree) => degree,
            Err(diagnostic) => {
                return ApplyRuleResponseDto {
                    outcome: MathematicalOutcomeKindDto::Unknown,
                    relation: "rule.application".to_owned(),
                    previous: Some(MathExpressionDto {
                        latex: LatexRenderer::polynomial_expression(&input.normal_form),
                    }),
                    result: None,
                    step: None,
                    diagnostics: vec![diagnostic],
                };
            }
        };

        apply_polynomial_calculus_rule(&input.normal_form, rule, degree)
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
    transform: fn(&PolynomialExpression) -> PolynomialExpression,
    steps: fn(&PolynomialExpression) -> Vec<MathDerivationStepDto>,
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
    let output_latex = LatexRenderer::polynomial_expression(&result);

    TransformMathExpressionResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        relation: relation.to_owned(),
        result: Some(MathExpressionDto {
            latex: output_latex.clone(),
        }),
        steps: steps(&input.normal_form),
        diagnostics: Vec::new(),
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FiniteSetExpression {
    elements: BTreeSet<FiniteSetElement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FiniteSetContext {
    universe: FiniteSetExpression,
    bindings: BTreeMap<String, FiniteSetExpression>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum FiniteSetElement {
    Atom(String),
    Set(FiniteSetExpression),
    OrderedPair(Box<FiniteSetElement>, Box<FiniteSetElement>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FiniteSetBinaryOperator {
    Union,
    Intersection,
    Difference,
    CartesianProduct,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FiniteSetStatementEvaluation {
    truth: bool,
    normalized_latex: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FiniteSetStatementOperator {
    In,
    NotIn,
    SubsetEq,
    NotSubsetEq,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FiniteSetAtomComparisonOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl FiniteSetExpression {
    fn empty() -> Self {
        Self {
            elements: BTreeSet::new(),
        }
    }

    fn to_latex(&self) -> String {
        if self.elements.is_empty() {
            return "\\varnothing".to_owned();
        }

        format!(
            "\\{{{}\\}}",
            self.elements
                .iter()
                .map(FiniteSetElement::to_latex)
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl FiniteSetElement {
    fn to_latex(&self) -> String {
        match self {
            Self::Atom(value) => value.clone(),
            Self::Set(value) => value.to_latex(),
            Self::OrderedPair(left, right) => {
                format!("({},{})", left.to_latex(), right.to_latex())
            }
        }
    }
}

fn normalize_finite_set_expression(
    source: &str,
    input_format: &str,
) -> Result<FiniteSetExpression, DiagnosticDto> {
    normalize_finite_set_expression_with_context(source, input_format, None)
}

fn normalize_finite_set_expression_with_context(
    source: &str,
    input_format: &str,
    context: Option<&FiniteSetContext>,
) -> Result<FiniteSetExpression, DiagnosticDto> {
    if !input_format.eq_ignore_ascii_case("latex") {
        return Err(DiagnosticDto {
            code: "Set.UnsupportedInputFormat".to_owned(),
            message: "set expressions currently require LaTeX input".to_owned(),
        });
    }

    let normalized_source = source.trim();
    let (set, rest) = parse_finite_set_union_expression(normalized_source, context)?;

    if !rest.trim().is_empty() {
        return Err(DiagnosticDto {
            code: "Set.TrailingInput".to_owned(),
            message: "unexpected input after finite set expression".to_owned(),
        });
    }

    Ok(set)
}

fn parse_finite_set_union_expression<'a>(
    source: &'a str,
    context: Option<&FiniteSetContext>,
) -> Result<(FiniteSetExpression, &'a str), DiagnosticDto> {
    let (mut left, mut rest) = parse_finite_set_intersection_expression(source, context)?;

    loop {
        rest = rest.trim_start();

        let Some(after_operator) = rest.strip_prefix("\\cup") else {
            return Ok((left, rest));
        };

        let (right, after_right) =
            parse_finite_set_intersection_expression(after_operator, context)?;
        left = apply_finite_set_binary_operator(left, right, FiniteSetBinaryOperator::Union);
        rest = after_right;
    }
}

fn parse_finite_set_intersection_expression<'a>(
    source: &'a str,
    context: Option<&FiniteSetContext>,
) -> Result<(FiniteSetExpression, &'a str), DiagnosticDto> {
    let (mut left, mut rest) = parse_finite_set_product_expression(source, context)?;

    loop {
        rest = rest.trim_start();

        let Some(after_operator) = rest.strip_prefix("\\cap") else {
            return Ok((left, rest));
        };

        let (right, after_right) = parse_finite_set_product_expression(after_operator, context)?;
        left = apply_finite_set_binary_operator(left, right, FiniteSetBinaryOperator::Intersection);
        rest = after_right;
    }
}

fn parse_finite_set_product_expression<'a>(
    source: &'a str,
    context: Option<&FiniteSetContext>,
) -> Result<(FiniteSetExpression, &'a str), DiagnosticDto> {
    let (mut left, mut rest) = parse_finite_set_difference_expression(source, context)?;

    loop {
        rest = rest.trim_start();

        let after_operator = rest
            .strip_prefix("\\times")
            .or_else(|| rest.strip_prefix("\\cartesianproduct"));
        let Some(after_operator) = after_operator else {
            return Ok((left, rest));
        };

        let (right, after_right) = parse_finite_set_difference_expression(after_operator, context)?;
        left = apply_finite_set_binary_operator(
            left,
            right,
            FiniteSetBinaryOperator::CartesianProduct,
        );
        rest = after_right;
    }
}

fn parse_finite_set_difference_expression<'a>(
    source: &'a str,
    context: Option<&FiniteSetContext>,
) -> Result<(FiniteSetExpression, &'a str), DiagnosticDto> {
    let (mut left, mut rest) = parse_finite_set_primary_expression(source, context)?;

    loop {
        rest = rest.trim_start();

        let after_operator = rest
            .strip_prefix("\\setminus")
            .or_else(|| rest.strip_prefix("\\backslash"));
        let Some(after_operator) = after_operator else {
            return Ok((left, rest));
        };

        let (right, after_right) =
            parse_finite_set_primary_expression(source_after_whitespace(after_operator), context)?;
        left = apply_finite_set_binary_operator(left, right, FiniteSetBinaryOperator::Difference);
        rest = after_right;
    }
}

fn parse_finite_set_primary_expression<'a>(
    source: &'a str,
    context: Option<&FiniteSetContext>,
) -> Result<(FiniteSetExpression, &'a str), DiagnosticDto> {
    let source = source.trim_start();

    let (mut set, mut rest) = if let Some(after_command) = source.strip_prefix("\\mathcal{P}") {
        let (argument, rest) = extract_parenthesized_argument(after_command)?;
        let (set, argument_rest) = parse_finite_set_union_expression(argument, context)?;

        if !argument_rest.trim().is_empty() {
            return Err(DiagnosticDto {
                code: "Set.TrailingInput".to_owned(),
                message: "unexpected input inside power set argument".to_owned(),
            });
        }

        (finite_power_set(&set)?, rest)
    } else if source.starts_with("\\left(") || source.starts_with('(') {
        let (argument, rest) = extract_parenthesized_argument(source)?;
        let (set, argument_rest) = parse_finite_set_union_expression(argument, context)?;

        if !argument_rest.trim().is_empty() {
            return Err(DiagnosticDto {
                code: "Set.TrailingInput".to_owned(),
                message: "unexpected input inside grouped set expression".to_owned(),
            });
        }

        (set, rest)
    } else if let Some((symbol, after_symbol)) = parse_set_expression_symbol(source) {
        let Some(context) = context else {
            return parse_finite_set(source, context);
        };
        let Some(bound_set) = context.bindings.get(symbol) else {
            return Err(DiagnosticDto {
                code: "Set.UnknownSymbol".to_owned(),
                message: format!("unknown finite set symbol '{symbol}'"),
            });
        };

        (bound_set.clone(), after_symbol)
    } else {
        parse_finite_set(source, context)?
    };

    loop {
        let after_space = rest.trim_start();
        let after_complement = after_space
            .strip_prefix("^{c}")
            .or_else(|| after_space.strip_prefix("^c"))
            .or_else(|| after_space.strip_prefix("'"))
            .or_else(|| after_space.strip_prefix("^\\complement"))
            .or_else(|| after_space.strip_prefix("^{\\complement}"));
        let Some(after_complement) = after_complement else {
            return Ok((set, rest));
        };
        let Some(context) = context else {
            return Err(DiagnosticDto {
                code: "Set.ComplementRequiresUniverse".to_owned(),
                message: "set complements require an explicit finite universe".to_owned(),
            });
        };

        set = finite_relative_complement(&context.universe, &set)?;
        rest = after_complement;
    }
}

fn extract_parenthesized_argument(source: &str) -> Result<(&str, &str), DiagnosticDto> {
    let source = source.trim_start();

    if let Some(after_open) = source.strip_prefix("\\left(") {
        return extract_until_right_parenthesis(after_open);
    }

    let Some(after_open) = source.strip_prefix('(') else {
        return Err(DiagnosticDto {
            code: "Set.ExpectedPowerSetArgument".to_owned(),
            message: "expected a parenthesized set expression after \\mathcal{P}".to_owned(),
        });
    };

    extract_until_plain_parenthesis(after_open)
}

fn extract_until_plain_parenthesis(source: &str) -> Result<(&str, &str), DiagnosticDto> {
    let mut depth = 1_i32;

    for (index, character) in source.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    let argument = &source[..index];
                    let rest = &source[index + character.len_utf8()..];
                    return Ok((argument, rest));
                }
            }
            _ => {}
        }
    }

    Err(DiagnosticDto {
        code: "Set.ExpectedRightParenthesis".to_owned(),
        message: "expected ')' to close the power set argument".to_owned(),
    })
}

fn extract_until_right_parenthesis(source: &str) -> Result<(&str, &str), DiagnosticDto> {
    let Some(index) = source.find("\\right)") else {
        return Err(DiagnosticDto {
            code: "Set.ExpectedRightParenthesis".to_owned(),
            message: "expected \\right) to close the power set argument".to_owned(),
        });
    };

    Ok((&source[..index], &source[index + "\\right)".len()..]))
}

fn finite_power_set(set: &FiniteSetExpression) -> Result<FiniteSetExpression, DiagnosticDto> {
    let elements = set.elements.iter().cloned().collect::<Vec<_>>();

    if elements.len() > 20 {
        return Err(DiagnosticDto {
            code: "Set.PowerSetTooLarge".to_owned(),
            message: "power set expansion is limited to sets with at most 20 elements".to_owned(),
        });
    }

    let mut subsets = BTreeSet::new();

    for mask in 0_usize..(1_usize << elements.len()) {
        let mut subset_elements = BTreeSet::new();

        for (index, element) in elements.iter().enumerate() {
            if (mask & (1_usize << index)) != 0 {
                subset_elements.insert(element.clone());
            }
        }

        subsets.insert(FiniteSetElement::Set(FiniteSetExpression {
            elements: subset_elements,
        }));
    }

    Ok(FiniteSetExpression { elements: subsets })
}

fn finite_set_context(
    universe_source: &str,
    bindings: &[SetBindingDto],
    input_format: &str,
) -> Result<FiniteSetContext, DiagnosticDto> {
    let universe = normalize_finite_set_expression(universe_source, input_format)?;
    let mut normalized_bindings = BTreeMap::new();
    normalized_bindings.insert("U".to_owned(), universe.clone());

    for binding in bindings {
        validate_set_symbol(&binding.symbol)?;
        if binding.symbol == "U" {
            return Err(DiagnosticDto {
                code: "Set.ReservedUniverseSymbol".to_owned(),
                message: "the symbol U is reserved for the declared universe".to_owned(),
            });
        }

        let set = normalize_finite_set_expression_with_context(
            &binding.expression,
            input_format,
            Some(&FiniteSetContext {
                universe: universe.clone(),
                bindings: normalized_bindings.clone(),
            }),
        )?;

        if !set.elements.is_subset(&universe.elements) {
            return Err(DiagnosticDto {
                code: "Set.BindingOutsideUniverse".to_owned(),
                message: format!(
                    "the finite set bound to '{}' contains elements outside the universe",
                    binding.symbol
                ),
            });
        }

        normalized_bindings.insert(binding.symbol.clone(), set);
    }

    Ok(FiniteSetContext {
        universe,
        bindings: normalized_bindings,
    })
}

fn validate_set_symbol(symbol: &str) -> Result<(), DiagnosticDto> {
    if parse_set_expression_symbol(symbol)
        .is_some_and(|(parsed, rest)| parsed == symbol && rest.is_empty())
    {
        return Ok(());
    }

    Err(DiagnosticDto {
        code: "Set.InvalidSymbol".to_owned(),
        message: "finite set symbols must start with a letter and contain only letters, digits, or underscores".to_owned(),
    })
}

fn finite_relative_complement(
    universe: &FiniteSetExpression,
    set: &FiniteSetExpression,
) -> Result<FiniteSetExpression, DiagnosticDto> {
    if !set.elements.is_subset(&universe.elements) {
        return Err(DiagnosticDto {
            code: "Set.ComplementOutsideUniverse".to_owned(),
            message: "cannot complement a set that is not a subset of the declared universe"
                .to_owned(),
        });
    }

    Ok(FiniteSetExpression {
        elements: universe
            .elements
            .difference(&set.elements)
            .cloned()
            .collect(),
    })
}

fn apply_finite_set_binary_operator(
    left: FiniteSetExpression,
    right: FiniteSetExpression,
    operator: FiniteSetBinaryOperator,
) -> FiniteSetExpression {
    let elements = match operator {
        FiniteSetBinaryOperator::Union => left.elements.union(&right.elements).cloned().collect(),
        FiniteSetBinaryOperator::Intersection => left
            .elements
            .intersection(&right.elements)
            .cloned()
            .collect(),
        FiniteSetBinaryOperator::Difference => {
            left.elements.difference(&right.elements).cloned().collect()
        }
        FiniteSetBinaryOperator::CartesianProduct => left
            .elements
            .iter()
            .flat_map(|left_element| {
                right.elements.iter().map(|right_element| {
                    FiniteSetElement::OrderedPair(
                        Box::new(left_element.clone()),
                        Box::new(right_element.clone()),
                    )
                })
            })
            .collect(),
    };

    FiniteSetExpression { elements }
}

fn evaluate_finite_relation_predicate(
    relation_source: &str,
    domain_source: &str,
    codomain_source: &str,
    input_format: &str,
    relation_name: &str,
    predicate: fn(
        &FiniteSetExpression,
        &FiniteSetExpression,
        &FiniteSetExpression,
    ) -> Result<bool, DiagnosticDto>,
) -> EvaluateFiniteRelationPredicateResponseDto {
    let relation = match normalize_finite_set_expression(relation_source, input_format) {
        Ok(value) => value,
        Err(diagnostic) => return unknown_finite_relation_predicate(relation_name, diagnostic),
    };
    let domain = match normalize_finite_set_expression(domain_source, input_format) {
        Ok(value) => value,
        Err(diagnostic) => return unknown_finite_relation_predicate(relation_name, diagnostic),
    };
    let codomain = match normalize_finite_set_expression(codomain_source, input_format) {
        Ok(value) => value,
        Err(diagnostic) => return unknown_finite_relation_predicate(relation_name, diagnostic),
    };
    let truth = match predicate(&relation, &domain, &codomain) {
        Ok(value) => value,
        Err(diagnostic) => return unknown_finite_relation_predicate(relation_name, diagnostic),
    };

    EvaluateFiniteRelationPredicateResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        relation: relation_name.to_owned(),
        truth: Some(truth),
        normalized_relation: Some(SetExpressionDto {
            latex: relation.to_latex(),
        }),
        normalized_domain: Some(SetExpressionDto {
            latex: domain.to_latex(),
        }),
        normalized_codomain: Some(SetExpressionDto {
            latex: codomain.to_latex(),
        }),
        diagnostics: Vec::new(),
    }
}

fn finite_relation_is_from(
    relation: &FiniteSetExpression,
    domain: &FiniteSetExpression,
    codomain: &FiniteSetExpression,
) -> Result<bool, DiagnosticDto> {
    let pairs = finite_relation_pairs(relation)?;

    Ok(pairs
        .iter()
        .all(|(left, right)| domain.elements.contains(left) && codomain.elements.contains(right)))
}

fn finite_relation_is_function_from(
    relation: &FiniteSetExpression,
    domain: &FiniteSetExpression,
    codomain: &FiniteSetExpression,
) -> Result<bool, DiagnosticDto> {
    let pairs = finite_relation_pairs(relation)?;

    if !pairs
        .iter()
        .all(|(left, right)| domain.elements.contains(left) && codomain.elements.contains(right))
    {
        return Ok(false);
    }

    for input in &domain.elements {
        let outputs = pairs
            .iter()
            .filter_map(|(left, right)| (left == input).then_some(right))
            .collect::<BTreeSet<_>>();

        if outputs.len() != 1 {
            return Ok(false);
        }
    }

    Ok(true)
}

fn finite_relation_is_reflexive(
    relation: &FiniteSetExpression,
    set: &FiniteSetExpression,
    _: &FiniteSetExpression,
) -> Result<bool, DiagnosticDto> {
    let pairs = finite_relation_pair_set(relation)?;

    if !finite_relation_is_from(relation, set, set)? {
        return Ok(false);
    }

    Ok(set
        .elements
        .iter()
        .all(|element| pairs.contains(&(element.clone(), element.clone()))))
}

fn finite_relation_is_symmetric(
    relation: &FiniteSetExpression,
    set: &FiniteSetExpression,
    _: &FiniteSetExpression,
) -> Result<bool, DiagnosticDto> {
    let pairs = finite_relation_pair_set(relation)?;

    if !finite_relation_is_from(relation, set, set)? {
        return Ok(false);
    }

    Ok(pairs
        .iter()
        .all(|(left, right)| pairs.contains(&(right.clone(), left.clone()))))
}

fn finite_relation_is_antisymmetric(
    relation: &FiniteSetExpression,
    set: &FiniteSetExpression,
    _: &FiniteSetExpression,
) -> Result<bool, DiagnosticDto> {
    let pairs = finite_relation_pair_set(relation)?;

    if !finite_relation_is_from(relation, set, set)? {
        return Ok(false);
    }

    Ok(pairs
        .iter()
        .all(|(left, right)| left == right || !pairs.contains(&(right.clone(), left.clone()))))
}

fn finite_relation_is_transitive(
    relation: &FiniteSetExpression,
    set: &FiniteSetExpression,
    _: &FiniteSetExpression,
) -> Result<bool, DiagnosticDto> {
    let pairs = finite_relation_pair_set(relation)?;

    if !finite_relation_is_from(relation, set, set)? {
        return Ok(false);
    }

    Ok(pairs.iter().all(|(left, middle)| {
        pairs.iter().all(|(candidate_middle, right)| {
            middle != candidate_middle || pairs.contains(&(left.clone(), right.clone()))
        })
    }))
}

fn finite_relation_pair_set(
    relation: &FiniteSetExpression,
) -> Result<BTreeSet<(FiniteSetElement, FiniteSetElement)>, DiagnosticDto> {
    Ok(finite_relation_pairs(relation)?.into_iter().collect())
}

fn finite_relation_pairs(
    relation: &FiniteSetExpression,
) -> Result<Vec<(FiniteSetElement, FiniteSetElement)>, DiagnosticDto> {
    relation
        .elements
        .iter()
        .map(|element| match element {
            FiniteSetElement::OrderedPair(left, right) => Ok(((**left).clone(), (**right).clone())),
            _ => Err(DiagnosticDto {
                code: "Relation.ExpectedOrderedPairs".to_owned(),
                message: "a finite relation must be a set of ordered pairs".to_owned(),
            }),
        })
        .collect()
}

fn evaluate_finite_relation_set_operation(
    relation_source: &str,
    input_format: &str,
    operation: fn(&FiniteSetExpression) -> Result<FiniteSetExpression, DiagnosticDto>,
) -> NormalizeSetExpressionResponseDto {
    let relation = match normalize_finite_set_expression(relation_source, input_format) {
        Ok(value) => value,
        Err(diagnostic) => {
            return NormalizeSetExpressionResponseDto {
                outcome: MathematicalOutcomeKindDto::Unknown,
                normalized: None,
                diagnostics: vec![diagnostic],
            };
        }
    };
    let result = match operation(&relation) {
        Ok(value) => value,
        Err(diagnostic) => {
            return NormalizeSetExpressionResponseDto {
                outcome: MathematicalOutcomeKindDto::Unknown,
                normalized: None,
                diagnostics: vec![diagnostic],
            };
        }
    };

    NormalizeSetExpressionResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        normalized: Some(SetExpressionDto {
            latex: result.to_latex(),
        }),
        diagnostics: Vec::new(),
    }
}

fn finite_relation_domain(
    relation: &FiniteSetExpression,
) -> Result<FiniteSetExpression, DiagnosticDto> {
    Ok(FiniteSetExpression {
        elements: finite_relation_pairs(relation)?
            .into_iter()
            .map(|(left, _)| left)
            .collect(),
    })
}

fn finite_relation_range(
    relation: &FiniteSetExpression,
) -> Result<FiniteSetExpression, DiagnosticDto> {
    Ok(FiniteSetExpression {
        elements: finite_relation_pairs(relation)?
            .into_iter()
            .map(|(_, right)| right)
            .collect(),
    })
}

fn finite_relation_inverse(
    relation: &FiniteSetExpression,
) -> Result<FiniteSetExpression, DiagnosticDto> {
    Ok(FiniteSetExpression {
        elements: finite_relation_pairs(relation)?
            .into_iter()
            .map(|(left, right)| FiniteSetElement::OrderedPair(Box::new(right), Box::new(left)))
            .collect(),
    })
}

fn unknown_finite_relation_predicate(
    relation_name: &str,
    diagnostic: DiagnosticDto,
) -> EvaluateFiniteRelationPredicateResponseDto {
    EvaluateFiniteRelationPredicateResponseDto {
        outcome: MathematicalOutcomeKindDto::Unknown,
        relation: relation_name.to_owned(),
        truth: None,
        normalized_relation: None,
        normalized_domain: None,
        normalized_codomain: None,
        diagnostics: vec![diagnostic],
    }
}

fn source_after_whitespace(source: &str) -> &str {
    source.trim_start()
}

fn evaluate_finite_set_statement(
    source: &str,
    input_format: &str,
) -> Result<FiniteSetStatementEvaluation, DiagnosticDto> {
    if !input_format.eq_ignore_ascii_case("latex") {
        return Err(DiagnosticDto {
            code: "Set.UnsupportedInputFormat".to_owned(),
            message: "set statements currently require LaTeX input".to_owned(),
        });
    }

    let (left_source, operator, right_source) = split_finite_set_statement(source)?;

    match operator {
        FiniteSetStatementOperator::In | FiniteSetStatementOperator::NotIn => {
            let left = normalize_finite_set_element_expression(left_source)?;
            let right = normalize_finite_set_expression(right_source, input_format)?;
            let contains = right.elements.contains(&left);
            let truth = match operator {
                FiniteSetStatementOperator::In => contains,
                FiniteSetStatementOperator::NotIn => !contains,
                FiniteSetStatementOperator::SubsetEq | FiniteSetStatementOperator::NotSubsetEq => {
                    unreachable!("membership branch only handles membership operators")
                }
            };

            Ok(FiniteSetStatementEvaluation {
                truth,
                normalized_latex: format!(
                    "{} {} {}",
                    left.to_latex(),
                    operator.to_latex(),
                    right.to_latex()
                ),
            })
        }
        FiniteSetStatementOperator::SubsetEq | FiniteSetStatementOperator::NotSubsetEq => {
            let left = normalize_finite_set_expression(left_source, input_format)?;
            let right = normalize_finite_set_expression(right_source, input_format)?;
            let is_subset = left.elements.is_subset(&right.elements);
            let truth = match operator {
                FiniteSetStatementOperator::SubsetEq => is_subset,
                FiniteSetStatementOperator::NotSubsetEq => !is_subset,
                FiniteSetStatementOperator::In | FiniteSetStatementOperator::NotIn => {
                    unreachable!("subset branch only handles subset operators")
                }
            };

            Ok(FiniteSetStatementEvaluation {
                truth,
                normalized_latex: format!(
                    "{} {} {}",
                    left.to_latex(),
                    operator.to_latex(),
                    right.to_latex()
                ),
            })
        }
    }
}

fn normalize_finite_set_element_expression(
    source: &str,
) -> Result<FiniteSetElement, DiagnosticDto> {
    let (element, rest) = parse_finite_set_element(source)?;

    if !rest.trim().is_empty() {
        return Err(DiagnosticDto {
            code: "Set.TrailingInput".to_owned(),
            message: "unexpected input after finite set element".to_owned(),
        });
    }

    Ok(element)
}

fn parse_finite_set<'a>(
    source: &'a str,
    context: Option<&FiniteSetContext>,
) -> Result<(FiniteSetExpression, &'a str), DiagnosticDto> {
    let source = source.trim_start();

    if let Some(rest) = source.strip_prefix("\\varnothing") {
        return Ok((FiniteSetExpression::empty(), rest));
    }

    let Some(after_open) = strip_set_open(source) else {
        return Err(DiagnosticDto {
            code: "Set.ExpectedRoster".to_owned(),
            message: "expected a finite roster set such as \\{1,2,3\\}".to_owned(),
        });
    };

    let mut elements = BTreeSet::new();
    let mut rest = after_open.trim_start();

    if let Some(after_close) = strip_set_close(rest) {
        return Ok((FiniteSetExpression { elements }, after_close));
    }

    if let Some(builder_result) = try_parse_bounded_set_builder(after_open, context)? {
        return Ok(builder_result);
    }

    loop {
        let (element, after_element) = parse_finite_set_element(rest)?;
        elements.insert(element);
        rest = after_element.trim_start();

        if let Some(after_comma) = rest.strip_prefix(',') {
            rest = after_comma.trim_start();
            continue;
        }

        let Some(after_close) = strip_set_close(rest) else {
            return Err(DiagnosticDto {
                code: "Set.ExpectedSeparatorOrClose".to_owned(),
                message: "expected ',' or the closing brace of the finite set".to_owned(),
            });
        };

        return Ok((FiniteSetExpression { elements }, after_close));
    }
}

fn try_parse_bounded_set_builder<'a>(
    source: &'a str,
    context: Option<&FiniteSetContext>,
) -> Result<Option<(FiniteSetExpression, &'a str)>, DiagnosticDto> {
    let source = source.trim_start();
    let Some((variable, after_variable)) = parse_set_builder_variable(source) else {
        return Ok(None);
    };
    let Some(after_membership) = after_variable.trim_start().strip_prefix("\\in") else {
        return Ok(None);
    };

    let (domain, after_domain) = parse_finite_set_union_expression(after_membership, context)?;
    let after_domain = after_domain.trim_start();
    let Some(after_separator) = after_domain
        .strip_prefix("\\mid")
        .or_else(|| after_domain.strip_prefix('|'))
        .or_else(|| after_domain.strip_prefix(':'))
    else {
        return Ok(None);
    };

    let (predicate_source, rest) = extract_until_set_close_at_top_level(after_separator)?;
    let mut elements = BTreeSet::new();

    for element in &domain.elements {
        if evaluate_finite_set_builder_predicate(variable, element, predicate_source)? {
            elements.insert(element.clone());
        }
    }

    Ok(Some((FiniteSetExpression { elements }, rest)))
}

fn parse_set_builder_variable(source: &str) -> Option<(&str, &str)> {
    let mut end = 0;

    for (index, character) in source.char_indices() {
        if character.is_ascii_alphanumeric() || character == '_' {
            end = index + character.len_utf8();
            continue;
        }

        break;
    }

    (end > 0).then_some((&source[..end], &source[end..]))
}

fn parse_set_expression_symbol(source: &str) -> Option<(&str, &str)> {
    let source = source.trim_start();
    let mut chars = source.char_indices();
    let (_, first) = chars.next()?;

    if !first.is_ascii_alphabetic() {
        return None;
    }

    let mut end = first.len_utf8();

    for (index, character) in chars {
        if character.is_ascii_alphanumeric() || character == '_' {
            end = index + character.len_utf8();
            continue;
        }

        break;
    }

    Some((&source[..end], &source[end..]))
}

fn extract_until_set_close_at_top_level(source: &str) -> Result<(&str, &str), DiagnosticDto> {
    let mut index = 0;
    let mut set_depth = 0_i32;

    while index < source.len() {
        let rest = &source[index..];

        if set_depth == 0
            && let Some(after_close) = strip_set_close(rest)
        {
            return Ok((source[..index].trim(), after_close));
        }

        if rest.starts_with("\\{") {
            set_depth += 1;
            index += "\\{".len();
            continue;
        }

        if rest.starts_with("\\}") {
            set_depth -= 1;
            index += "\\}".len();
            continue;
        }

        let Some(character) = rest.chars().next() else {
            break;
        };

        match character {
            '{' => set_depth += 1,
            '}' => set_depth -= 1,
            _ => {}
        }
        index += character.len_utf8();
    }

    Err(DiagnosticDto {
        code: "Set.ExpectedBuilderClose".to_owned(),
        message: "expected the closing brace of the bounded set-builder expression".to_owned(),
    })
}

fn evaluate_finite_set_builder_predicate(
    variable: &str,
    element: &FiniteSetElement,
    predicate_source: &str,
) -> Result<bool, DiagnosticDto> {
    let predicate = normalize_set_builder_predicate_source(predicate_source);

    if let Some(result) = evaluate_even_odd_predicate(variable, element, &predicate)? {
        return Ok(result);
    }

    if let Some(result) = evaluate_divisibility_predicate(variable, element, &predicate)? {
        return Ok(result);
    }

    if let Some(result) = evaluate_atom_comparison_predicate(variable, element, &predicate)? {
        return Ok(result);
    }

    if let Some(result) = evaluate_builder_membership_predicate(variable, element, &predicate)? {
        return Ok(result);
    }

    Err(DiagnosticDto {
        code: "Set.UnsupportedBuilderPredicate".to_owned(),
        message: "bounded set-builder predicates currently support equality, inequality, numeric comparisons, membership, even/odd, and divisibility".to_owned(),
    })
}

fn normalize_set_builder_predicate_source(source: &str) -> String {
    source
        .trim()
        .replace("\\text{ is even}", " is even")
        .replace("\\text{is even}", " is even")
        .replace("\\mathrm{even}", "even")
        .replace("\\text{ is odd}", " is odd")
        .replace("\\text{is odd}", " is odd")
        .replace("\\mathrm{odd}", "odd")
        .replace("\\,", "")
        .replace(' ', "")
}

fn evaluate_even_odd_predicate(
    variable: &str,
    element: &FiniteSetElement,
    predicate: &str,
) -> Result<Option<bool>, DiagnosticDto> {
    let even_forms = [
        format!("{variable}iseven"),
        format!("even({variable})"),
        format!("{variable}\\equiv0\\pmod{{2}}"),
    ];
    let odd_forms = [
        format!("{variable}isodd"),
        format!("odd({variable})"),
        format!("{variable}\\equiv1\\pmod{{2}}"),
    ];

    if even_forms.iter().any(|form| form == predicate) {
        return Ok(Some(integer_atom(element)? % 2 == 0));
    }

    if odd_forms.iter().any(|form| form == predicate) {
        return Ok(Some(integer_atom(element)? % 2 != 0));
    }

    Ok(None)
}

fn evaluate_divisibility_predicate(
    variable: &str,
    element: &FiniteSetElement,
    predicate: &str,
) -> Result<Option<bool>, DiagnosticDto> {
    let Some((left, right)) = predicate.split_once("\\mid") else {
        return Ok(None);
    };

    if right != variable {
        return Ok(None);
    }

    let divisor = left.parse::<i64>().map_err(|_| DiagnosticDto {
        code: "Set.UnsupportedBuilderPredicate".to_owned(),
        message: "divisibility predicates must use an integer divisor".to_owned(),
    })?;

    if divisor == 0 {
        return Err(DiagnosticDto {
            code: "Set.DivisionByZeroPredicate".to_owned(),
            message: "divisibility by zero is undefined".to_owned(),
        });
    }

    Ok(Some(integer_atom(element)? % divisor == 0))
}

fn evaluate_atom_comparison_predicate(
    variable: &str,
    element: &FiniteSetElement,
    predicate: &str,
) -> Result<Option<bool>, DiagnosticDto> {
    let operators = [
        ("\\neq", FiniteSetAtomComparisonOperator::NotEqual),
        ("!=", FiniteSetAtomComparisonOperator::NotEqual),
        ("\\ne", FiniteSetAtomComparisonOperator::NotEqual),
        ("\\leq", FiniteSetAtomComparisonOperator::LessThanOrEqual),
        ("<=", FiniteSetAtomComparisonOperator::LessThanOrEqual),
        ("\\geq", FiniteSetAtomComparisonOperator::GreaterThanOrEqual),
        (">=", FiniteSetAtomComparisonOperator::GreaterThanOrEqual),
        ("=", FiniteSetAtomComparisonOperator::Equal),
        ("<", FiniteSetAtomComparisonOperator::LessThan),
        (">", FiniteSetAtomComparisonOperator::GreaterThan),
    ];

    for (operator_source, operator) in operators {
        let Some((left, right)) = predicate.split_once(operator_source) else {
            continue;
        };

        if left == variable {
            return Ok(Some(compare_set_builder_atoms(element, right, operator)?));
        }

        if right == variable {
            return Ok(Some(compare_set_builder_atoms(
                element,
                left,
                operator.reversed(),
            )?));
        }
    }

    Ok(None)
}

fn compare_set_builder_atoms(
    element: &FiniteSetElement,
    literal: &str,
    operator: FiniteSetAtomComparisonOperator,
) -> Result<bool, DiagnosticDto> {
    match operator {
        FiniteSetAtomComparisonOperator::Equal => {
            Ok(element == &FiniteSetElement::Atom(literal.to_owned()))
        }
        FiniteSetAtomComparisonOperator::NotEqual => {
            Ok(element != &FiniteSetElement::Atom(literal.to_owned()))
        }
        FiniteSetAtomComparisonOperator::LessThan
        | FiniteSetAtomComparisonOperator::LessThanOrEqual
        | FiniteSetAtomComparisonOperator::GreaterThan
        | FiniteSetAtomComparisonOperator::GreaterThanOrEqual => {
            let left = integer_atom(element)?;
            let right = literal.parse::<i64>().map_err(|_| DiagnosticDto {
                code: "Set.UnsupportedBuilderPredicate".to_owned(),
                message: "numeric set-builder comparisons require integer literals".to_owned(),
            })?;

            Ok(match operator {
                FiniteSetAtomComparisonOperator::LessThan => left < right,
                FiniteSetAtomComparisonOperator::LessThanOrEqual => left <= right,
                FiniteSetAtomComparisonOperator::GreaterThan => left > right,
                FiniteSetAtomComparisonOperator::GreaterThanOrEqual => left >= right,
                FiniteSetAtomComparisonOperator::Equal
                | FiniteSetAtomComparisonOperator::NotEqual => {
                    unreachable!("equality operators are handled before numeric comparison")
                }
            })
        }
    }
}

fn evaluate_builder_membership_predicate(
    variable: &str,
    element: &FiniteSetElement,
    predicate: &str,
) -> Result<Option<bool>, DiagnosticDto> {
    let Some(after_variable) = predicate.strip_prefix(variable) else {
        return Ok(None);
    };
    let after_operator = if let Some(rest) = after_variable.strip_prefix("\\notin") {
        rest
    } else if let Some(rest) = after_variable.strip_prefix("\\in") {
        rest
    } else {
        return Ok(None);
    };
    let (set, rest) = parse_finite_set_union_expression(after_operator, None)?;

    if !rest.trim().is_empty() {
        return Err(DiagnosticDto {
            code: "Set.TrailingInput".to_owned(),
            message: "unexpected input after set-builder membership predicate".to_owned(),
        });
    }

    let contains = set.elements.contains(element);
    Ok(Some(if after_variable.starts_with("\\notin") {
        !contains
    } else {
        contains
    }))
}

fn integer_atom(element: &FiniteSetElement) -> Result<i64, DiagnosticDto> {
    let FiniteSetElement::Atom(value) = element else {
        return Err(DiagnosticDto {
            code: "Set.ExpectedIntegerAtom".to_owned(),
            message: "this bounded set-builder predicate requires integer atom elements".to_owned(),
        });
    };

    value.parse::<i64>().map_err(|_| DiagnosticDto {
        code: "Set.ExpectedIntegerAtom".to_owned(),
        message: "this bounded set-builder predicate requires integer atom elements".to_owned(),
    })
}

impl FiniteSetAtomComparisonOperator {
    fn reversed(self) -> Self {
        match self {
            Self::Equal => Self::Equal,
            Self::NotEqual => Self::NotEqual,
            Self::LessThan => Self::GreaterThan,
            Self::LessThanOrEqual => Self::GreaterThanOrEqual,
            Self::GreaterThan => Self::LessThan,
            Self::GreaterThanOrEqual => Self::LessThanOrEqual,
        }
    }
}

fn parse_finite_set_element(source: &str) -> Result<(FiniteSetElement, &str), DiagnosticDto> {
    let source = source.trim_start();

    if source.starts_with("\\varnothing") || strip_set_open(source).is_some() {
        let (set, rest) = parse_finite_set(source, None)?;
        return Ok((FiniteSetElement::Set(set), rest));
    }

    if source.starts_with('(') {
        return parse_ordered_pair(source);
    }

    let mut end = 0;
    for (index, character) in source.char_indices() {
        if character == ',' || character == ')' || starts_set_close(&source[index..]) {
            break;
        }
        end = index + character.len_utf8();
    }

    let atom = source[..end].trim().replace(char::is_whitespace, "");
    if atom.is_empty() {
        return Err(DiagnosticDto {
            code: "Set.ExpectedElement".to_owned(),
            message: "expected an element in the finite set".to_owned(),
        });
    }

    Ok((FiniteSetElement::Atom(atom), &source[end..]))
}

fn parse_ordered_pair(source: &str) -> Result<(FiniteSetElement, &str), DiagnosticDto> {
    let Some(after_open) = source.strip_prefix('(') else {
        return Err(DiagnosticDto {
            code: "Set.ExpectedOrderedPair".to_owned(),
            message: "expected an ordered pair".to_owned(),
        });
    };

    let (left, after_left) = parse_finite_set_element(after_open)?;
    let after_left = after_left.trim_start();
    let Some(after_comma) = after_left.strip_prefix(',') else {
        return Err(DiagnosticDto {
            code: "Set.ExpectedOrderedPairComma".to_owned(),
            message: "expected ',' between ordered pair coordinates".to_owned(),
        });
    };

    let (right, after_right) = parse_finite_set_element(after_comma)?;
    let after_right = after_right.trim_start();
    let Some(rest) = after_right.strip_prefix(')') else {
        return Err(DiagnosticDto {
            code: "Set.ExpectedOrderedPairClose".to_owned(),
            message: "expected ')' to close the ordered pair".to_owned(),
        });
    };

    Ok((
        FiniteSetElement::OrderedPair(Box::new(left), Box::new(right)),
        rest,
    ))
}

fn strip_set_open(source: &str) -> Option<&str> {
    source
        .strip_prefix("\\{")
        .or_else(|| source.strip_prefix('{'))
}

fn strip_set_close(source: &str) -> Option<&str> {
    source
        .strip_prefix("\\}")
        .or_else(|| source.strip_prefix('}'))
}

fn starts_set_close(source: &str) -> bool {
    source.starts_with("\\}") || source.starts_with('}')
}

fn split_finite_set_statement(
    source: &str,
) -> Result<(&str, FiniteSetStatementOperator, &str), DiagnosticDto> {
    let operators = [
        ("\\not\\subseteq", FiniteSetStatementOperator::NotSubsetEq),
        ("\\nsubseteq", FiniteSetStatementOperator::NotSubsetEq),
        ("\\notin", FiniteSetStatementOperator::NotIn),
        ("\\subseteq", FiniteSetStatementOperator::SubsetEq),
        ("\\subset", FiniteSetStatementOperator::SubsetEq),
        ("\\in", FiniteSetStatementOperator::In),
    ];
    let mut index = 0;
    let mut set_depth = 0_i32;

    while index < source.len() {
        let rest = &source[index..];

        if set_depth == 0 {
            for (operator_source, operator) in operators {
                if rest.starts_with(operator_source) {
                    let left = source[..index].trim();
                    let right = source[index + operator_source.len()..].trim();

                    if left.is_empty() || right.is_empty() {
                        return Err(DiagnosticDto {
                            code: "Set.ExpectedStatementOperand".to_owned(),
                            message: "expected operands on both sides of the set statement"
                                .to_owned(),
                        });
                    }

                    return Ok((left, operator, right));
                }
            }
        }

        if rest.starts_with("\\{") {
            set_depth += 1;
            index += "\\{".len();
            continue;
        }

        if rest.starts_with("\\}") {
            set_depth -= 1;
            index += "\\}".len();
            continue;
        }

        let Some(character) = rest.chars().next() else {
            break;
        };

        match character {
            '{' => set_depth += 1,
            '}' => set_depth -= 1,
            _ => {}
        }
        index += character.len_utf8();
    }

    Err(DiagnosticDto {
        code: "Set.ExpectedStatementOperator".to_owned(),
        message: "expected a supported set statement operator such as \\in or \\subseteq"
            .to_owned(),
    })
}

impl FiniteSetStatementOperator {
    fn to_latex(self) -> &'static str {
        match self {
            Self::In => "\\in",
            Self::NotIn => "\\notin",
            Self::SubsetEq => "\\subseteq",
            Self::NotSubsetEq => "\\nsubseteq",
        }
    }
}

fn unknown_set_comparison(diagnostics: Vec<DiagnosticDto>) -> CompareSetExpressionsResponseDto {
    CompareSetExpressionsResponseDto {
        outcome: MathematicalOutcomeKindDto::Unknown,
        relation: "set.extensional_equal".to_owned(),
        equal: None,
        left_normalized: None,
        right_normalized: None,
        diagnostics,
    }
}

fn unknown_contextual_set_comparison(
    diagnostics: Vec<DiagnosticDto>,
) -> CompareSetExpressionsResponseDto {
    CompareSetExpressionsResponseDto {
        outcome: MathematicalOutcomeKindDto::Unknown,
        relation: "set.extensional_equal.in_context".to_owned(),
        equal: None,
        left_normalized: None,
        right_normalized: None,
        diagnostics,
    }
}

fn derivative_steps(input: &PolynomialExpression) -> Vec<MathDerivationStepDto> {
    let mut steps = Vec::new();

    if input.coefficients.len() > 1 {
        steps.push(MathDerivationStepDto {
            rule: "calculus.derivative.sum-rule".to_owned(),
            reason: "Differentiate a sum term-by-term.".to_owned(),
            target: Some(RuleTargetDto::Whole),
            input_latex: Some(derivative_goal(
                &input.variable,
                &LatexRenderer::polynomial_expression(input),
            )),
            output_latex: Some(split_derivative_goals(input)),
        });
    }

    for (degree, coefficient) in input.coefficients.iter().rev() {
        let input_term = monomial(input.variable.clone(), *degree, coefficient.clone());
        let output_term = input_term.derivative();

        if *degree > 0 && !exact_rational_is_one(coefficient) {
            let base_term = monomial(
                input.variable.clone(),
                *degree,
                socrates_math_core::ExactRational::integer(1),
            );
            let coefficient_latex = coefficient_latex(&input.variable, coefficient);
            let base_latex = LatexRenderer::polynomial_expression(&base_term);

            steps.push(MathDerivationStepDto {
                rule: "calculus.derivative.constant-multiple-rule".to_owned(),
                reason: "Pull the constant coefficient outside the derivative.".to_owned(),
                target: Some(RuleTargetDto::PolynomialTerm { degree: *degree }),
                input_latex: Some(derivative_goal(
                    &input.variable,
                    &LatexRenderer::polynomial_expression(&input_term),
                )),
                output_latex: Some(format!(
                    "{coefficient_latex}{}",
                    derivative_goal(&input.variable, &base_latex)
                )),
            });
        }

        steps.push(MathDerivationStepDto {
            rule: if *degree == 0 {
                "calculus.polynomial.derivative.constant-rule".to_owned()
            } else {
                "calculus.polynomial.derivative.power-rule".to_owned()
            },
            reason: if *degree == 0 {
                "Differentiate a constant term to 0.".to_owned()
            } else {
                "Apply the derivative power rule to one term: d/dx a*x^n = a*n*x^(n-1).".to_owned()
            },
            target: Some(RuleTargetDto::PolynomialTerm { degree: *degree }),
            input_latex: Some(LatexRenderer::polynomial_expression(&input_term)),
            output_latex: Some(LatexRenderer::polynomial_expression(&output_term)),
        });
    }

    if steps.is_empty() {
        steps.push(MathDerivationStepDto {
            rule: "calculus.polynomial.derivative.constant-rule".to_owned(),
            reason: "Differentiate zero to 0.".to_owned(),
            target: Some(RuleTargetDto::Whole),
            input_latex: Some("0".to_owned()),
            output_latex: Some("0".to_owned()),
        });
    }

    steps
}

fn antiderivative_steps(input: &PolynomialExpression) -> Vec<MathDerivationStepDto> {
    let mut steps = Vec::new();

    if input.coefficients.len() > 1 {
        steps.push(MathDerivationStepDto {
            rule: "calculus.integral.sum-rule".to_owned(),
            reason: "Integrate a sum term-by-term.".to_owned(),
            target: Some(RuleTargetDto::Whole),
            input_latex: Some(antiderivative_goal(
                &input.variable,
                &LatexRenderer::polynomial_expression(input),
            )),
            output_latex: Some(split_antiderivative_goals(input)),
        });
    }

    for (degree, coefficient) in input.coefficients.iter().rev() {
        let input_term = monomial(input.variable.clone(), *degree, coefficient.clone());
        let output_term = input_term.antiderivative();

        if !exact_rational_is_one(coefficient) {
            let base_term = monomial(
                input.variable.clone(),
                *degree,
                socrates_math_core::ExactRational::integer(1),
            );
            let coefficient_latex = coefficient_latex(&input.variable, coefficient);
            let base_latex = LatexRenderer::polynomial_expression(&base_term);

            steps.push(MathDerivationStepDto {
                rule: "calculus.integral.constant-multiple-rule".to_owned(),
                reason: "Pull the constant coefficient outside the integral.".to_owned(),
                target: Some(RuleTargetDto::PolynomialTerm { degree: *degree }),
                input_latex: Some(antiderivative_goal(
                    &input.variable,
                    &LatexRenderer::polynomial_expression(&input_term),
                )),
                output_latex: Some(format!(
                    "{coefficient_latex}{}",
                    antiderivative_goal(&input.variable, &base_latex)
                )),
            });
        }

        steps.push(MathDerivationStepDto {
            rule: "calculus.polynomial.integral.power-rule".to_owned(),
            reason: "Apply the antiderivative power rule to one term: integral a*x^n dx = a/(n+1)*x^(n+1).".to_owned(),
            target: Some(RuleTargetDto::PolynomialTerm { degree: *degree }),
            input_latex: Some(LatexRenderer::polynomial_expression(&input_term)),
            output_latex: Some(LatexRenderer::polynomial_expression(&output_term)),
        });
    }

    if steps.is_empty() {
        steps.push(MathDerivationStepDto {
            rule: "calculus.polynomial.integral.zero-rule".to_owned(),
            reason: "Integrate zero to 0 for the chosen representative antiderivative.".to_owned(),
            target: Some(RuleTargetDto::Whole),
            input_latex: Some("0".to_owned()),
            output_latex: Some("0".to_owned()),
        });
    }

    steps
}

fn derivative_goal(variable: &str, operand_latex: &str) -> String {
    format!("\\frac{{d}}{{d{variable}}}\\left({operand_latex}\\right)")
}

fn antiderivative_goal(variable: &str, operand_latex: &str) -> String {
    format!("\\int {operand_latex}\\,d{variable}")
}

fn split_derivative_goals(input: &PolynomialExpression) -> String {
    input
        .coefficients
        .iter()
        .rev()
        .map(|(degree, coefficient)| {
            let term = monomial(input.variable.clone(), *degree, coefficient.clone());
            derivative_goal(
                &input.variable,
                &LatexRenderer::polynomial_expression(&term),
            )
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

fn split_antiderivative_goals(input: &PolynomialExpression) -> String {
    input
        .coefficients
        .iter()
        .rev()
        .map(|(degree, coefficient)| {
            let term = monomial(input.variable.clone(), *degree, coefficient.clone());
            antiderivative_goal(
                &input.variable,
                &LatexRenderer::polynomial_expression(&term),
            )
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

fn coefficient_latex(variable: &str, coefficient: &socrates_math_core::ExactRational) -> String {
    LatexRenderer::polynomial_expression(&PolynomialExpression::constant(
        variable.to_owned(),
        coefficient.clone(),
    ))
}

fn exact_rational_is_one(value: &socrates_math_core::ExactRational) -> bool {
    value == &socrates_math_core::ExactRational::integer(1)
}

fn rules_for_polynomial_term(input: &PolynomialExpression, degree: u32) -> Vec<ApplicableRuleDto> {
    let mut rules = Vec::new();

    if let Some(coefficient) = input.coefficients.get(&degree) {
        if degree > 0 && !exact_rational_is_one(coefficient) {
            rules.push(derivative_constant_multiple_rule_for_term(input, degree));
        }

        if !exact_rational_is_one(coefficient) {
            rules.push(antiderivative_constant_multiple_rule_for_term(
                input, degree,
            ));
        }
    }

    rules.push(derivative_rule_for_term(input, degree));
    rules.push(antiderivative_rule_for_term(input, degree));
    rules
}

fn whole_polynomial_rules(input: &PolynomialExpression) -> Vec<ApplicableRuleDto> {
    vec![
        ApplicableRuleDto {
            rule: "calculus.derivative.sum-rule".to_owned(),
            status: RuleApplicabilityStatusDto::Applicable,
            relation: "calculus.derivative".to_owned(),
            target: Some(RuleTargetDto::Whole),
            reason: "A polynomial sum can be differentiated term-by-term.".to_owned(),
            required_conditions: Vec::new(),
            concepts: vec!["derivative.sum-rule".to_owned()],
        },
        ApplicableRuleDto {
            rule: "calculus.integral.sum-rule".to_owned(),
            status: RuleApplicabilityStatusDto::Applicable,
            relation: "calculus.antiderivative".to_owned(),
            target: Some(RuleTargetDto::Whole),
            reason: "A polynomial sum can be integrated term-by-term.".to_owned(),
            required_conditions: Vec::new(),
            concepts: vec!["integral.sum-rule".to_owned()],
        },
    ]
    .into_iter()
    .filter(|_| input.coefficients.len() > 1)
    .collect()
}

fn derivative_constant_multiple_rule_for_term(
    input: &PolynomialExpression,
    degree: u32,
) -> ApplicableRuleDto {
    let has_term = input.coefficients.contains_key(&degree);

    ApplicableRuleDto {
        rule: "calculus.derivative.constant-multiple-rule".to_owned(),
        status: if has_term {
            RuleApplicabilityStatusDto::Applicable
        } else {
            RuleApplicabilityStatusDto::NotApplicable
        },
        relation: "calculus.derivative".to_owned(),
        target: Some(RuleTargetDto::PolynomialTerm { degree }),
        reason: "A constant coefficient can be pulled outside the derivative.".to_owned(),
        required_conditions: Vec::new(),
        concepts: vec!["derivative.constant-multiple-rule".to_owned()],
    }
}

fn antiderivative_constant_multiple_rule_for_term(
    input: &PolynomialExpression,
    degree: u32,
) -> ApplicableRuleDto {
    let has_term = input.coefficients.contains_key(&degree);

    ApplicableRuleDto {
        rule: "calculus.integral.constant-multiple-rule".to_owned(),
        status: if has_term {
            RuleApplicabilityStatusDto::Applicable
        } else {
            RuleApplicabilityStatusDto::NotApplicable
        },
        relation: "calculus.antiderivative".to_owned(),
        target: Some(RuleTargetDto::PolynomialTerm { degree }),
        reason: "A constant coefficient can be pulled outside the integral.".to_owned(),
        required_conditions: Vec::new(),
        concepts: vec!["integral.constant-multiple-rule".to_owned()],
    }
}

fn derivative_rule_for_term(input: &PolynomialExpression, degree: u32) -> ApplicableRuleDto {
    let has_term = input.coefficients.contains_key(&degree);
    let is_constant = degree == 0;

    ApplicableRuleDto {
        rule: if is_constant {
            "calculus.polynomial.derivative.constant-rule".to_owned()
        } else {
            "calculus.polynomial.derivative.power-rule".to_owned()
        },
        status: if has_term {
            RuleApplicabilityStatusDto::Applicable
        } else {
            RuleApplicabilityStatusDto::NotApplicable
        },
        relation: "calculus.derivative".to_owned(),
        target: Some(RuleTargetDto::PolynomialTerm { degree }),
        reason: if has_term && is_constant {
            "A constant term differentiates to 0.".to_owned()
        } else if has_term {
            "A polynomial term can be differentiated with the power rule.".to_owned()
        } else {
            "No polynomial term exists at the selected degree.".to_owned()
        },
        required_conditions: Vec::new(),
        concepts: if is_constant {
            vec!["derivative.constant-rule".to_owned()]
        } else {
            vec!["derivative.power-rule".to_owned()]
        },
    }
}

fn antiderivative_rule_for_term(input: &PolynomialExpression, degree: u32) -> ApplicableRuleDto {
    let has_term = input.coefficients.contains_key(&degree);

    ApplicableRuleDto {
        rule: "calculus.polynomial.integral.power-rule".to_owned(),
        status: if has_term {
            RuleApplicabilityStatusDto::Applicable
        } else {
            RuleApplicabilityStatusDto::NotApplicable
        },
        relation: "calculus.antiderivative".to_owned(),
        target: Some(RuleTargetDto::PolynomialTerm { degree }),
        reason: if has_term {
            "A polynomial term can be integrated with the antiderivative power rule.".to_owned()
        } else {
            "No polynomial term exists at the selected degree.".to_owned()
        },
        required_conditions: Vec::new(),
        concepts: vec!["integral.power-rule".to_owned()],
    }
}

fn resolve_polynomial_rule_target(
    input: &PolynomialExpression,
    target: Option<RuleTargetDto>,
) -> Result<u32, DiagnosticDto> {
    match target {
        Some(RuleTargetDto::PolynomialTerm { degree }) => Ok(degree),
        Some(RuleTargetDto::Whole) | None => {
            unique_polynomial_term_degree(input).ok_or_else(ambiguous_rule_target_diagnostic)
        }
    }
}

fn unique_polynomial_term_degree(input: &PolynomialExpression) -> Option<u32> {
    let mut degrees = input.coefficients.keys();
    let degree = degrees.next().copied()?;

    if degrees.next().is_none() {
        Some(degree)
    } else {
        None
    }
}

fn ambiguous_rule_target_diagnostic() -> DiagnosticDto {
    DiagnosticDto {
        code: "Rule.AmbiguousTarget".to_owned(),
        message: "select a polynomial term occurrence before applying this rule".to_owned(),
    }
}

fn apply_whole_polynomial_rule(
    input: &PolynomialExpression,
    rule: &str,
) -> Option<ApplyRuleResponseDto> {
    let (relation, reason, input_latex, output_latex) = match rule {
        "calculus.derivative.sum-rule" if input.coefficients.len() > 1 => (
            "calculus.derivative",
            "Differentiate a sum term-by-term.",
            derivative_goal(
                &input.variable,
                &LatexRenderer::polynomial_expression(input),
            ),
            split_derivative_goals(input),
        ),
        "calculus.integral.sum-rule" if input.coefficients.len() > 1 => (
            "calculus.antiderivative",
            "Integrate a sum term-by-term.",
            antiderivative_goal(
                &input.variable,
                &LatexRenderer::polynomial_expression(input),
            ),
            split_antiderivative_goals(input),
        ),
        "calculus.derivative.sum-rule" | "calculus.integral.sum-rule" => {
            return Some(rule_not_applicable(
                input,
                "sum rule requires a polynomial expression with more than one term",
            ));
        }
        _ => return None,
    };

    Some(ApplyRuleResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        relation: relation.to_owned(),
        previous: Some(MathExpressionDto {
            latex: input_latex.clone(),
        }),
        result: Some(MathExpressionDto {
            latex: output_latex.clone(),
        }),
        step: Some(MathDerivationStepDto {
            rule: rule.to_owned(),
            reason: reason.to_owned(),
            target: Some(RuleTargetDto::Whole),
            input_latex: Some(input_latex),
            output_latex: Some(output_latex),
        }),
        diagnostics: Vec::new(),
    })
}

fn apply_polynomial_calculus_rule(
    input: &PolynomialExpression,
    rule: &str,
    degree: u32,
) -> ApplyRuleResponseDto {
    if rule == "calculus.derivative.constant-multiple-rule" {
        return apply_constant_multiple_rule(input, degree, CalculusOperation::Derivative);
    }

    if rule == "calculus.integral.constant-multiple-rule" {
        return apply_constant_multiple_rule(input, degree, CalculusOperation::Antiderivative);
    }

    let Some(calculus_rule) = PolynomialCalculusRule::from_rule_id(rule, degree) else {
        return rule_not_applicable(input, "rule is not supported for selected polynomial terms");
    };

    let Some(coefficient) = input.coefficients.get(&degree) else {
        return ApplyRuleResponseDto {
            outcome: MathematicalOutcomeKindDto::Unknown,
            relation: calculus_rule.relation().to_owned(),
            previous: Some(MathExpressionDto {
                latex: LatexRenderer::polynomial_expression(input),
            }),
            result: None,
            step: None,
            diagnostics: vec![DiagnosticDto {
                code: "Rule.TargetNotFound".to_owned(),
                message: "no polynomial term exists at the selected degree".to_owned(),
            }],
        };
    };

    let input_term = monomial(input.variable.clone(), degree, coefficient.clone());
    let output_term = calculus_rule.apply(&input_term);
    let output_latex = LatexRenderer::polynomial_expression(&output_term);

    ApplyRuleResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        relation: calculus_rule.relation().to_owned(),
        previous: Some(MathExpressionDto {
            latex: LatexRenderer::polynomial_expression(&input_term),
        }),
        result: Some(MathExpressionDto {
            latex: output_latex.clone(),
        }),
        step: Some(MathDerivationStepDto {
            rule: calculus_rule.rule_id(degree).to_owned(),
            reason: calculus_rule.reason(degree),
            target: Some(RuleTargetDto::PolynomialTerm { degree }),
            input_latex: Some(LatexRenderer::polynomial_expression(&input_term)),
            output_latex: Some(output_latex),
        }),
        diagnostics: Vec::new(),
    }
}

fn apply_constant_multiple_rule(
    input: &PolynomialExpression,
    degree: u32,
    operation: CalculusOperation,
) -> ApplyRuleResponseDto {
    let Some(coefficient) = input.coefficients.get(&degree) else {
        return ApplyRuleResponseDto {
            outcome: MathematicalOutcomeKindDto::Unknown,
            relation: operation.relation().to_owned(),
            previous: Some(MathExpressionDto {
                latex: LatexRenderer::polynomial_expression(input),
            }),
            result: None,
            step: None,
            diagnostics: vec![DiagnosticDto {
                code: "Rule.TargetNotFound".to_owned(),
                message: "no polynomial term exists at the selected degree".to_owned(),
            }],
        };
    };

    if operation == CalculusOperation::Derivative && degree == 0 {
        return rule_not_applicable(
            input,
            "derivative constant-multiple rule requires a nonconstant term",
        );
    }

    if exact_rational_is_one(coefficient) {
        return rule_not_applicable(
            input,
            "constant-multiple rule requires a coefficient other than 1",
        );
    }

    let input_term = monomial(input.variable.clone(), degree, coefficient.clone());
    let base_term = monomial(
        input.variable.clone(),
        degree,
        socrates_math_core::ExactRational::integer(1),
    );
    let coefficient_latex = coefficient_latex(&input.variable, coefficient);
    let input_term_latex = LatexRenderer::polynomial_expression(&input_term);
    let base_latex = LatexRenderer::polynomial_expression(&base_term);
    let input_latex = operation.goal(&input.variable, &input_term_latex);
    let output_latex = format!(
        "{coefficient_latex}{}",
        operation.goal(&input.variable, &base_latex)
    );

    ApplyRuleResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        relation: operation.relation().to_owned(),
        previous: Some(MathExpressionDto {
            latex: input_latex.clone(),
        }),
        result: Some(MathExpressionDto {
            latex: output_latex.clone(),
        }),
        step: Some(MathDerivationStepDto {
            rule: operation.constant_multiple_rule_id().to_owned(),
            reason: operation.constant_multiple_reason(),
            target: Some(RuleTargetDto::PolynomialTerm { degree }),
            input_latex: Some(input_latex),
            output_latex: Some(output_latex),
        }),
        diagnostics: Vec::new(),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CalculusOperation {
    Derivative,
    Antiderivative,
}

impl CalculusOperation {
    fn relation(self) -> &'static str {
        match self {
            Self::Derivative => "calculus.derivative",
            Self::Antiderivative => "calculus.antiderivative",
        }
    }

    fn goal(self, variable: &str, operand_latex: &str) -> String {
        match self {
            Self::Derivative => derivative_goal(variable, operand_latex),
            Self::Antiderivative => antiderivative_goal(variable, operand_latex),
        }
    }

    fn constant_multiple_rule_id(self) -> &'static str {
        match self {
            Self::Derivative => "calculus.derivative.constant-multiple-rule",
            Self::Antiderivative => "calculus.integral.constant-multiple-rule",
        }
    }

    fn constant_multiple_reason(self) -> String {
        match self {
            Self::Derivative => "Pull the constant coefficient outside the derivative.".to_owned(),
            Self::Antiderivative => {
                "Pull the constant coefficient outside the integral.".to_owned()
            }
        }
    }
}

enum PolynomialCalculusRule {
    Derivative,
    Antiderivative,
}

impl PolynomialCalculusRule {
    fn from_rule_id(rule: &str, degree: u32) -> Option<Self> {
        match rule {
            "calculus.polynomial.derivative.constant-rule" if degree == 0 => Some(Self::Derivative),
            "calculus.polynomial.derivative.power-rule" if degree > 0 => Some(Self::Derivative),
            "calculus.polynomial.integral.power-rule" => Some(Self::Antiderivative),
            _ => None,
        }
    }

    fn relation(&self) -> &'static str {
        match self {
            Self::Derivative => "calculus.derivative",
            Self::Antiderivative => "calculus.antiderivative",
        }
    }

    fn rule_id(&self, degree: u32) -> &'static str {
        match self {
            Self::Derivative if degree == 0 => "calculus.polynomial.derivative.constant-rule",
            Self::Derivative => "calculus.polynomial.derivative.power-rule",
            Self::Antiderivative => "calculus.polynomial.integral.power-rule",
        }
    }

    fn reason(&self, degree: u32) -> String {
        match self {
            Self::Derivative if degree == 0 => "Differentiate a constant term to 0.".to_owned(),
            Self::Derivative => {
                "Apply the derivative power rule to one selected term: d/dx a*x^n = a*n*x^(n-1)."
                    .to_owned()
            }
            Self::Antiderivative => {
                "Apply the antiderivative power rule to one selected term: integral a*x^n dx = a/(n+1)*x^(n+1).".to_owned()
            }
        }
    }

    fn apply(&self, input: &PolynomialExpression) -> PolynomialExpression {
        match self {
            Self::Derivative => input.derivative(),
            Self::Antiderivative => input.antiderivative(),
        }
    }
}

fn rule_not_applicable(input: &PolynomialExpression, message: &str) -> ApplyRuleResponseDto {
    ApplyRuleResponseDto {
        outcome: MathematicalOutcomeKindDto::Unknown,
        relation: "rule.application".to_owned(),
        previous: Some(MathExpressionDto {
            latex: LatexRenderer::polynomial_expression(input),
        }),
        result: None,
        step: None,
        diagnostics: vec![DiagnosticDto {
            code: "Rule.NotApplicable".to_owned(),
            message: message.to_owned(),
        }],
    }
}

fn generic_rule_not_applicable(
    previous_latex: String,
    message: &str,
    relation: &str,
) -> ApplyRuleResponseDto {
    ApplyRuleResponseDto {
        outcome: MathematicalOutcomeKindDto::Unknown,
        relation: relation.to_owned(),
        previous: Some(MathExpressionDto {
            latex: previous_latex,
        }),
        result: None,
        step: None,
        diagnostics: vec![DiagnosticDto {
            code: "Rule.NotApplicable".to_owned(),
            message: message.to_owned(),
        }],
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RationalPowerMonomial {
    variable: String,
    coefficient: ExactRational,
    exponent: ExactRational,
}

fn transform_rational_power_monomial(
    source: &str,
    input_format: &str,
    variable: &str,
    operation: CalculusOperation,
) -> Option<TransformMathExpressionResponseDto> {
    if !input_format.eq_ignore_ascii_case("latex") {
        return None;
    }

    let term = parse_and_elaborate_expression(source, variable).ok()?;
    let monomial = rational_power_monomial(&term, variable)?;
    let (result, rule, reason) = match operation {
        CalculusOperation::Derivative => derivative_rational_power_monomial(&monomial),
        CalculusOperation::Antiderivative => antiderivative_rational_power_monomial(&monomial)?,
    };
    let input_latex = render_rational_power_monomial(&monomial);
    let output_latex = render_rational_power_monomial(&result);

    Some(TransformMathExpressionResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        relation: operation.relation().to_owned(),
        result: Some(MathExpressionDto {
            latex: output_latex.clone(),
        }),
        steps: vec![MathDerivationStepDto {
            rule,
            reason,
            target: Some(RuleTargetDto::Whole),
            input_latex: Some(input_latex),
            output_latex: Some(output_latex),
        }],
        diagnostics: Vec::new(),
    })
}

fn list_rational_power_monomial_rules(
    source: &str,
    input_format: &str,
    variable: &str,
    target: Option<RuleTargetDto>,
) -> Option<ListApplicableRulesResponseDto> {
    if !input_format.eq_ignore_ascii_case("latex")
        || matches!(target, Some(RuleTargetDto::PolynomialTerm { .. }))
    {
        return None;
    }

    let term = parse_and_elaborate_expression(source, variable).ok()?;
    let monomial = rational_power_monomial(&term, variable)?;
    let integral_is_power_rule = monomial.exponent != ExactRational::integer(-1);

    let rules = vec![
        ApplicableRuleDto {
            rule: "calculus.power.derivative.rational-rule".to_owned(),
            status: RuleApplicabilityStatusDto::Applicable,
            relation: "calculus.derivative".to_owned(),
            target: Some(RuleTargetDto::Whole),
            reason: "A rational-power monomial can be differentiated with the power rule."
                .to_owned(),
            required_conditions: Vec::new(),
            concepts: vec!["derivative.power-rule".to_owned()],
        },
        ApplicableRuleDto {
            rule: "calculus.power.integral.rational-rule".to_owned(),
            status: if integral_is_power_rule {
                RuleApplicabilityStatusDto::Applicable
            } else {
                RuleApplicabilityStatusDto::NotApplicable
            },
            relation: "calculus.antiderivative".to_owned(),
            target: Some(RuleTargetDto::Whole),
            reason: if integral_is_power_rule {
                "A rational-power monomial with exponent n != -1 can be integrated with the power rule."
                    .to_owned()
            } else {
                "The antiderivative of x^-1 requires the logarithm rule, not the power rule."
                    .to_owned()
            },
            required_conditions: if integral_is_power_rule {
                Vec::new()
            } else {
                vec!["exponent != -1".to_owned()]
            },
            concepts: vec!["integral.power-rule".to_owned()],
        },
    ];

    Some(ListApplicableRulesResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        rules,
        diagnostics: Vec::new(),
    })
}

fn apply_rational_power_monomial_rule(
    source: &str,
    input_format: &str,
    variable: &str,
    rule: &str,
    target: Option<RuleTargetDto>,
) -> Option<ApplyRuleResponseDto> {
    if !input_format.eq_ignore_ascii_case("latex")
        || matches!(target, Some(RuleTargetDto::PolynomialTerm { .. }))
    {
        return None;
    }

    let term = parse_and_elaborate_expression(source, variable).ok()?;
    let monomial = rational_power_monomial(&term, variable)?;
    let input_latex = render_rational_power_monomial(&monomial);

    let (operation, result, rule, reason) = match rule {
        "calculus.power.derivative.rational-rule" => {
            let (result, rule, reason) = derivative_rational_power_monomial(&monomial);
            (CalculusOperation::Derivative, result, rule, reason)
        }
        "calculus.power.integral.rational-rule" => {
            let Some((result, rule, reason)) = antiderivative_rational_power_monomial(&monomial)
            else {
                return Some(generic_rule_not_applicable(
                    input_latex,
                    "the rational-power antiderivative rule requires exponent n != -1",
                    "calculus.antiderivative",
                ));
            };
            (CalculusOperation::Antiderivative, result, rule, reason)
        }
        _ => return None,
    };

    let output_latex = render_rational_power_monomial(&result);

    Some(ApplyRuleResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        relation: operation.relation().to_owned(),
        previous: Some(MathExpressionDto {
            latex: input_latex.clone(),
        }),
        result: Some(MathExpressionDto {
            latex: output_latex.clone(),
        }),
        step: Some(MathDerivationStepDto {
            rule,
            reason,
            target: Some(RuleTargetDto::Whole),
            input_latex: Some(input_latex),
            output_latex: Some(output_latex),
        }),
        diagnostics: Vec::new(),
    })
}

fn derivative_rational_power_monomial(
    monomial: &RationalPowerMonomial,
) -> (RationalPowerMonomial, String, String) {
    (
        RationalPowerMonomial {
            variable: monomial.variable.clone(),
            coefficient: monomial.coefficient.mul(&monomial.exponent),
            exponent: monomial.exponent.sub(&ExactRational::integer(1)),
        },
        "calculus.power.derivative.rational-rule".to_owned(),
        "Apply the derivative power rule for a rational exponent: d/dx a*x^n = a*n*x^(n-1)."
            .to_owned(),
    )
}

fn antiderivative_rational_power_monomial(
    monomial: &RationalPowerMonomial,
) -> Option<(RationalPowerMonomial, String, String)> {
    if monomial.exponent == ExactRational::integer(-1) {
        return None;
    }

    let antiderivative_exponent = monomial.exponent.add(&ExactRational::integer(1));
    let coefficient = monomial.coefficient.div(&antiderivative_exponent).ok()?;

    Some((
        RationalPowerMonomial {
            variable: monomial.variable.clone(),
            coefficient,
            exponent: antiderivative_exponent,
        },
        "calculus.power.integral.rational-rule".to_owned(),
        "Apply the antiderivative power rule for a rational exponent: integral a*x^n dx = a/(n+1)*x^(n+1), n != -1."
            .to_owned(),
    ))
}

fn rational_power_monomial(term: &SemanticTerm, variable: &str) -> Option<RationalPowerMonomial> {
    match term {
        SemanticTerm::RationalLiteral(value) => Some(RationalPowerMonomial {
            variable: variable.to_owned(),
            coefficient: value.clone(),
            exponent: ExactRational::integer(0),
        }),
        SemanticTerm::LocalVariable { name, .. } if name == variable => {
            Some(RationalPowerMonomial {
                variable: variable.to_owned(),
                coefficient: ExactRational::integer(1),
                exponent: ExactRational::integer(1),
            })
        }
        SemanticTerm::Apply { symbol, args, .. } if symbol.as_str() == "core.rational.mul" => {
            let [left, right] = args.as_slice() else {
                return None;
            };
            let left = rational_power_monomial(left, variable)?;
            let right = rational_power_monomial(right, variable)?;

            Some(RationalPowerMonomial {
                variable: variable.to_owned(),
                coefficient: left.coefficient.mul(&right.coefficient),
                exponent: left.exponent.add(&right.exponent),
            })
        }
        SemanticTerm::Apply { symbol, args, .. } if symbol.as_str() == "core.rational.pow" => {
            let [base, exponent] = args.as_slice() else {
                return None;
            };
            let SemanticTerm::LocalVariable { name, .. } = base else {
                return None;
            };

            if name != variable {
                return None;
            }

            Some(RationalPowerMonomial {
                variable: variable.to_owned(),
                coefficient: ExactRational::integer(1),
                exponent: rational_constant(exponent)?,
            })
        }
        _ => None,
    }
}

fn rational_constant(term: &SemanticTerm) -> Option<ExactRational> {
    match term {
        SemanticTerm::RationalLiteral(value) => Some(value.clone()),
        SemanticTerm::Apply { symbol, args, .. } if symbol.as_str() == "core.rational.neg" => {
            let [operand] = args.as_slice() else {
                return None;
            };
            Some(rational_constant(operand)?.neg())
        }
        _ => None,
    }
}

fn render_rational_power_monomial(monomial: &RationalPowerMonomial) -> String {
    if monomial.coefficient.is_zero() {
        return "0".to_owned();
    }

    if monomial.exponent == ExactRational::integer(0) {
        return rational_latex(&monomial.coefficient);
    }

    let power_latex = if monomial.exponent == ExactRational::integer(1) {
        monomial.variable.clone()
    } else {
        format!(
            "{}^{{{}}}",
            monomial.variable,
            rational_exponent_latex(&monomial.exponent)
        )
    };

    if monomial.coefficient == ExactRational::integer(1) {
        power_latex
    } else if monomial.coefficient == ExactRational::integer(-1) {
        format!("-{power_latex}")
    } else {
        format!("{}{}", rational_latex(&monomial.coefficient), power_latex)
    }
}

fn rational_latex(value: &ExactRational) -> String {
    if value.denominator().to_string() == "1" {
        value.numerator().to_string()
    } else {
        format!("\\frac{{{}}}{{{}}}", value.numerator(), value.denominator())
    }
}

fn rational_exponent_latex(value: &ExactRational) -> String {
    if value.denominator().to_string() == "1" {
        value.numerator().to_string()
    } else {
        let numerator = value.numerator().to_string();

        if let Some(positive_numerator) = numerator.strip_prefix('-') {
            format!("-\\frac{{{positive_numerator}}}{{{}}}", value.denominator())
        } else {
            format!("\\frac{{{numerator}}}{{{}}}", value.denominator())
        }
    }
}

fn monomial(
    variable: String,
    degree: u32,
    coefficient: socrates_math_core::ExactRational,
) -> PolynomialExpression {
    let mut coefficients = BTreeMap::new();

    if !coefficient.is_zero() {
        coefficients.insert(degree, coefficient);
    }

    PolynomialExpression {
        variable,
        coefficients,
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
