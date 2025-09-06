use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum QualifierKind {
    #[serde(rename = "Value")]
    ValueQualifier,
    #[serde(rename = "Concept")]
    ConceptQualifier,
    #[serde(rename = "Template")]
    TemplateQualifier
}