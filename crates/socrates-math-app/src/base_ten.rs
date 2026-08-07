use socrates_math_core::ExactRational;
use socrates_math_protocol::{
    BaseTenDecompositionResponseDto, BaseTenPlaceDto, CompareBaseTenResponseDto, DiagnosticDto,
    ExactValueDto, MathematicalOutcomeKindDto,
};

const MIN_EXPONENT: i32 = -18;
const MAX_EXPONENT: i32 = 18;

pub fn decompose_response(
    value: &ExactValueDto,
    minimum_exponent: i32,
    maximum_exponent: i32,
) -> BaseTenDecompositionResponseDto {
    match decompose(value, minimum_exponent, maximum_exponent) {
        Ok((normalized, places)) => BaseTenDecompositionResponseDto {
            outcome: MathematicalOutcomeKindDto::Proven,
            value: Some(normalized),
            places,
            diagnostics: Vec::new(),
        },
        Err(diagnostic) => BaseTenDecompositionResponseDto {
            outcome: MathematicalOutcomeKindDto::Unknown,
            value: None,
            places: Vec::new(),
            diagnostics: vec![diagnostic],
        },
    }
}

pub fn compose_response(places: &[BaseTenPlaceDto]) -> BaseTenDecompositionResponseDto {
    match compose(places) {
        Ok(value) => BaseTenDecompositionResponseDto {
            outcome: MathematicalOutcomeKindDto::Proven,
            value: Some(ExactValueDto::from(&value)),
            places: canonical_places(places),
            diagnostics: Vec::new(),
        },
        Err(diagnostic) => BaseTenDecompositionResponseDto {
            outcome: MathematicalOutcomeKindDto::Unknown,
            value: None,
            places: Vec::new(),
            diagnostics: vec![diagnostic],
        },
    }
}

pub fn compare_response(
    expected: &ExactValueDto,
    submitted: &[BaseTenPlaceDto],
    minimum_exponent: i32,
    maximum_exponent: i32,
) -> CompareBaseTenResponseDto {
    let expected_decomposition = match decompose(expected, minimum_exponent, maximum_exponent) {
        Ok(value) => value,
        Err(diagnostic) => return comparison_unknown(diagnostic),
    };
    let submitted_value = match compose(submitted) {
        Ok(value) => value,
        Err(diagnostic) => return comparison_unknown(diagnostic),
    };
    let submitted_places = canonical_places(submitted);
    let equal = ExactValueDto::from(&submitted_value) == expected_decomposition.0;
    let diagnostics = if equal {
        Vec::new()
    } else {
        vec![classify_mismatch(
            &expected_decomposition.1,
            &submitted_places,
        )]
    };
    CompareBaseTenResponseDto {
        outcome: if equal {
            MathematicalOutcomeKindDto::Proven
        } else {
            MathematicalOutcomeKindDto::Disproven
        },
        relation: "number.base-ten-place-value".to_owned(),
        equal: Some(equal),
        expected_normalized: Some(expected_decomposition.0),
        submitted_normalized: Some(ExactValueDto::from(&submitted_value)),
        expected_places: expected_decomposition.1,
        submitted_places,
        diagnostics,
    }
}

fn decompose(
    value: &ExactValueDto,
    minimum_exponent: i32,
    maximum_exponent: i32,
) -> Result<(ExactValueDto, Vec<BaseTenPlaceDto>), DiagnosticDto> {
    validate_bounds(minimum_exponent, maximum_exponent)?;
    let value = exact(value)?;
    if value.numerator().to_string().starts_with('-') {
        return Err(diagnostic(
            "BaseTen.NegativeUnsupported",
            "version 1 base-ten decomposition supports nonnegative values only",
        ));
    }
    let scaled = value.mul(&power_of_ten(-minimum_exponent)?);
    if !scaled.is_integer() {
        return Err(diagnostic(
            "BaseTen.InsufficientFractionalPlaces",
            "the declared minimum exponent cannot represent this value exactly",
        ));
    }
    let digits = scaled.numerator().to_string();
    let capacity = usize::try_from(maximum_exponent - minimum_exponent + 1)
        .map_err(|_| diagnostic("BaseTen.InvalidBounds", "invalid exponent bounds"))?;
    if digits.len() > capacity {
        return Err(diagnostic(
            "BaseTen.InsufficientWholePlaces",
            "the declared maximum exponent cannot represent this value exactly",
        ));
    }
    let padded = format!("{:0>width$}", digits, width = capacity);
    let places = padded
        .bytes()
        .enumerate()
        .map(|(index, byte)| BaseTenPlaceDto {
            exponent: maximum_exponent - i32::try_from(index).expect("bounded index fits i32"),
            coefficient: byte - b'0',
        })
        .collect();
    Ok((ExactValueDto::from(&value), places))
}

