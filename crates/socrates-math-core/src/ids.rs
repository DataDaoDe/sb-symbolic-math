use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StableId(String);

impl StableId {
    pub fn new(value: impl Into<String>) -> Result<Self, StableIdError> {
        let value = value.into();

        if value.is_empty() {
            return Err(StableIdError::Empty);
        }

        if value.chars().all(|ch| {
            ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '.' | '-' | '_')
        }) {
            Ok(Self(value))
        } else {
            Err(StableIdError::InvalidCharacter)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StableIdError {
    Empty,
    InvalidCharacter,
}

macro_rules! id_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
        pub struct $name(StableId);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, StableIdError> {
                StableId::new(value).map(Self)
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl From<$name> for StableId {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

id_newtype!(TheoryId);
id_newtype!(SymbolId);
id_newtype!(RuleId);
id_newtype!(RelationId);
id_newtype!(TypeId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_theory_qualified_ids() {
        let id = StableId::new("core.rational.add").unwrap();
        assert_eq!(id.as_str(), "core.rational.add");
    }

    #[test]
    fn rejects_display_notation_as_id() {
        assert_eq!(StableId::new("+"), Err(StableIdError::InvalidCharacter));
    }
}
