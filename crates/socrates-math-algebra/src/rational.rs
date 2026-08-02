use crate::PolynomialExpression;
use socrates_math_core::{ExactRational, SemanticTerm, UnknownReason};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RationalFunctionExpression {
    pub numerator: PolynomialExpression,
    pub denominator: PolynomialExpression,
}

impl RationalFunctionExpression {
    pub fn formula_equal(&self, other: &Self) -> bool {
        let left_numerator = rebind(&self.numerator);
        let left_denominator = rebind(&self.denominator);
        let right_numerator = rebind(&other.numerator);
        let right_denominator = rebind(&other.denominator);
        match (
            left_numerator.mul(&right_denominator),
            right_numerator.mul(&left_denominator),
        ) {
            (Some(left), Some(right)) => left == right,
            _ => false,
        }
    }
}

fn rebind(polynomial: &PolynomialExpression) -> PolynomialExpression {
    PolynomialExpression {
        variable: "_input".to_owned(),
        coefficients: polynomial.coefficients.clone(),
    }
}

pub struct RationalFunctionNormalizer;

impl RationalFunctionNormalizer {
    pub fn normalize(
        term: &SemanticTerm,
        variable: &str,
    ) -> Result<RationalFunctionExpression, UnknownReason> {
        normalize_term(term, variable)
    }
}

fn normalize_term(
    term: &SemanticTerm,
    variable: &str,
) -> Result<RationalFunctionExpression, UnknownReason> {
    match term {
        SemanticTerm::RationalLiteral(value) => Ok(from_polynomial(
            PolynomialExpression::constant(variable, value.clone()),
            variable,
        )),
        SemanticTerm::LocalVariable { name, .. } if name == variable => Ok(from_polynomial(
            PolynomialExpression::variable(variable),
            variable,
        )),
        SemanticTerm::LocalVariable { .. } => Err(UnknownReason::UnsupportedDomain),
        SemanticTerm::Apply { symbol, args, .. } => match symbol.as_str() {
            "core.rational.add" => binary(args, variable, add),
            "core.rational.sub" => binary(args, variable, sub),
            "core.rational.mul" => binary(args, variable, mul),
            "core.rational.div" => binary(args, variable, div),
            "core.rational.neg" => {
                let [operand] = args.as_slice() else {
                    return Err(UnknownReason::UnsupportedDomain);
                };
                let mut value = normalize_term(operand, variable)?;
                value.numerator = value.numerator.neg();
                Ok(value)
            }
            "core.rational.pow" => normalize_power(args, variable),
            _ => Err(UnknownReason::UnsupportedDomain),
        },
    }
}

fn from_polynomial(polynomial: PolynomialExpression, variable: &str) -> RationalFunctionExpression {
    RationalFunctionExpression {
        numerator: polynomial,
        denominator: PolynomialExpression::constant(variable, ExactRational::integer(1)),
    }
}

fn binary(
    args: &[SemanticTerm],
    variable: &str,
    operation: fn(
        &RationalFunctionExpression,
        &RationalFunctionExpression,
    ) -> Option<RationalFunctionExpression>,
) -> Result<RationalFunctionExpression, UnknownReason> {
    let [left, right] = args else {
        return Err(UnknownReason::UnsupportedDomain);
    };
    operation(
        &normalize_term(left, variable)?,
        &normalize_term(right, variable)?,
    )
    .ok_or(UnknownReason::UnsupportedDomain)
}

fn add(
    left: &RationalFunctionExpression,
    right: &RationalFunctionExpression,
) -> Option<RationalFunctionExpression> {
    Some(RationalFunctionExpression {
        numerator: left
            .numerator
            .mul(&right.denominator)?
            .add(&right.numerator.mul(&left.denominator)?)?,
        denominator: left.denominator.mul(&right.denominator)?,
    })
}

fn sub(
    left: &RationalFunctionExpression,
    right: &RationalFunctionExpression,
) -> Option<RationalFunctionExpression> {
    add(
        left,
        &RationalFunctionExpression {
            numerator: right.numerator.neg(),
            denominator: right.denominator.clone(),
        },
    )
}

fn mul(
    left: &RationalFunctionExpression,
    right: &RationalFunctionExpression,
) -> Option<RationalFunctionExpression> {
    Some(RationalFunctionExpression {
        numerator: left.numerator.mul(&right.numerator)?,
        denominator: left.denominator.mul(&right.denominator)?,
    })
}

fn div(
    left: &RationalFunctionExpression,
    right: &RationalFunctionExpression,
) -> Option<RationalFunctionExpression> {
    if right.numerator.is_zero() {
        return None;
    }
    Some(RationalFunctionExpression {
        numerator: left.numerator.mul(&right.denominator)?,
        denominator: left.denominator.mul(&right.numerator)?,
    })
}

fn normalize_power(
    args: &[SemanticTerm],
    variable: &str,
) -> Result<RationalFunctionExpression, UnknownReason> {
    let [base, exponent] = args else {
        return Err(UnknownReason::UnsupportedDomain);
    };
    let SemanticTerm::RationalLiteral(exponent) = exponent else {
        return Err(UnknownReason::UnsupportedDomain);
    };
    if !exponent.is_integer() {
        return Err(UnknownReason::UnsupportedDomain);
    }
    let exponent = exponent
        .numerator()
        .to_string()
        .parse::<u32>()
        .map_err(|_| UnknownReason::UnsupportedDomain)?;
    let base = normalize_term(base, variable)?;
    Ok(RationalFunctionExpression {
        numerator: base
            .numerator
            .pow_u32(exponent)
            .ok_or(UnknownReason::UnsupportedDomain)?,
        denominator: base
            .denominator
            .pow_u32(exponent)
            .ok_or(UnknownReason::UnsupportedDomain)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use socrates_math_core::{SymbolId, TypeRef};

    fn symbol(id: &str) -> SymbolId {
        SymbolId::new(id).unwrap()
    }

    #[test]
    fn cancellation_does_not_erase_the_original_denominator() {
        let x = SemanticTerm::variable("x", TypeRef::rational());
        let one = SemanticTerm::rational(ExactRational::integer(1));
        let numerator = SemanticTerm::apply(
            symbol("core.rational.sub"),
            vec![
                SemanticTerm::apply(
                    symbol("core.rational.pow"),
                    vec![x.clone(), SemanticTerm::rational(ExactRational::integer(2))],
                    TypeRef::rational(),
                ),
                one.clone(),
            ],
            TypeRef::rational(),
        );
        let denominator = SemanticTerm::apply(
            symbol("core.rational.sub"),
            vec![x, one],
            TypeRef::rational(),
        );
        let term = SemanticTerm::apply(
            symbol("core.rational.div"),
            vec![numerator, denominator],
            TypeRef::rational(),
        );
        let normalized = RationalFunctionNormalizer::normalize(&term, "x").unwrap();
        assert!(!normalized.denominator.is_zero());
        assert_eq!(normalized.denominator.coefficients.len(), 2);
    }
}
