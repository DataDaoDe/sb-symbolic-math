use crate::{parse_and_elaborate_expression, real_domain, stable_fnv1a64, unit};
use socrates_math_algebra::{
    PolynomialExpression, RationalFunctionExpression, RationalFunctionNormalizer,
};
use socrates_math_core::ExactRational;
use socrates_math_protocol::{
    CompareRealFunctionsResponseDto, DiagnosticDto, EvaluateRealFunctionResponseDto,
    EvaluateRealFunctionTableResponseDto, ExactQuantityDto, ExactValueDto, MathContextDto,
    MathematicalOutcomeKindDto, RealDomainDto, RealDomainProvenanceDto, RealFunctionDto,
    RealFunctionSourceDto, RealFunctionTableRowDto, RealSetDto, TypedVariableDto,
    ValidateRealFunctionResponseDto, ValidatedMathExpressionDto,
};
use socrates_math_render::LatexRenderer;

const FUNCTION_SCHEMA: &str = "socrates.real-function";
const FUNCTION_VERSION: u32 = 1;
const REAL_TYPE: &str = "core.real.real";
const COMPLETENESS: &str = "single-variable explicit polynomials and rational functions over exact rational coefficients with constant or linear normalized denominators";

pub(crate) struct ValidatedInternal {
    pub(crate) dto: RealFunctionDto,
    pub(crate) formula: RationalFunctionExpression,
}

pub fn validate_response(source: &RealFunctionSourceDto) -> ValidateRealFunctionResponseDto {
    match validate(source) {
        Ok(validated) => ValidateRealFunctionResponseDto {
            outcome: MathematicalOutcomeKindDto::Proven,
            function: Some(validated.dto),
            diagnostics: Vec::new(),
        },
        Err((outcome, diagnostic)) => ValidateRealFunctionResponseDto {
            outcome,
            function: None,
            diagnostics: vec![diagnostic],
        },
    }
}

pub fn compare_response(
    left_source: &RealFunctionSourceDto,
    right_source: &RealFunctionSourceDto,
    relation: &str,
) -> CompareRealFunctionsResponseDto {
    let left = match validate(left_source) {
        Ok(value) => value,
        Err((outcome, diagnostic)) => return unknown_compare(outcome, relation, diagnostic),
    };
    let right = match validate(right_source) {
        Ok(value) => value,
        Err((outcome, diagnostic)) => return unknown_compare(outcome, relation, diagnostic),
    };
    let formula_equal = left.formula.formula_equal(&right.formula);
    let domains_equal = domain_equal(&left.dto.effective_domain, &right.dto.effective_domain);
    let left_subset = domain_subset(&left.dto.effective_domain, &right.dto.effective_domain);
    let right_subset = domain_subset(&right.dto.effective_domain, &left.dto.effective_domain);
    let holds = match relation {
        "function.equal" => formula_equal && domains_equal,
        "function.formula_equal_on_intersection" => formula_equal,
        "function.restriction_of" => formula_equal && left_subset,
        "function.extension_of" => formula_equal && right_subset,
        _ => {
            return unknown_compare(
                MathematicalOutcomeKindDto::Unknown,
                relation,
                diagnostic(
                    "Function.UnsupportedRelation",
                    "unsupported real-function relation",
                ),
            );
        }
    };
    let conditions = if relation == "function.formula_equal_on_intersection" {
        vec!["comparison is restricted to the effective-domain intersection".to_owned()]
    } else {
        Vec::new()
    };
    CompareRealFunctionsResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        relation: relation.to_owned(),
        holds: Some(holds),
        conditions,
        left: Some(left.dto),
        right: Some(right.dto),
        completeness: COMPLETENESS.to_owned(),
        diagnostics: Vec::new(),
    }
}

