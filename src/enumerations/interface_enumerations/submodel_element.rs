use serde::{Deserialize, Serialize};

use crate::structs::annotated_relationship_element::AnnotatedRelationshipElement;
use crate::structs::basic_event_element::BasicEventElement;
use crate::structs::blob::Blob;
use crate::structs::capability::Capability;
use crate::structs::entity::Entity;
use crate::structs::file::File;
use crate::structs::multi_language_property::MultiLanguageProperty;
use crate::structs::operation::Operation;
use crate::structs::property::Property;
use crate::structs::range::Range;
use crate::structs::reference_element::ReferenceElement;
use crate::structs::relationship_element::RelationshipElement;
use crate::structs::submodel_element_collection::SubmodelElementCollection;
use crate::structs::submodel_element_list::SubmodelElementList;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "modelType")]
pub enum SubmodelElement {
    RelationshipElement(RelationshipElement),
    AnnotatedRelationshipElement(AnnotatedRelationshipElement),
    Property(Property),
    MultiLanguageProperty(MultiLanguageProperty),
    Range(Range),
    Blob(Blob),
    File(File),
    ReferenceElement(ReferenceElement),
    Capability(Capability),
    SubmodelElementList(SubmodelElementList),
    SubmodelElementCollection(SubmodelElementCollection),
    Entity(Entity),
    BasicEventElement(BasicEventElement),
    Operation(Operation)
}