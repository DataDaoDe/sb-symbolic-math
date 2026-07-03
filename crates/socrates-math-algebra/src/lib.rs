pub mod linear;
pub mod polynomial;

pub use linear::{LinearExpression, LinearNormalization, LinearNormalizer};
pub use polynomial::{PolynomialExpression, PolynomialNormalization, PolynomialNormalizer};
