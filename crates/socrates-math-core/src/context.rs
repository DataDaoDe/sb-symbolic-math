use crate::ids::TypeId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ContextId(String);

impl ContextId {
    pub fn root() -> Self {
        Self("root".to_owned())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Declaration {
    pub name: String,
    pub type_id: TypeId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Context {
    pub id: ContextId,
    pub parent: Option<ContextId>,
    pub declarations: Vec<Declaration>,
    pub hypotheses: Vec<String>,
}

impl Context {
    pub fn root() -> Self {
        Self {
            id: ContextId::root(),
            parent: None,
            declarations: Vec::new(),
            hypotheses: Vec::new(),
        }
    }

    pub fn with_declaration(&self, declaration: Declaration) -> Self {
        let mut declarations = self.declarations.clone();
        declarations.push(declaration);

        Self {
            id: ContextId(format!("ctx.{}", declarations.len())),
            parent: Some(self.id.clone()),
            declarations,
            hypotheses: self.hypotheses.clone(),
        }
    }
}
