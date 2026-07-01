use serde::{Deserialize, Serialize};
use socrates_math_core::{ExactRational, Judgment, RuleId};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ProofNodeId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProofNode {
    pub id: ProofNodeId,
    pub rule: ProofRule,
    pub conclusion: Judgment,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProofRule {
    EqualityReflexive,
    EqualitySymmetric {
        premise: Judgment,
    },
    EqualityTransitive {
        left_premise: Judgment,
        right_premise: Judgment,
    },
    RationalArithmetic {
        operation: RationalOperation,
        left: ExactRational,
        right: ExactRational,
    },
}

impl ProofRule {
    pub fn id(&self) -> RuleId {
        let id = match self {
            Self::EqualityReflexive => "logic.equal.reflexive",
            Self::EqualitySymmetric { .. } => "logic.equal.symmetric",
            Self::EqualityTransitive { .. } => "logic.equal.transitive",
            Self::RationalArithmetic { operation, .. } => match operation {
                RationalOperation::Add => "core.rational.add.exact",
                RationalOperation::Subtract => "core.rational.sub.exact",
                RationalOperation::Multiply => "core.rational.mul.exact",
                RationalOperation::Divide => "core.rational.div.exact",
            },
        };

        RuleId::new(id).expect("static rule id is valid")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RationalOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationError {
    ConclusionDoesNotMatchRule { rule_id: RuleId },
    DivisionByZero,
    IncompatibleTransitivePremises,
    UnsupportedRelation,
}
