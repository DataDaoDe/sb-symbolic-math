use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ExactInteger(BigInt);

impl ExactInteger {
    pub fn parse(source: &str) -> Result<Self, ExactValueError> {
        BigInt::from_str(source)
            .map(Self)
            .map_err(|_| ExactValueError::InvalidIntegerLiteral(source.to_owned()))
    }

    pub fn from_i64(value: i64) -> Self {
        Self(BigInt::from(value))
    }

    pub fn inner(&self) -> &BigInt {
        &self.0
    }
}

impl fmt::Display for ExactInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ExactRational {
    numerator: BigInt,
    denominator: BigInt,
}

impl ExactRational {
    pub fn new(numerator: BigInt, denominator: BigInt) -> Result<Self, ExactValueError> {
        if denominator.is_zero() {
            return Err(ExactValueError::ZeroDenominator);
        }

        let gcd = numerator.gcd(&denominator);
        let mut numerator = numerator / &gcd;
        let mut denominator = denominator / gcd;

        if denominator.is_negative() {
            numerator = -numerator;
            denominator = -denominator;
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    pub fn integer(value: i64) -> Self {
        Self {
            numerator: BigInt::from(value),
            denominator: BigInt::one(),
        }
    }

    pub fn parse_integer(source: &str) -> Result<Self, ExactValueError> {
        Ok(Self {
            numerator: BigInt::from_str(source)
                .map_err(|_| ExactValueError::InvalidIntegerLiteral(source.to_owned()))?,
            denominator: BigInt::one(),
        })
    }

    pub fn parse_fraction(numerator: &str, denominator: &str) -> Result<Self, ExactValueError> {
        Self::new(
            BigInt::from_str(numerator)
                .map_err(|_| ExactValueError::InvalidIntegerLiteral(numerator.to_owned()))?,
            BigInt::from_str(denominator)
                .map_err(|_| ExactValueError::InvalidIntegerLiteral(denominator.to_owned()))?,
        )
    }

    /// Parses a plain exact numeric response: an integer, `a/b`, a decimal, or
    /// scientific notation. No floating-point conversion occurs.
    pub fn parse_number(source: &str) -> Result<Self, ExactValueError> {
        let source = source.trim();
        if let Some((numerator, denominator)) = source.split_once('/') {
            if denominator.contains('/') {
                return Err(ExactValueError::InvalidNumericLiteral(source.to_owned()));
            }
            return Self::parse_fraction(numerator.trim(), denominator.trim());
        }

        let (mantissa, exponent) = match source.find(['e', 'E']) {
            Some(index) => {
                let exponent = source[index + 1..]
                    .parse::<i32>()
                    .map_err(|_| ExactValueError::InvalidNumericLiteral(source.to_owned()))?;
                (&source[..index], exponent)
            }
            None => (source, 0),
        };
        if exponent.unsigned_abs() > 10_000 {
            return Err(ExactValueError::InvalidNumericLiteral(source.to_owned()));
        }

        let (negative, unsigned) = match mantissa.as_bytes().first() {
            Some(b'-') => (true, &mantissa[1..]),
            Some(b'+') => (false, &mantissa[1..]),
            _ => (false, mantissa),
        };
        let (whole, fractional) = match unsigned.split_once('.') {
            Some(parts) => parts,
            None => (unsigned, ""),
        };
        if whole.is_empty()
            || !whole.bytes().all(|byte| byte.is_ascii_digit())
            || !fractional.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(ExactValueError::InvalidNumericLiteral(source.to_owned()));
        }

        let digits = BigInt::from_str(&format!("{whole}{fractional}"))
            .map_err(|_| ExactValueError::InvalidNumericLiteral(source.to_owned()))?;
        let numerator = if negative { -digits } else { digits };
        let scale = i32::try_from(fractional.len())
            .map_err(|_| ExactValueError::InvalidNumericLiteral(source.to_owned()))?
            - exponent;
        if scale >= 0 {
            Self::new(numerator, BigInt::from(10u8).pow(scale as u32))
        } else {
            Self::new(
                numerator * BigInt::from(10u8).pow(scale.unsigned_abs()),
                BigInt::one(),
            )
        }
    }

    pub fn numerator(&self) -> &BigInt {
        &self.numerator
    }

    pub fn denominator(&self) -> &BigInt {
        &self.denominator
    }

    pub fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }

