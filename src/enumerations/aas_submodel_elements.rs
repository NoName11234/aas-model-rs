use serde::{Deserialize, Serialize};

///Enumeration of submodel element types including abstract submodel element types.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum AasSubmodelElements {
    SubmodelElement,
    AnnotatedRelationshipElement,
    Entity,
    SubmodelElementCollection,
    SubmodelElementList,
    BasicEventElement,
    Blob,
    Capability,
    DataElement,
    EventElement,
    File,
    MultiLanguageProperty,
    Operation,
    Property,
    Range,
    ReferenceElement,
    RelationshipElement
}