fn compose(places: &[BaseTenPlaceDto]) -> Result<ExactRational, DiagnosticDto> {
    if places.is_empty() {
        return Err(diagnostic(
            "BaseTen.EmptyConstruction",
            "at least one semantic place is required",
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut result = ExactRational::integer(0);
    for place in places {
        if !(MIN_EXPONENT..=MAX_EXPONENT).contains(&place.exponent) {
            return Err(diagnostic(
                "BaseTen.ExponentOutsideEnvelope",
                "a place exponent is outside the supported version 1 envelope",
            ));
        }
        if place.coefficient > 9 {
            return Err(diagnostic(
                "BaseTen.InvalidCoefficient",
                "base-ten place coefficients must be digits from 0 through 9",
            ));
        }
        if !seen.insert(place.exponent) {
            return Err(diagnostic(
                "BaseTen.DuplicateExponent",
                "each base-ten exponent may occur at most once",
            ));
        }
        result = result.add(
            &ExactRational::integer(i64::from(place.coefficient))
                .mul(&power_of_ten(place.exponent)?),
        );
    }
    Ok(result)
}

fn power_of_ten(exponent: i32) -> Result<ExactRational, DiagnosticDto> {
    if !(MIN_EXPONENT..=MAX_EXPONENT).contains(&exponent) {
        return Err(diagnostic(
            "BaseTen.ExponentOutsideEnvelope",
            "a place exponent is outside the supported version 1 envelope",
        ));
    }
    let magnitude = ExactRational::integer(10).pow_u32(exponent.unsigned_abs());
    if exponent >= 0 {
        Ok(magnitude)
    } else {
        ExactRational::integer(1)
            .div(&magnitude)
            .map_err(|_| diagnostic("BaseTen.InvalidPower", "could not construct place unit"))
    }
}

fn exact(value: &ExactValueDto) -> Result<ExactRational, DiagnosticDto> {
    let parsed = match value {
        ExactValueDto::Integer { value } => ExactRational::parse_integer(value),
        ExactValueDto::Rational {
            numerator,
            denominator,
        } => ExactRational::parse_fraction(numerator, denominator),
    };
    parsed.map_err(|_| diagnostic("BaseTen.InvalidExactValue", "invalid exact value"))
}

fn validate_bounds(minimum: i32, maximum: i32) -> Result<(), DiagnosticDto> {
    if minimum > maximum || minimum < MIN_EXPONENT || maximum > MAX_EXPONENT {
        return Err(diagnostic(
            "BaseTen.InvalidBounds",
            "exponent bounds must be ordered and lie within -18 through 18",
        ));
    }
    Ok(())
}

fn canonical_places(places: &[BaseTenPlaceDto]) -> Vec<BaseTenPlaceDto> {
    let mut places = places.to_vec();
    places.sort_by_key(|place| std::cmp::Reverse(place.exponent));
    places
}

fn classify_mismatch(expected: &[BaseTenPlaceDto], submitted: &[BaseTenPlaceDto]) -> DiagnosticDto {
    if expected.len() == submitted.len()
        && expected.iter().zip(submitted).all(|(left, right)| {
            left.coefficient == right.coefficient && left.exponent == right.exponent + 1
        })
    {
        return diagnostic(
            "BaseTen.ShiftedOnePlaceRight",
            "every digit is one place too far right, making the value ten times smaller",
        );
    }
    if expected.len() == submitted.len()
        && expected.iter().zip(submitted).all(|(left, right)| {
            left.coefficient == right.coefficient && left.exponent == right.exponent - 1
        })
    {
        return diagnostic(
            "BaseTen.ShiftedOnePlaceLeft",
            "every digit is one place too far left, making the value ten times larger",
        );
    }
    diagnostic(
        "BaseTen.PlaceValueMismatch",
        "the submitted place coefficients reconstruct a different exact value",
    )
}

fn comparison_unknown(diagnostic: DiagnosticDto) -> CompareBaseTenResponseDto {
    CompareBaseTenResponseDto {
        outcome: MathematicalOutcomeKindDto::Unknown,
        relation: "number.base-ten-place-value".to_owned(),
        equal: None,
        expected_normalized: None,
        submitted_normalized: None,
        expected_places: Vec::new(),
        submitted_places: Vec::new(),
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

    #[test]
    fn decomposes_finite_decimals_without_floating_point() {
        let response = decompose_response(
            &ExactValueDto::Rational {
                numerator: "2461".to_owned(),
                denominator: "200".to_owned(),
            },
            -3,
            2,
        );
        assert_eq!(response.outcome, MathematicalOutcomeKindDto::Proven);
        assert_eq!(
            response.places,
            vec![
                BaseTenPlaceDto {
                    exponent: 2,
                    coefficient: 0
                },
                BaseTenPlaceDto {
                    exponent: 1,
                    coefficient: 1
                },
                BaseTenPlaceDto {
                    exponent: 0,
                    coefficient: 2
                },
                BaseTenPlaceDto {
                    exponent: -1,
                    coefficient: 3
                },
                BaseTenPlaceDto {
                    exponent: -2,
                    coefficient: 0
                },
                BaseTenPlaceDto {
                    exponent: -3,
                    coefficient: 5
                },
            ]
        );
    }

    #[test]
    fn reconstructs_and_grades_semantic_places() {
        let submitted = vec![
            BaseTenPlaceDto {
                exponent: 1,
                coefficient: 4,
            },
            BaseTenPlaceDto {
                exponent: 0,
                coefficient: 0,
            },
            BaseTenPlaceDto {
                exponent: -1,
                coefficient: 3,
            },
            BaseTenPlaceDto {
                exponent: -2,
                coefficient: 0,
            },
            BaseTenPlaceDto {
                exponent: -3,
                coefficient: 6,
            },
        ];
        let response = compare_response(
            &ExactValueDto::Rational {
                numerator: "20153".to_owned(),
                denominator: "500".to_owned(),
            },
            &submitted,
            -3,
            1,
        );
        assert_eq!(response.outcome, MathematicalOutcomeKindDto::Proven);
        assert_eq!(response.equal, Some(true));
    }

    #[test]
    fn rejects_values_that_do_not_fit_declared_fractional_places() {
        let response = decompose_response(
            &ExactValueDto::Rational {
                numerator: "1".to_owned(),
                denominator: "3".to_owned(),
            },
            -4,
            0,
        );
        assert_eq!(response.outcome, MathematicalOutcomeKindDto::Unknown);
        assert_eq!(
            response.diagnostics[0].code,
            "BaseTen.InsufficientFractionalPlaces"
        );
    }
}
