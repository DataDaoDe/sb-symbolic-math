use serde::{Deserialize, Serialize};
use socrates_math_core::ExactRational;

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
}
