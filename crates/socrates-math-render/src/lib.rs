use socrates_math_algebra::{LinearExpression, PolynomialExpression};
use socrates_math_core::ExactRational;
use socrates_math_solve::SolutionSet;

pub struct LatexRenderer;

impl LatexRenderer {
    pub fn exact_rational(value: &ExactRational) -> String {
        if value.is_integer() {
            value.numerator().to_string()
        } else {
            format!("\\frac{{{}}}{{{}}}", value.numerator(), value.denominator())
        }
    }

    pub fn solution_set(variable: &str, solution_set: &SolutionSet) -> String {
        match solution_set {
            SolutionSet::Empty => "\\varnothing".to_owned(),
            SolutionSet::Unique(value) => {
                format!("{variable} = {}", Self::exact_rational(value))
            }
            SolutionSet::AllRationals => format!("{variable} \\in \\mathbb{{Q}}"),
        }
    }

    pub fn linear_expression(expression: &LinearExpression) -> String {
        let coefficient = &expression.coefficient;
        let constant = &expression.constant;
        let variable = &expression.variable;

        if coefficient.is_zero() {
            return Self::exact_rational(constant);
        }

        let variable_term = if coefficient == &ExactRational::integer(1) {
            variable.clone()
        } else if coefficient == &ExactRational::integer(-1) {
            format!("-{variable}")
        } else {
            format!("{}{}", Self::exact_rational(coefficient), variable)
        };

        if constant.is_zero() {
            variable_term
        } else if constant.numerator().to_string().starts_with('-') {
            format!(
                "{variable_term} - {}",
                Self::exact_rational(&constant.neg())
            )
        } else {
            format!("{variable_term} + {}", Self::exact_rational(constant))
        }
    }

    pub fn polynomial_expression(expression: &PolynomialExpression) -> String {
        if expression.coefficients.is_empty() {
            return "0".to_owned();
        }

        let mut rendered = String::new();

        for (degree, coefficient) in expression.coefficients.iter().rev() {
            let is_negative = coefficient.numerator().to_string().starts_with('-');
            let magnitude = if is_negative {
                coefficient.neg()
            } else {
                coefficient.clone()
            };
            let term = render_polynomial_term(&expression.variable, *degree, &magnitude);

            if rendered.is_empty() {
                if is_negative {
                    rendered.push('-');
                }
                rendered.push_str(&term);
            } else if is_negative {
                rendered.push_str(" - ");
                rendered.push_str(&term);
            } else {
                rendered.push_str(" + ");
                rendered.push_str(&term);
            }
        }

        rendered
    }
}

fn render_polynomial_term(variable: &str, degree: u32, coefficient: &ExactRational) -> String {
    if degree == 0 {
        return LatexRenderer::exact_rational(coefficient);
    }

    let variable_factor = if degree == 1 {
        variable.to_owned()
    } else {
        format!("{variable}^{{{degree}}}")
    };

    if coefficient == &ExactRational::integer(1) {
        variable_factor
    } else {
        format!(
            "{}{variable_factor}",
            LatexRenderer::exact_rational(coefficient)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_integer_rational_as_integer() {
        assert_eq!(
            LatexRenderer::exact_rational(&ExactRational::integer(11)),
            "11"
        );
    }

    #[test]
    fn renders_non_integer_rational_as_frac() {
        let value = ExactRational::parse_fraction("-2", "3").unwrap();

        assert_eq!(LatexRenderer::exact_rational(&value), "\\frac{-2}{3}");
    }

    #[test]
    fn renders_solution_sets() {
        assert_eq!(
            LatexRenderer::solution_set("x", &SolutionSet::Unique(ExactRational::integer(11))),
            "x = 11"
        );
        assert_eq!(
            LatexRenderer::solution_set("x", &SolutionSet::Empty),
            "\\varnothing"
        );
        assert_eq!(
            LatexRenderer::solution_set("x", &SolutionSet::AllRationals),
            "x \\in \\mathbb{Q}"
        );
    }

    #[test]
    fn renders_linear_expressions() {
        assert_eq!(
            LatexRenderer::linear_expression(&LinearExpression {
                variable: "x".to_owned(),
                coefficient: ExactRational::integer(3),
                constant: ExactRational::integer(-2),
            }),
            "3x - 2"
        );
    }

    #[test]
    fn renders_polynomial_expressions() {
        let mut coefficients = std::collections::BTreeMap::new();
        coefficients.insert(3, ExactRational::integer(2));
        coefficients.insert(1, ExactRational::integer(-4));
        coefficients.insert(0, ExactRational::integer(1));

        assert_eq!(
            LatexRenderer::polynomial_expression(&PolynomialExpression {
                variable: "x".to_owned(),
                coefficients,
            }),
            "2x^{3} - 4x + 1"
        );
    }
}
