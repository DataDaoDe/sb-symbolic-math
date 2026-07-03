use serde::{Deserialize, Serialize};
use socrates_math_core::{
    ExactRational, MathematicalOutcome, SemanticTerm, Unknown, UnknownReason, Verified,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LinearExpression {
    pub variable: String,
    pub coefficient: ExactRational,
    pub constant: ExactRational,
}

impl LinearExpression {
    pub fn constant(variable: impl Into<String>, constant: ExactRational) -> Self {
        Self {
            variable: variable.into(),
            coefficient: ExactRational::integer(0),
            constant,
        }
    }

    pub fn variable(variable: impl Into<String>) -> Self {
        Self {
            variable: variable.into(),
            coefficient: ExactRational::integer(1),
            constant: ExactRational::integer(0),
        }
    }

    pub fn add(&self, rhs: &Self) -> Option<Self> {
        self.same_variable(rhs).then(|| Self {
            variable: self.variable.clone(),
            coefficient: self.coefficient.add(&rhs.coefficient),
            constant: self.constant.add(&rhs.constant),
        })
    }

    pub fn sub(&self, rhs: &Self) -> Option<Self> {
        self.same_variable(rhs).then(|| Self {
            variable: self.variable.clone(),
            coefficient: self.coefficient.sub(&rhs.coefficient),
            constant: self.constant.sub(&rhs.constant),
        })
    }

    pub fn neg(&self) -> Self {
        Self {
            variable: self.variable.clone(),
            coefficient: self.coefficient.neg(),
            constant: self.constant.neg(),
        }
    }

    pub fn scale(&self, scalar: &ExactRational) -> Self {
        Self {
            variable: self.variable.clone(),
            coefficient: self.coefficient.mul(scalar),
            constant: self.constant.mul(scalar),
        }
    }

    pub fn as_constant(&self) -> Option<&ExactRational> {
        self.coefficient.is_zero().then_some(&self.constant)
    }

    fn same_variable(&self, rhs: &Self) -> bool {
        self.variable == rhs.variable
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LinearNormalization {
    pub source: SemanticTerm,
    pub normal_form: LinearExpression,
    pub relation: String,
    pub theory: String,
    pub completeness_domain: String,
}

pub struct LinearNormalizer;

impl LinearNormalizer {
    pub fn normalize(
        term: &SemanticTerm,
        variable: &str,
    ) -> MathematicalOutcome<LinearNormalization> {
        match normalize_term(term, variable) {
            Ok(normal_form) => MathematicalOutcome::Proven(Verified {
                value: LinearNormalization {
                    source: term.clone(),
                    normal_form,
                    relation: "logic.equal".to_owned(),
                    theory: "algebra.linear-expression@0.1".to_owned(),
                    completeness_domain: "rational expressions equivalent to a*x + b".to_owned(),
                },
                evidence_id: Some("algebra.linear-expression.normalize.rational".to_owned()),
            }),
            Err(reason) => MathematicalOutcome::Unknown(Unknown { reason }),
        }
    }
}

fn normalize_term(term: &SemanticTerm, variable: &str) -> Result<LinearExpression, UnknownReason> {
    match term {
        SemanticTerm::RationalLiteral(value) => {
            Ok(LinearExpression::constant(variable, value.clone()))
        }
        SemanticTerm::LocalVariable { name, .. } if name == variable => {
            Ok(LinearExpression::variable(variable))
        }
        SemanticTerm::LocalVariable { .. } => Err(UnknownReason::UnsupportedDomain),
        SemanticTerm::Apply { symbol, args, .. } => match symbol.as_str() {
            "core.rational.add" => normalize_binary(args, variable, LinearExpression::add),
            "core.rational.sub" => normalize_binary(args, variable, LinearExpression::sub),
            "core.rational.neg" => {
                let [operand] = args.as_slice() else {
                    return Err(UnknownReason::UnsupportedDomain);
                };
                Ok(normalize_term(operand, variable)?.neg())
            }
            "core.rational.mul" => normalize_multiplication(args, variable),
            _ => Err(UnknownReason::UnsupportedDomain),
        },
    }
}

fn normalize_binary(
    args: &[SemanticTerm],
    variable: &str,
    operation: fn(&LinearExpression, &LinearExpression) -> Option<LinearExpression>,
) -> Result<LinearExpression, UnknownReason> {
    let [left, right] = args else {
        return Err(UnknownReason::UnsupportedDomain);
    };

    operation(
        &normalize_term(left, variable)?,
        &normalize_term(right, variable)?,
    )
    .ok_or(UnknownReason::UnsupportedDomain)
}

fn normalize_multiplication(
    args: &[SemanticTerm],
    variable: &str,
) -> Result<LinearExpression, UnknownReason> {
    let [left, right] = args else {
        return Err(UnknownReason::UnsupportedDomain);
    };

    let left = normalize_term(left, variable)?;
    let right = normalize_term(right, variable)?;

    if let Some(scalar) = left.as_constant() {
        return Ok(right.scale(scalar));
    }

    if let Some(scalar) = right.as_constant() {
        return Ok(left.scale(scalar));
    }

    Err(UnknownReason::UnsupportedDomain)
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
    fn normalizes_instructional_linear_expression() {
        let term = elaborated_expression("3(x - 2) + 4");

        let MathematicalOutcome::Proven(result) = LinearNormalizer::normalize(&term, "x") else {
            panic!("expected proven normalization");
        };

        assert_eq!(result.value.normal_form.coefficient.to_string(), "3");
        assert_eq!(result.value.normal_form.constant.to_string(), "-2");
        assert_eq!(result.value.relation, "logic.equal");
        assert_eq!(
            result.value.theory,
            "algebra.linear-expression@0.1".to_owned()
        );
    }

    #[test]
    fn normalizes_rational_coefficients() {
        let term = elaborated_expression("\\frac{1}{2}x + \\frac{1}{3}");

        let MathematicalOutcome::Proven(result) = LinearNormalizer::normalize(&term, "x") else {
            panic!("expected proven normalization");
        };

        assert_eq!(result.value.normal_form.coefficient.to_string(), "1/2");
        assert_eq!(result.value.normal_form.constant.to_string(), "1/3");
    }

    #[test]
    fn rejects_nonlinear_expression_as_unknown() {
        let term = elaborated_expression("x x + 1");

        let MathematicalOutcome::Unknown(unknown) = LinearNormalizer::normalize(&term, "x") else {
            panic!("expected unknown result");
        };

        assert_eq!(unknown.reason, UnknownReason::UnsupportedDomain);
    }

    #[test]
    fn normalizes_negative_variable_expression() {
        let term = elaborated_expression("-x + 2");

        let MathematicalOutcome::Proven(result) = LinearNormalizer::normalize(&term, "x") else {
            panic!("expected proven normalization");
        };

        assert_eq!(result.value.normal_form.coefficient.to_string(), "-1");
        assert_eq!(result.value.normal_form.constant.to_string(), "2");
    }
}
