use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum KeyType {
    AnnotatedRelationshipElement,
    AssetAdministrationShell,
    BasicEventElement,
    Blob,
    Capability,
    ConceptDescription,
    DataElement,
    Entity,
    EventElement,
    File,
    FragmentReference,
    GlobalReference,
    Identifiable,
    MultiLanguageProperty,
    Operation,
    Property,
    Range,
    Referable,
    ReferenceElement,
    RelationshipElement,
    Submodel,
    SubmodelElement,
    SubmodelElementCollection,
    SubmodelElementList
}