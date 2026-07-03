use serde::{Deserialize, Serialize};
use socrates_math_core::{
    ExactRational, MathematicalOutcome, SemanticTerm, Unknown, UnknownReason, Verified,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PolynomialExpression {
    pub variable: String,
    pub coefficients: BTreeMap<u32, ExactRational>,
}

impl PolynomialExpression {
    pub fn constant(variable: impl Into<String>, constant: ExactRational) -> Self {
        let mut coefficients = BTreeMap::new();
        insert_coefficient(&mut coefficients, 0, constant);

        Self {
            variable: variable.into(),
            coefficients,
        }
    }

    pub fn variable(variable: impl Into<String>) -> Self {
        let variable = variable.into();
        let mut coefficients = BTreeMap::new();
        insert_coefficient(&mut coefficients, 1, ExactRational::integer(1));

        Self {
            variable,
            coefficients,
        }
    }

    pub fn add(&self, rhs: &Self) -> Option<Self> {
        if self.variable != rhs.variable {
            return None;
        }

        let mut coefficients = self.coefficients.clone();

        for (degree, coefficient) in &rhs.coefficients {
            let existing = coefficients
                .get(degree)
                .cloned()
                .unwrap_or_else(|| ExactRational::integer(0));
            insert_coefficient(&mut coefficients, *degree, existing.add(coefficient));
        }

        Some(Self {
            variable: self.variable.clone(),
            coefficients,
        })
    }

    pub fn sub(&self, rhs: &Self) -> Option<Self> {
        self.add(&rhs.neg())
    }

    pub fn neg(&self) -> Self {
        Self {
            variable: self.variable.clone(),
            coefficients: self
                .coefficients
                .iter()
                .map(|(degree, coefficient)| (*degree, coefficient.neg()))
                .collect(),
        }
    }

    pub fn mul(&self, rhs: &Self) -> Option<Self> {
        if self.variable != rhs.variable {
            return None;
        }

        let mut coefficients = BTreeMap::new();

        for (left_degree, left_coefficient) in &self.coefficients {
            for (right_degree, right_coefficient) in &rhs.coefficients {
                let degree = left_degree.checked_add(*right_degree)?;
                let existing = coefficients
                    .get(&degree)
                    .cloned()
                    .unwrap_or_else(|| ExactRational::integer(0));
                insert_coefficient(
                    &mut coefficients,
                    degree,
                    existing.add(&left_coefficient.mul(right_coefficient)),
                );
            }
        }

        Some(Self {
            variable: self.variable.clone(),
            coefficients,
        })
    }

    pub fn pow_u32(&self, exponent: u32) -> Option<Self> {
        let mut result = Self::constant(self.variable.clone(), ExactRational::integer(1));

        for _ in 0..exponent {
            result = result.mul(self)?;
        }

        Some(result)
    }

    pub fn derivative(&self) -> Self {
        let mut coefficients = BTreeMap::new();

        for (degree, coefficient) in &self.coefficients {
            if *degree == 0 {
                continue;
            }

            let degree_factor = ExactRational::integer(i64::from(*degree));
            insert_coefficient(
                &mut coefficients,
                degree - 1,
                coefficient.mul(&degree_factor),
            );
        }

        Self {
            variable: self.variable.clone(),
            coefficients,
        }
    }

    pub fn antiderivative(&self) -> Self {
        let mut coefficients = BTreeMap::new();

        for (degree, coefficient) in &self.coefficients {
            let antiderivative_degree = degree + 1;
            let degree_factor = ExactRational::integer(i64::from(antiderivative_degree));
            let antiderivative_coefficient = coefficient
                .div(&degree_factor)
                .expect("positive integer degree factors are nonzero");
            insert_coefficient(
                &mut coefficients,
                antiderivative_degree,
                antiderivative_coefficient,
            );
        }

        Self {
            variable: self.variable.clone(),
            coefficients,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PolynomialNormalization {
    pub source: SemanticTerm,
    pub normal_form: PolynomialExpression,
    pub relation: String,
    pub theory: String,
    pub completeness_domain: String,
}

pub struct PolynomialNormalizer;

impl PolynomialNormalizer {
    pub fn normalize(
        term: &SemanticTerm,
        variable: &str,
    ) -> MathematicalOutcome<PolynomialNormalization> {
        match normalize_term(term, variable) {
            Ok(normal_form) => MathematicalOutcome::Proven(Verified {
                value: PolynomialNormalization {
                    source: term.clone(),
                    normal_form,
                    relation: "logic.equal".to_owned(),
                    theory: "algebra.polynomial-expression@0.1".to_owned(),
                    completeness_domain:
                        "single-variable polynomials over exact rational coefficients".to_owned(),
                },
                evidence_id: Some("algebra.polynomial-expression.normalize.rational".to_owned()),
            }),
            Err(reason) => MathematicalOutcome::Unknown(Unknown { reason }),
        }
    }
}

fn normalize_term(
    term: &SemanticTerm,
    variable: &str,
) -> Result<PolynomialExpression, UnknownReason> {
    match term {
        SemanticTerm::RationalLiteral(value) => {
            Ok(PolynomialExpression::constant(variable, value.clone()))
        }
        SemanticTerm::LocalVariable { name, .. } if name == variable => {
            Ok(PolynomialExpression::variable(variable))
        }
        SemanticTerm::LocalVariable { .. } => Err(UnknownReason::UnsupportedDomain),
        SemanticTerm::Apply { symbol, args, .. } => match symbol.as_str() {
            "core.rational.add" => normalize_binary(args, variable, PolynomialExpression::add),
            "core.rational.sub" => normalize_binary(args, variable, PolynomialExpression::sub),
            "core.rational.mul" => normalize_binary(args, variable, PolynomialExpression::mul),
            "core.rational.neg" => {
                let [operand] = args.as_slice() else {
                    return Err(UnknownReason::UnsupportedDomain);
                };
                Ok(normalize_term(operand, variable)?.neg())
            }
            "core.rational.pow" => normalize_power(args, variable),
            _ => Err(UnknownReason::UnsupportedDomain),
        },
    }
}

fn normalize_binary(
    args: &[SemanticTerm],
    variable: &str,
    operation: fn(&PolynomialExpression, &PolynomialExpression) -> Option<PolynomialExpression>,
) -> Result<PolynomialExpression, UnknownReason> {
    let [left, right] = args else {
        return Err(UnknownReason::UnsupportedDomain);
    };

    operation(
        &normalize_term(left, variable)?,
        &normalize_term(right, variable)?,
    )
    .ok_or(UnknownReason::UnsupportedDomain)
}

fn normalize_power(
    args: &[SemanticTerm],
    variable: &str,
) -> Result<PolynomialExpression, UnknownReason> {
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

    normalize_term(base, variable)?
        .pow_u32(exponent)
        .ok_or(UnknownReason::UnsupportedDomain)
}

fn insert_coefficient(
    coefficients: &mut BTreeMap<u32, ExactRational>,
    degree: u32,
    coefficient: ExactRational,
) {
    if coefficient.is_zero() {
        coefficients.remove(&degree);
    } else {
        coefficients.insert(degree, coefficient);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use socrates_math_core::{Context, Declaration, TypeId};
    use socrates_math_elab::{ElaborationOutcome, Elaborator};
    use socrates_math_syntax::{ParseOutcome, Parser};

    fn rational_context() -> Context {
        Context::root().with_declaration(Declaration {
            name: "x".to_owned(),
            type_id: TypeId::new("core.rational.rational").unwrap(),
        })
    }

    fn elaborated_expression(source: &str) -> SemanticTerm {
        let ParseOutcome::Parsed(expression) = Parser::parse_expression(source) else {
            panic!("expected parse success");
        };
        let ElaborationOutcome::Elaborated(term) =
            Elaborator::elaborate_expression(&expression, &rational_context())
        else {
            panic!("expected elaboration success");
        };
        term
    }

    #[test]
    fn normalizes_quadratic_polynomial() {
        let term = elaborated_expression("(x + 1)(x - 1)");

        let MathematicalOutcome::Proven(result) = PolynomialNormalizer::normalize(&term, "x")
        else {
            panic!("expected proven normalization");
        };

        assert_eq!(
            result
                .value
                .normal_form
                .coefficients
                .get(&2)
                .unwrap()
                .to_string(),
            "1"
        );
        assert_eq!(
            result
                .value
                .normal_form
                .coefficients
                .get(&0)
                .unwrap()
                .to_string(),
            "-1"
        );
    }

    #[test]
    fn normalizes_power_expression() {
        let term = elaborated_expression("x^3 + x^3");

        let MathematicalOutcome::Proven(result) = PolynomialNormalizer::normalize(&term, "x")
        else {
            panic!("expected proven normalization");
        };

        assert_eq!(
            result
                .value
                .normal_form
                .coefficients
                .get(&3)
                .unwrap()
                .to_string(),
            "2"
        );
    }

    #[test]
    fn rejects_negative_exponents_as_unknown() {
        let term = elaborated_expression("x^-1");

        let MathematicalOutcome::Unknown(unknown) = PolynomialNormalizer::normalize(&term, "x")
        else {
            panic!("expected unknown normalization");
        };

        assert_eq!(unknown.reason, UnknownReason::UnsupportedDomain);
    }

    #[test]
    fn differentiates_polynomial_expression() {
        let term = elaborated_expression("3x^4 - 2x + 7");
        let MathematicalOutcome::Proven(result) = PolynomialNormalizer::normalize(&term, "x")
        else {
            panic!("expected proven normalization");
        };

        let derivative = result.value.normal_form.derivative();

        assert_eq!(derivative.coefficients.get(&3).unwrap().to_string(), "12");
        assert_eq!(derivative.coefficients.get(&0).unwrap().to_string(), "-2");
        assert!(!derivative.coefficients.contains_key(&6));
    }

    #[test]
    fn integrates_polynomial_expression() {
        let term = elaborated_expression("x^3");
        let MathematicalOutcome::Proven(result) = PolynomialNormalizer::normalize(&term, "x")
        else {
            panic!("expected proven normalization");
        };

        let antiderivative = result.value.normal_form.antiderivative();

        assert_eq!(
            antiderivative.coefficients.get(&4).unwrap().to_string(),
            "1/4"
        );
    }
}
