use serde::{Deserialize, Serialize};

use crate::structs::annotated_relationship_element::AnnotatedRelationshipElement;
use crate::structs::reference::Reference;
use crate::traits::relationship_element::TRelationshipElement;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "modelType")]
pub enum RelationshipElement {
    AnnotatedRelationshipElement(AnnotatedRelationshipElement)
}