    pub fn abs(&self) -> Self {
        Self {
            numerator: self.numerator.abs(),
            denominator: self.denominator.clone(),
        }
    }

    pub fn cmp_exact(&self, rhs: &Self) -> Ordering {
        (&self.numerator * &rhs.denominator).cmp(&(&rhs.numerator * &self.denominator))
    }

    pub fn add(&self, rhs: &Self) -> Self {
        Self::new(
            &self.numerator * &rhs.denominator + &rhs.numerator * &self.denominator,
            &self.denominator * &rhs.denominator,
        )
        .expect("multiplying nonzero rational denominators cannot produce zero")
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        Self::new(
            &self.numerator * &rhs.denominator - &rhs.numerator * &self.denominator,
            &self.denominator * &rhs.denominator,
        )
        .expect("multiplying nonzero rational denominators cannot produce zero")
    }

    pub fn neg(&self) -> Self {
        Self {
            numerator: -&self.numerator,
            denominator: self.denominator.clone(),
        }
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        Self::new(
            &self.numerator * &rhs.numerator,
            &self.denominator * &rhs.denominator,
        )
        .expect("multiplying nonzero rational denominators cannot produce zero")
    }

    pub fn pow_u32(&self, exponent: u32) -> Self {
        Self::new(self.numerator.pow(exponent), self.denominator.pow(exponent))
            .expect("raising a nonzero rational denominator to a power cannot produce zero")
    }

    pub fn div(&self, rhs: &Self) -> Result<Self, ExactValueError> {
        if rhs.is_zero() {
            return Err(ExactValueError::DivisionByZero);
        }

        Self::new(
            &self.numerator * &rhs.denominator,
            &self.denominator * &rhs.numerator,
        )
    }

    pub fn is_integer(&self) -> bool {
        self.denominator.is_one()
    }
}

impl fmt::Display for ExactRational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator.is_one() {
            self.numerator.fmt(f)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactValueError {
    DivisionByZero,
    InvalidIntegerLiteral(String),
    InvalidNumericLiteral(String),
    ZeroDenominator,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_rationals() {
        let value = ExactRational::parse_fraction("10", "15").unwrap();
        assert_eq!(value.to_string(), "2/3");
    }

    #[test]
    fn keeps_denominator_positive() {
        let value = ExactRational::parse_fraction("4", "-6").unwrap();
        assert_eq!(value.to_string(), "-2/3");
    }

    #[test]
    fn rejects_zero_denominator() {
        assert_eq!(
            ExactRational::parse_fraction("1", "0"),
            Err(ExactValueError::ZeroDenominator)
        );
    }

    #[test]
    fn performs_exact_rational_arithmetic() {
        let half = ExactRational::parse_fraction("1", "2").unwrap();
        let third = ExactRational::parse_fraction("1", "3").unwrap();
        assert_eq!(half.add(&third).to_string(), "5/6");
    }

    #[test]
    fn parses_plain_decimal_and_scientific_notation_exactly() {
        assert_eq!(
            ExactRational::parse_number("0.75").unwrap().to_string(),
            "3/4"
        );
        assert_eq!(
            ExactRational::parse_number("-1.25e2").unwrap().to_string(),
            "-125"
        );
        assert_eq!(
            ExactRational::parse_number("6/8").unwrap().to_string(),
            "3/4"
        );
    }

    #[test]
    fn raises_exact_rationals_to_nonnegative_integer_powers() {
        let value = ExactRational::parse_fraction("-2", "3").unwrap();

        assert_eq!(value.pow_u32(3).to_string(), "-8/27");
        assert_eq!(value.pow_u32(0).to_string(), "1");
    }
}
