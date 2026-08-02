use serde::{Deserialize, Serialize};
use socrates_math_core::ExactRational;
use socrates_math_solve::SolutionSet;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ExactValueDto {
    #[serde(rename = "integer")]
    Integer { value: String },
    #[serde(rename = "rational")]
    Rational {
        numerator: String,
        denominator: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RealDomainProvenanceDto {
    Declared,
    Natural,
    Contextual,
    Restricted,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RealSetDto {
    Empty,
    AllReal,
    Point {
        value: ExactValueDto,
    },
    Interval {
        lower: ExactValueDto,
        upper: ExactValueDto,
        lower_inclusive: bool,
        upper_inclusive: bool,
    },
    Ray {
        direction: String,
        boundary: ExactValueDto,
        inclusive: bool,
    },
    Union {
        members: Vec<RealSetDto>,
    },
    Exclude {
        base: Box<RealSetDto>,
        points: Vec<ExactValueDto>,
    },
    SetBuilder {
        source: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RealDomainDto {
    pub schema: String,
    pub version: u32,
    pub set: RealSetDto,
    pub provenance: RealDomainProvenanceDto,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NormalizeRealDomainResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub domain: Option<RealDomainDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompareRealDomainsResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub equal: Option<bool>,
    pub left_normalized: Option<RealDomainDto>,
    pub right_normalized: Option<RealDomainDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RealDomainMembershipResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub contains: Option<bool>,
    pub normalized_domain: Option<RealDomainDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

impl From<&ExactRational> for ExactValueDto {
    fn from(value: &ExactRational) -> Self {
        if value.denominator().to_string() == "1" {
            Self::Integer {
                value: value.numerator().to_string(),
            }
        } else {
            Self::Rational {
                numerator: value.numerator().to_string(),
                denominator: value.denominator().to_string(),
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StableIdentifierDto {
    pub id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SolutionSetDto {
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "unique")]
    Unique { value: ExactValueDto },
    #[serde(rename = "all-rationals")]
    AllRationals,
}

impl From<&SolutionSet> for SolutionSetDto {
    fn from(value: &SolutionSet) -> Self {
        match value {
            SolutionSet::Empty => Self::Empty,
            SolutionSet::Unique(value) => Self::Unique {
                value: ExactValueDto::from(value),
            },
            SolutionSet::AllRationals => Self::AllRationals,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SolveLinearEquationResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub variable: String,
    pub solution_set: Option<SolutionSetDto>,
    pub solution_set_latex: Option<String>,
    pub completeness: Option<String>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompareEquationsResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub equal: Option<bool>,
    pub left_solution_set: Option<SolutionSetDto>,
    pub right_solution_set: Option<SolutionSetDto>,
    pub left_solution_set_latex: Option<String>,
    pub right_solution_set_latex: Option<String>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MathExpressionDto {
    pub latex: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProtocolCapabilityDto {
    pub id: String,
    pub version: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProtocolManifestDto {
    pub schema: String,
    pub version: u32,
    pub capabilities: Vec<ProtocolCapabilityDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypedVariableDto {
    pub symbol: String,
    pub type_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MathContextDto {
    pub theory_ids: Vec<String>,
    pub variables: Vec<TypedVariableDto>,
    pub assumptions: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ValidatedMathExpressionDto {
    pub schema: String,
    pub version: u32,
    pub source_latex: String,
    pub canonical_latex: String,
    pub theory: String,
    pub context: MathContextDto,
    pub value_type: String,
    pub free_variables: Vec<TypedVariableDto>,
    pub semantic_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ValidateMathExpressionResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub expression: Option<ValidatedMathExpressionDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RealFunctionInputSourceDto {
    pub variable: String,
    pub label: Option<String>,
    pub unit: Option<UnitDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RealFunctionOutputSourceDto {
    pub label: Option<String>,
    pub unit: Option<UnitDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnitDimensionDto {
    pub base: String,
    pub exponent: ExactValueDto,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnitDto {
    pub schema: String,
    pub version: u32,
    pub dimensions: Vec<UnitDimensionDto>,
    pub scale_to_canonical: ExactValueDto,
    pub symbol: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExactQuantityDto {
    pub value: ExactValueDto,
    pub unit: Option<UnitDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExplicitFunctionDefinitionSourceDto {
    pub kind: String,
    pub expression: String,
    pub input_format: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RealFunctionSourceDto {
    pub schema: String,
    pub version: u32,
    pub input: RealFunctionInputSourceDto,
    pub output: RealFunctionOutputSourceDto,
    pub definition: ExplicitFunctionDefinitionSourceDto,
    pub declared_domain: Option<RealDomainDto>,
    pub declared_codomain: Option<RealDomainDto>,
    pub parameters: Vec<String>,
    pub assumptions: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RealFunctionDto {
    pub schema: String,
    pub version: u32,
    pub input: TypedVariableDto,
    pub input_label: Option<String>,
    pub input_unit: Option<UnitDto>,
    pub output_type: String,
    pub output_label: Option<String>,
    pub output_unit: Option<UnitDto>,
    pub expression: ValidatedMathExpressionDto,
    pub natural_domain: RealDomainDto,
    pub declared_domain: Option<RealDomainDto>,
    pub effective_domain: RealDomainDto,
    pub declared_codomain: Option<RealDomainDto>,
    pub parameters: Vec<String>,
    pub assumptions: Vec<String>,
    pub semantic_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvaluateRealFunctionResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub value: Option<ExactQuantityDto>,
    pub function: Option<RealFunctionDto>,
    pub completeness: String,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RealFunctionTableRowDto {
    pub input: ExactQuantityDto,
    pub outcome: MathematicalOutcomeKindDto,
    pub output: Option<ExactQuantityDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvaluateRealFunctionTableResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub function: Option<RealFunctionDto>,
    pub rows: Vec<RealFunctionTableRowDto>,
    pub completeness: String,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AverageRateResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub value: Option<ExactQuantityDto>,
    pub left_input: ExactQuantityDto,
    pub right_input: ExactQuantityDto,
    pub function: Option<RealFunctionDto>,
    pub completeness: String,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DifferenceQuotientResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub increment_variable: String,
    pub conditions: Vec<String>,
    pub initial: Option<MathExpressionDto>,
    pub result: Option<MathExpressionDto>,
    pub result_unit: Option<UnitDto>,
    pub applicable_rules: Vec<String>,
    pub steps: Vec<MathDerivationStepDto>,
    pub completeness: String,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ApplyDifferenceQuotientRuleResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub rule: String,
    pub conditions: Vec<String>,
    pub previous: Option<MathExpressionDto>,
    pub result: Option<MathExpressionDto>,
    pub step: Option<MathDerivationStepDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ValidateRealFunctionResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub function: Option<RealFunctionDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompareRealFunctionsResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub holds: Option<bool>,
    pub conditions: Vec<String>,
    pub left: Option<RealFunctionDto>,
    pub right: Option<RealFunctionDto>,
    pub completeness: String,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SetExpressionDto {
    pub latex: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SetStatementDto {
    pub latex: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NormalizeMathExpressionResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub normalized: Option<MathExpressionDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompareMathExpressionsResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub equal: Option<bool>,
    pub left_normalized: Option<MathExpressionDto>,
    pub right_normalized: Option<MathExpressionDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NormalizeSetExpressionResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub normalized: Option<SetExpressionDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompareSetExpressionsResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub equal: Option<bool>,
    pub left_normalized: Option<SetExpressionDto>,
    pub right_normalized: Option<SetExpressionDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SetBindingDto {
    pub symbol: String,
    pub expression: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvaluateSetStatementResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub truth: Option<bool>,
    pub normalized: Option<SetStatementDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvaluateSetCardinalityResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub cardinality: Option<u64>,
    pub cardinality_latex: Option<String>,
    pub normalized_set: Option<SetExpressionDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvaluateFiniteRelationPredicateResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub truth: Option<bool>,
    pub normalized_relation: Option<SetExpressionDto>,
    pub normalized_domain: Option<SetExpressionDto>,
    pub normalized_codomain: Option<SetExpressionDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompareNumericAnswerResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub equal: Option<bool>,
    pub submitted_normalized: Option<ExactValueDto>,
    pub expected_normalized: Option<ExactValueDto>,
    pub absolute_error: Option<ExactValueDto>,
    pub accepted_tolerance: Option<ExactValueDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MathDerivationStepDto {
    pub rule: String,
    pub reason: String,
    pub target: Option<RuleTargetDto>,
    pub input_latex: Option<String>,
    pub output_latex: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TransformMathExpressionResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub result: Option<MathExpressionDto>,
    pub steps: Vec<MathDerivationStepDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum RuleTargetDto {
    #[serde(rename = "whole")]
    Whole,
    #[serde(rename = "polynomial-term")]
    PolynomialTerm { degree: u32 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RuleApplicabilityStatusDto {
    Applicable,
    ApplicableWithConditions,
    NotApplicable,
    AmbiguousTarget,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ApplicableRuleDto {
    pub rule: String,
    pub status: RuleApplicabilityStatusDto,
    pub relation: String,
    pub target: Option<RuleTargetDto>,
    pub reason: String,
    pub required_conditions: Vec<String>,
    pub concepts: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ListApplicableRulesResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub rules: Vec<ApplicableRuleDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ApplyRuleResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub previous: Option<MathExpressionDto>,
    pub result: Option<MathExpressionDto>,
    pub step: Option<MathDerivationStepDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ApplyLinearEquationRuleResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub previous_latex: Option<String>,
    pub result_latex: Option<String>,
    pub step: Option<MathDerivationStepDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RunLinearEquationStrategyResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub strategy: String,
    pub initial_latex: String,
    pub result_latex: Option<String>,
    pub steps: Vec<MathDerivationStepDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MathematicalOutcomeKindDto {
    Proven,
    Disproven,
    Conditional,
    Unknown,
    Undefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticDto {
    pub code: String,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_rational_as_tagged_strings() {
        let value = ExactRational::parse_fraction("10", "12").unwrap();

        assert_eq!(
            ExactValueDto::from(&value),
            ExactValueDto::Rational {
                numerator: "5".to_owned(),
                denominator: "6".to_owned()
            }
        );
    }

    #[test]
    fn serializes_integer_rational_as_integer_tag() {
        let value = ExactRational::parse_integer("42").unwrap();

        assert_eq!(
            ExactValueDto::from(&value),
            ExactValueDto::Integer {
                value: "42".to_owned()
            }
        );
    }

    #[test]
    fn serializes_solution_set_with_exact_value() {
        let solution_set = SolutionSet::Unique(ExactRational::parse_fraction("2", "4").unwrap());

        assert_eq!(
            SolutionSetDto::from(&solution_set),
            SolutionSetDto::Unique {
                value: ExactValueDto::Rational {
                    numerator: "1".to_owned(),
                    denominator: "2".to_owned()
                }
            }
        );
    }
}
