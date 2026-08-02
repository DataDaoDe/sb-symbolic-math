use socrates_math_core::ExactRational;
use socrates_math_protocol::{DiagnosticDto, ExactValueDto, UnitDimensionDto, UnitDto};
use std::cmp::Ordering;
use std::collections::BTreeMap;

const UNIT_SCHEMA: &str = "socrates.unit";
const UNIT_VERSION: u32 = 1;
const BASES: &[&str] = &[
    "amount",
    "angle",
    "electric_current",
    "length",
    "luminous_intensity",
    "mass",
    "temperature",
    "time",
];

pub fn normalize(unit: &UnitDto) -> Result<UnitDto, DiagnosticDto> {
    if unit.schema != UNIT_SCHEMA || unit.version != UNIT_VERSION {
        return Err(diagnostic(
            "Unit.UnsupportedProtocol",
            "units require schema socrates.unit version 1",
        ));
    }
    let scale = exact(&unit.scale_to_canonical)?;
    if scale.cmp_exact(&ExactRational::integer(0)) != Ordering::Greater {
        return Err(diagnostic(
            "Unit.InvalidScale",
            "multiplicative unit scale must be positive",
        ));
    }
    let mut dimensions: BTreeMap<String, ExactRational> = BTreeMap::new();
    for dimension in &unit.dimensions {
        if !BASES.contains(&dimension.base.as_str()) {
            return Err(diagnostic(
                "Unit.UnsupportedDimension",
                "unit contains an unsupported base dimension",
            ));
        }
        let exponent = exact(&dimension.exponent)?;
        let previous = dimensions
            .remove(&dimension.base)
            .unwrap_or_else(|| ExactRational::integer(0));
        let combined = previous.add(&exponent);
        if !combined.is_zero() {
            dimensions.insert(dimension.base.clone(), combined);
        }
    }
    Ok(UnitDto {
        schema: UNIT_SCHEMA.to_owned(),
        version: UNIT_VERSION,
        dimensions: dimensions
            .into_iter()
            .map(|(base, exponent)| UnitDimensionDto {
                base,
                exponent: ExactValueDto::from(&exponent),
            })
            .collect(),
        scale_to_canonical: ExactValueDto::from(&scale),
        symbol: unit.symbol.clone(),
    })
}

pub fn convert_value(
    value: &ExactRational,
    from: &UnitDto,
    to: &UnitDto,
) -> Result<ExactRational, DiagnosticDto> {
    let from = normalize(from)?;
    let to = normalize(to)?;
    if from.dimensions != to.dimensions {
        return Err(diagnostic(
            "Unit.IncompatibleDimensions",
            "quantity unit is not compatible with the function input unit",
        ));
    }
    let from_scale = exact(&from.scale_to_canonical)?;
    let to_scale = exact(&to.scale_to_canonical)?;
    value
        .mul(&from_scale)
        .div(&to_scale)
        .map_err(|error| diagnostic("Unit.InvalidScale", &format!("{error:?}")))
}

pub fn quotient(
    output: Option<&UnitDto>,
    input: Option<&UnitDto>,
) -> Result<Option<UnitDto>, DiagnosticDto> {
    if output.is_none() && input.is_none() {
        return Ok(None);
    }
    let output = output.map(normalize).transpose()?;
    let input = input.map(normalize).transpose()?;
    let mut dimensions = output
        .as_ref()
        .map_or_else(Vec::new, |unit| unit.dimensions.clone());
    if let Some(input) = &input {
        dimensions.extend(input.dimensions.iter().map(|dimension| {
            UnitDimensionDto {
                base: dimension.base.clone(),
                exponent: ExactValueDto::from(
                    &exact(&dimension.exponent)
                        .expect("normalized exponent")
                        .neg(),
                ),
            }
        }));
    }
    let output_scale = output
        .as_ref()
        .map(|unit| exact(&unit.scale_to_canonical).expect("normalized scale"))
        .unwrap_or_else(|| ExactRational::integer(1));
    let input_scale = input
        .as_ref()
        .map(|unit| exact(&unit.scale_to_canonical).expect("normalized scale"))
        .unwrap_or_else(|| ExactRational::integer(1));
    let scale = output_scale
        .div(&input_scale)
        .expect("positive scale is nonzero");
    let symbol = match (&output, &input) {
        (Some(output), Some(input)) => format!("{}/{}", output.symbol, input.symbol),
        (Some(output), None) => output.symbol.clone(),
        (None, Some(input)) => format!("1/{}", input.symbol),
        (None, None) => unreachable!(),
    };
    normalize(&UnitDto {
        schema: UNIT_SCHEMA.to_owned(),
        version: UNIT_VERSION,
        dimensions,
        scale_to_canonical: ExactValueDto::from(&scale),
        symbol,
    })
    .map(Some)
}

pub fn exact(value: &ExactValueDto) -> Result<ExactRational, DiagnosticDto> {
    let parsed = match value {
        ExactValueDto::Integer { value } => ExactRational::parse_integer(value),
        ExactValueDto::Rational {
            numerator,
            denominator,
        } => ExactRational::parse_fraction(numerator, denominator),
    };
    parsed.map_err(|error| diagnostic("Value.InvalidExactValue", &format!("{error:?}")))
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

    fn length(symbol: &str, scale: &str) -> UnitDto {
        UnitDto {
            schema: UNIT_SCHEMA.to_owned(),
            version: UNIT_VERSION,
            dimensions: vec![UnitDimensionDto {
                base: "length".to_owned(),
                exponent: ExactValueDto::Integer {
                    value: "1".to_owned(),
                },
            }],
            scale_to_canonical: ExactValueDto::Rational {
                numerator: scale.to_owned(),
                denominator: "1".to_owned(),
            },
            symbol: symbol.to_owned(),
        }
    }

    #[test]
    fn converts_compatible_multiplicative_units_exactly() {
        let centimetre = length("cm", "1");
        let metre = length("m", "100");
        let converted = convert_value(&ExactRational::integer(250), &centimetre, &metre).unwrap();
        assert_eq!(converted, ExactRational::parse_fraction("5", "2").unwrap());
    }
}