pub(crate) fn validate(
    source: &RealFunctionSourceDto,
) -> Result<ValidatedInternal, (MathematicalOutcomeKindDto, DiagnosticDto)> {
    if source.schema != FUNCTION_SCHEMA || source.version != FUNCTION_VERSION {
        return Err((
            MathematicalOutcomeKindDto::Unknown,
            diagnostic(
                "Function.UnsupportedProtocol",
                "real functions require schema socrates.real-function version 1",
            ),
        ));
    }
    if source.definition.kind != "explicit" || source.definition.input_format != "latex" {
        return Err((
            MathematicalOutcomeKindDto::Unknown,
            diagnostic(
                "Function.UnsupportedDefinition",
                "version 1 requires an explicit LaTeX definition",
            ),
        ));
    }
    if source.input.variable.trim().is_empty() {
        return Err((
            MathematicalOutcomeKindDto::Unknown,
            diagnostic(
                "Function.InvalidBinding",
                "the input variable must be explicit",
            ),
        ));
    }
    if !source.parameters.is_empty() || !source.assumptions.is_empty() {
        return Err((
            MathematicalOutcomeKindDto::Unknown,
            diagnostic(
                "Function.UnsupportedContext",
                "parameters and assumptions are outside this bounded function slice",
            ),
        ));
    }
    let input_unit = source
        .input
        .unit
        .as_ref()
        .map(unit::normalize)
        .transpose()
        .map_err(|diagnostic| (MathematicalOutcomeKindDto::Unknown, diagnostic))?;
    let output_unit = source
        .output
        .unit
        .as_ref()
        .map(unit::normalize)
        .transpose()
        .map_err(|diagnostic| (MathematicalOutcomeKindDto::Unknown, diagnostic))?;
    let term =
        parse_and_elaborate_expression(&source.definition.expression, &source.input.variable)
            .map_err(|diagnostic| (MathematicalOutcomeKindDto::Unknown, diagnostic))?;
    let formula =
        RationalFunctionNormalizer::normalize(&term, &source.input.variable).map_err(|_| {
            (
                MathematicalOutcomeKindDto::Unknown,
                diagnostic(
                    "Function.UnsupportedExpression",
                    "definition is outside the supported polynomial/rational function theory",
                ),
            )
        })?;
    if formula.denominator.is_zero() {
        return Err((
            MathematicalOutcomeKindDto::Undefined,
            diagnostic(
                "Function.ZeroDenominator",
                "the function definition has an identically zero denominator",
            ),
        ));
    }

    let excluded_points = denominator_roots(&formula.denominator)?;
    let natural_domain = normalized_domain(
        RealSetDto::Exclude {
            base: Box::new(RealSetDto::AllReal),
            points: excluded_points,
        },
        RealDomainProvenanceDto::Natural,
    )?;
    let declared_domain = source
        .declared_domain
        .as_ref()
        .map(normalize_domain)
        .transpose()?;
    let effective_domain = match &declared_domain {
        Some(declared) => intersect_domains(&natural_domain, declared)?,
        None => natural_domain.clone(),
    };
    let declared_codomain = source
        .declared_codomain
        .as_ref()
        .map(normalize_domain)
        .transpose()?;

    let variable = TypedVariableDto {
        symbol: source.input.variable.clone(),
        type_id: REAL_TYPE.to_owned(),
    };
    let canonical_latex = render_formula(&formula);
    let expression_fingerprint_payload = format!(
        "theory=algebra.rational-function.real|variable={}|canonical={canonical_latex}",
        source.input.variable
    );
    let expression = ValidatedMathExpressionDto {
        schema: "socrates.math.validated-expression".to_owned(),
        version: 1,
        source_latex: source.definition.expression.clone(),
        canonical_latex,
        theory: "algebra.rational-function.real".to_owned(),
        context: MathContextDto {
            theory_ids: vec!["algebra.rational-function.real".to_owned()],
            variables: vec![variable.clone()],
            assumptions: Vec::new(),
        },
        value_type: REAL_TYPE.to_owned(),
        free_variables: if is_constant(&formula) {
            Vec::new()
        } else {
            vec![variable.clone()]
        },
        semantic_fingerprint: fingerprint(&expression_fingerprint_payload),
    };
    let effective_json = serde_json::to_string(&effective_domain).expect("domain DTO serializes");
    let units_json =
        serde_json::to_string(&(&input_unit, &output_unit)).expect("unit DTOs serialize");
    let function_fingerprint = fingerprint(&format!(
        "expression={}|domain={effective_json}|input={REAL_TYPE}|output={REAL_TYPE}|units={units_json}",
        expression.semantic_fingerprint
    ));
    Ok(ValidatedInternal {
        dto: RealFunctionDto {
            schema: FUNCTION_SCHEMA.to_owned(),
            version: FUNCTION_VERSION,
            input: variable,
            input_label: source.input.label.clone(),
            input_unit,
            output_type: REAL_TYPE.to_owned(),
            output_label: source.output.label.clone(),
            output_unit,
            expression,
            natural_domain,
            declared_domain,
            effective_domain,
            declared_codomain,
            parameters: Vec::new(),
            assumptions: Vec::new(),
            semantic_fingerprint: function_fingerprint,
        },
        formula,
    })
}

