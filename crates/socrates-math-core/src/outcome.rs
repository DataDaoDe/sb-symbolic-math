use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MathematicalOutcome<T> {
    Proven(Verified<T>),
    Disproven(Disproven),
    Conditional(Conditional<T>),
    Unknown(Unknown),
    Undefined(Undefined),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Verified<T> {
    pub value: T,
    pub evidence_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Disproven {
    pub reason: String,
    pub evidence_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Conditional<T> {
    pub value: T,
    pub conditions: Vec<String>,
    pub evidence_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Unknown {
    pub reason: UnknownReason,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum UnknownReason {
    ExternalBackendUnavailable,
    IncompleteProcedure,
    InsufficientAssumptions,
    NoApplicableMethod,
    ResourceLimit,
    UnsupportedDomain,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Undefined {
    pub reason: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum VerificationStatus {
    Verified,
    Conditional,
    Unverified,
}
