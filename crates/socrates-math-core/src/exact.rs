use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use serde::{Deserialize, Serialize};
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

    pub fn numerator(&self) -> &BigInt {
        &self.numerator
    }

    pub fn denominator(&self) -> &BigInt {
        &self.denominator
    }

    pub fn is_zero(&self) -> bool {
        self.numerator.is_zero()
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
    fn raises_exact_rationals_to_nonnegative_integer_powers() {
        let value = ExactRational::parse_fraction("-2", "3").unwrap();

        assert_eq!(value.pow_u32(3).to_string(), "-8/27");
        assert_eq!(value.pow_u32(0).to_string(), "1");
    }
}