pub fn evaluate_response(
    source: &RealFunctionSourceDto,
    input: &ExactQuantityDto,
) -> EvaluateRealFunctionResponseDto {
    let validated = match validate(source) {
        Ok(value) => value,
        Err((outcome, diagnostic)) => {
            return EvaluateRealFunctionResponseDto {
                outcome,
                value: None,
                function: None,
                completeness: COMPLETENESS.to_owned(),
                diagnostics: vec![diagnostic],
            };
        }
    };
    let evaluated = evaluate_input(&validated, input);
    EvaluateRealFunctionResponseDto {
        outcome: evaluated.outcome,
        value: evaluated.output,
        function: Some(validated.dto),
        completeness: COMPLETENESS.to_owned(),
        diagnostics: evaluated.diagnostics,
    }
}

pub fn evaluate_table_response(
    source: &RealFunctionSourceDto,
    inputs: &[ExactQuantityDto],
) -> EvaluateRealFunctionTableResponseDto {
    let validated = match validate(source) {
        Ok(value) => value,
        Err((outcome, diagnostic)) => {
            return EvaluateRealFunctionTableResponseDto {
                outcome,
                function: None,
                rows: Vec::new(),
                completeness: COMPLETENESS.to_owned(),
                diagnostics: vec![diagnostic],
            };
        }
    };
    let rows = inputs
        .iter()
        .map(|input| {
            let evaluated = evaluate_input(&validated, input);
            RealFunctionTableRowDto {
                input: input.clone(),
                outcome: evaluated.outcome,
                output: evaluated.output,
                diagnostics: evaluated.diagnostics,
            }
        })
        .collect();
    EvaluateRealFunctionTableResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        function: Some(validated.dto),
        rows,
        completeness: COMPLETENESS.to_owned(),
        diagnostics: Vec::new(),
    }
}

pub(crate) struct EvaluatedInput {
    pub(crate) outcome: MathematicalOutcomeKindDto,
    pub(crate) output: Option<ExactQuantityDto>,
    pub(crate) diagnostics: Vec<DiagnosticDto>,
}

pub(crate) fn coerce_input(
    validated: &ValidatedInternal,
    input: &ExactQuantityDto,
) -> Result<ExactRational, DiagnosticDto> {
    let authored_value = unit::exact(&input.value)?;
    match (&input.unit, &validated.dto.input_unit) {
        (None, None) => Ok(authored_value),
        (Some(from), Some(to)) => unit::convert_value(&authored_value, from, to),
        (None, Some(_)) => Err(diagnostic(
            "Unit.Required",
            "the function input requires a compatible unit",
        )),
        (Some(_), None) => Err(diagnostic(
            "Unit.Unexpected",
            "the function input is unitless",
        )),
    }
}

