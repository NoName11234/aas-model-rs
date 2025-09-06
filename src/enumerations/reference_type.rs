use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum ReferenceType {
    ExternalReference,
    ModelReference
}