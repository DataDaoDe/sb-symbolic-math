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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompareNumericAnswerResponseDto {
    pub outcome: MathematicalOutcomeKindDto,
    pub relation: String,
    pub equal: Option<bool>,
    pub submitted_value: Option<f64>,
    pub expected_value: Option<f64>,
    pub absolute_error: Option<f64>,
    pub tolerance: f64,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MathDerivationStepDto {
    pub rule: String,
    pub reason: String,
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