pub(crate) fn evaluate_input(
    validated: &ValidatedInternal,
    input: &ExactQuantityDto,
) -> EvaluatedInput {
    let value = match coerce_input(validated, input) {
        Ok(value) => value,
        Err(diagnostic) => {
            return failed_evaluation(MathematicalOutcomeKindDto::Unknown, diagnostic);
        }
    };
    let exact_dto = ExactValueDto::from(&value);
    let membership = real_domain::membership_response(&validated.dto.effective_domain, &exact_dto);
    if membership.outcome != MathematicalOutcomeKindDto::Proven {
        return EvaluatedInput {
            outcome: membership.outcome,
            output: None,
            diagnostics: membership.diagnostics,
        };
    }
    if membership.contains != Some(true) {
        return failed_evaluation(
            MathematicalOutcomeKindDto::Undefined,
            diagnostic(
                "Function.InputOutsideDomain",
                "the input is outside the function's effective domain",
            ),
        );
    }
    let numerator = evaluate_polynomial(&validated.formula.numerator, &value);
    let denominator = evaluate_polynomial(&validated.formula.denominator, &value);
    let result = match numerator.div(&denominator) {
        Ok(value) => value,
        Err(_) => {
            return failed_evaluation(
                MathematicalOutcomeKindDto::Undefined,
                diagnostic(
                    "Function.DivisionByZero",
                    "the function is undefined at this input",
                ),
            );
        }
    };
    EvaluatedInput {
        outcome: MathematicalOutcomeKindDto::Proven,
        output: Some(ExactQuantityDto {
            value: ExactValueDto::from(&result),
            unit: validated.dto.output_unit.clone(),
        }),
        diagnostics: Vec::new(),
    }
}

fn evaluate_polynomial(polynomial: &PolynomialExpression, input: &ExactRational) -> ExactRational {
    polynomial
        .coefficients
        .iter()
        .fold(ExactRational::integer(0), |sum, (degree, coefficient)| {
            sum.add(&coefficient.mul(&input.pow_u32(*degree)))
        })
}

fn failed_evaluation(
    outcome: MathematicalOutcomeKindDto,
    diagnostic: DiagnosticDto,
) -> EvaluatedInput {
    EvaluatedInput {
        outcome,
        output: None,
        diagnostics: vec![diagnostic],
    }
}

fn denominator_roots(
    denominator: &PolynomialExpression,
) -> Result<Vec<ExactValueDto>, (MathematicalOutcomeKindDto, DiagnosticDto)> {
    let degree = denominator
        .coefficients
        .keys()
        .next_back()
        .copied()
        .unwrap_or(0);
    match degree {
        0 => Ok(Vec::new()),
        1 => {
            let coefficient = denominator
                .coefficients
                .get(&1)
                .expect("degree one coefficient");
            let constant = denominator
                .coefficients
                .get(&0)
                .cloned()
                .unwrap_or_else(|| ExactRational::integer(0));
            let root = constant
                .neg()
                .div(coefficient)
                .expect("nonzero linear coefficient");
            Ok(vec![ExactValueDto::from(&root)])
        }
        _ => Err((
            MathematicalOutcomeKindDto::Unknown,
            diagnostic(
                "Function.UnsupportedDenominatorDomain",
                "version 1 domain derivation currently requires a constant or linear normalized denominator",
            ),
        )),
    }
}

fn normalize_domain(
    domain: &RealDomainDto,
) -> Result<RealDomainDto, (MathematicalOutcomeKindDto, DiagnosticDto)> {
    let response = real_domain::normalize_response(domain);
    response.domain.ok_or_else(|| {
        (
            response.outcome,
            response.diagnostics.into_iter().next().unwrap_or_else(|| {
                diagnostic("Function.InvalidDomain", "domain validation failed")
            }),
        )
    })
}

fn normalized_domain(
    set: RealSetDto,
    provenance: RealDomainProvenanceDto,
) -> Result<RealDomainDto, (MathematicalOutcomeKindDto, DiagnosticDto)> {
    normalize_domain(&RealDomainDto {
        schema: "socrates.real-domain".to_owned(),
        version: 1,
        set,
        provenance,
    })
}

