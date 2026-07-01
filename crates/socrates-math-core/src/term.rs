use crate::exact::ExactRational;
use crate::ids::{RelationId, SymbolId, TypeId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TypeRef {
    pub id: TypeId,
}

impl TypeRef {
    pub fn rational() -> Self {
        Self {
            id: TypeId::new("core.rational.rational").expect("static type id is valid"),
        }
    }

    pub fn proposition() -> Self {
        Self {
            id: TypeId::new("core.logic.proposition").expect("static type id is valid"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SemanticTerm {
    RationalLiteral(ExactRational),
    LocalVariable {
        name: String,
        type_ref: TypeRef,
    },
    Apply {
        symbol: SymbolId,
        args: Vec<SemanticTerm>,
        type_ref: TypeRef,
    },
}

impl SemanticTerm {
    pub fn rational(value: ExactRational) -> Self {
        Self::RationalLiteral(value)
    }

    pub fn variable(name: impl Into<String>, type_ref: TypeRef) -> Self {
        Self::LocalVariable {
            name: name.into(),
            type_ref,
        }
    }

    pub fn apply(symbol: SymbolId, args: Vec<Self>, type_ref: TypeRef) -> Self {
        Self::Apply {
            symbol,
            args,
            type_ref,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Relation {
    pub id: RelationId,
}

impl Relation {
    pub fn equality() -> Self {
        Self {
            id: RelationId::new("logic.equal").expect("static relation id is valid"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Judgment {
    pub left: SemanticTerm,
    pub relation: Relation,
    pub right: SemanticTerm,
}