fn intersect_domains(
    left: &RealDomainDto,
    right: &RealDomainDto,
) -> Result<RealDomainDto, (MathematicalOutcomeKindDto, DiagnosticDto)> {
    let response = real_domain::intersection_response(left, right);
    response.domain.ok_or_else(|| {
        (
            response.outcome,
            response.diagnostics.into_iter().next().unwrap_or_else(|| {
                diagnostic("Function.InvalidDomain", "domain intersection failed")
            }),
        )
    })
}

fn domain_equal(left: &RealDomainDto, right: &RealDomainDto) -> bool {
    real_domain::comparison_response(left, right).equal == Some(true)
}

fn domain_subset(left: &RealDomainDto, right: &RealDomainDto) -> bool {
    intersect_domains(left, right).is_ok_and(|intersection| domain_equal(&intersection, left))
}

fn is_constant(formula: &RationalFunctionExpression) -> bool {
    formula
        .numerator
        .coefficients
        .keys()
        .all(|degree| *degree == 0)
        && formula
            .denominator
            .coefficients
            .keys()
            .all(|degree| *degree == 0)
}

fn render_formula(formula: &RationalFunctionExpression) -> String {
    let numerator = LatexRenderer::polynomial_expression(&formula.numerator);
    if formula.denominator.coefficients.len() == 1
        && formula.denominator.coefficients.get(&0) == Some(&ExactRational::integer(1))
    {
        numerator
    } else {
        format!(
            "\\frac{{{numerator}}}{{{}}}",
            LatexRenderer::polynomial_expression(&formula.denominator)
        )
    }
}

fn fingerprint(payload: &str) -> String {
    format!("fnv1a64:{:016x}", stable_fnv1a64(payload.as_bytes()))
}

fn diagnostic(code: &str, message: &str) -> DiagnosticDto {
    DiagnosticDto {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

fn unknown_compare(
    outcome: MathematicalOutcomeKindDto,
    relation: &str,
    diagnostic: DiagnosticDto,
) -> CompareRealFunctionsResponseDto {
    CompareRealFunctionsResponseDto {
        outcome,
        relation: relation.to_owned(),
        holds: None,
        conditions: Vec::new(),
        left: None,
        right: None,
        completeness: COMPLETENESS.to_owned(),
        diagnostics: vec![diagnostic],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use socrates_math_protocol::{
        ExplicitFunctionDefinitionSourceDto, RealFunctionInputSourceDto,
        RealFunctionOutputSourceDto, UnitDimensionDto, UnitDto,
    };

    fn source(expression: &str, declared_domain: Option<RealDomainDto>) -> RealFunctionSourceDto {
        RealFunctionSourceDto {
            schema: FUNCTION_SCHEMA.to_owned(),
            version: FUNCTION_VERSION,
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
            declared_domain,
            declared_codomain: None,
            parameters: Vec::new(),
            assumptions: Vec::new(),
        }
    }

    fn nonnegative_domain() -> RealDomainDto {
        RealDomainDto {
            schema: "socrates.real-domain".to_owned(),
            version: 1,
            set: RealSetDto::Ray {
                direction: "above".to_owned(),
                boundary: ExactValueDto::Integer {
                    value: "0".to_owned(),
                },
                inclusive: true,
            },
            provenance: RealDomainProvenanceDto::Declared,
        }
    }

    #[test]
    fn same_formula_with_different_domains_is_not_the_same_function() {
        let unrestricted = source("x^2", None);
        let restricted = source("x^2", Some(nonnegative_domain()));
        let result = compare_response(&unrestricted, &restricted, "function.equal");
        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
        assert_eq!(result.holds, Some(false));
    }

    #[test]
    fn cancelled_rational_formula_retains_its_hole() {
        let rational = source("(x^2 - 1)/(x - 1)", None);
        let linear = source("x + 1", None);
        let validated = validate_response(&rational)
            .function
            .expect("valid function");
        let membership = real_domain::membership_response(
            &validated.effective_domain,
            &ExactValueDto::Integer {
                value: "1".to_owned(),
            },
        );
        assert_eq!(membership.contains, Some(false));

        let equal = compare_response(&rational, &linear, "function.equal");
        let formula_equal =
            compare_response(&rational, &linear, "function.formula_equal_on_intersection");
        assert_eq!(equal.holds, Some(false));
        assert_eq!(formula_equal.holds, Some(true));
        assert!(!formula_equal.conditions.is_empty());
    }

    #[test]
    fn explicit_binding_rejects_undeclared_identifiers() {
        let result = validate_response(&source("x + y", None));
        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
        assert_eq!(result.function, None);
        assert!(result.diagnostics[0].code.contains("UnknownSymbol"));
    }

    #[test]
    fn restriction_and_extension_relations_are_domain_sensitive() {
        let unrestricted = source("x^2", None);
        let restricted = source("x^2", Some(nonnegative_domain()));
        assert_eq!(
            compare_response(&restricted, &unrestricted, "function.restriction_of").holds,
            Some(true)
        );
        assert_eq!(
            compare_response(&unrestricted, &restricted, "function.extension_of").holds,
            Some(true)
        );
    }

    #[test]
    fn alpha_renamed_explicit_bindings_compare_by_meaning() {
        let left = source("x^2 + 1", None);
        let mut right = source("t^2 + 1", None);
        right.input.variable = "t".to_owned();
        assert_eq!(
            compare_response(&left, &right, "function.equal").holds,
            Some(true)
        );
    }

    fn quantity(value: &str) -> ExactQuantityDto {
        ExactQuantityDto {
            value: ExactValueDto::Integer {
                value: value.to_owned(),
            },
            unit: None,
        }
    }

    #[test]
    fn evaluates_rational_functions_exactly_and_distinguishes_undefined() {
        let rational = source("(x^2 - 1)/(x - 1)", None);
        let defined = evaluate_response(&rational, &quantity("2"));
        let undefined = evaluate_response(&rational, &quantity("1"));
        assert_eq!(defined.outcome, MathematicalOutcomeKindDto::Proven);
        assert_eq!(
            defined.value.expect("value").value,
            ExactValueDto::Integer {
                value: "3".to_owned()
            }
        );
        assert_eq!(undefined.outcome, MathematicalOutcomeKindDto::Undefined);
        assert_eq!(undefined.value, None);
    }

    #[test]
    fn table_preserves_input_order_and_duplicates() {
        let result = evaluate_table_response(
            &source("x + 1", None),
            &[quantity("2"), quantity("1"), quantity("2")],
        );
        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
        assert_eq!(result.rows.len(), 3);
        assert_eq!(result.rows[0].input, result.rows[2].input);
        assert_eq!(result.rows[0].output, result.rows[2].output);
        assert_ne!(result.rows[0].input, result.rows[1].input);
    }

    #[test]
    fn unsupported_denominator_domain_is_unknown() {
        let result = evaluate_response(&source("x/(x^2 + 1)", None), &quantity("1"));
        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
        assert_eq!(result.value, None);
        assert_eq!(
            result.diagnostics[0].code,
            "Function.UnsupportedDenominatorDomain"
        );
    }

    #[test]
    fn converts_compatible_input_units_before_exact_evaluation() {
        let mut function = source("x", None);
        function.input.unit = Some(UnitDto {
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
            symbol: "m".to_owned(),
        });
        let centimetre = UnitDto {
            schema: "socrates.unit".to_owned(),
            version: 1,
            dimensions: vec![UnitDimensionDto {
                base: "length".to_owned(),
                exponent: ExactValueDto::Integer {
                    value: "1".to_owned(),
                },
            }],
            scale_to_canonical: ExactValueDto::Rational {
                numerator: "1".to_owned(),
                denominator: "100".to_owned(),
            },
            symbol: "cm".to_owned(),
        };
        let result = evaluate_response(
            &function,
            &ExactQuantityDto {
                value: ExactValueDto::Integer {
                    value: "250".to_owned(),
                },
                unit: Some(centimetre),
            },
        );
        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
        assert_eq!(
            result.value.expect("value").value,
            ExactValueDto::Rational {
                numerator: "5".to_owned(),
                denominator: "2".to_owned()
            }
        );
    }
}
