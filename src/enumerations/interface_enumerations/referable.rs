use serde::{Deserialize, Serialize};

use crate::enumerations::interface_enumerations::data_element::DataElement;
use crate::enumerations::interface_enumerations::submodel_element::SubmodelElement;
use crate::structs::annotated_relationship_element::AnnotatedRelationshipElement;
use crate::structs::asset_administration_shell::AssetAdministrationShell;
use crate::structs::basic_event_element::BasicEventElement;
use crate::structs::blob::Blob;
use crate::structs::capability::Capability;
use crate::structs::concept_description::ConceptDescription;
use crate::structs::entity::Entity;
use crate::structs::file::File;
use crate::structs::multi_language_name_type::MultiLanguageNameType;
use crate::structs::multi_language_property::MultiLanguageProperty;
use crate::structs::multi_language_text_type::MultiLanguageTextType;
use crate::structs::operation::Operation;
use crate::structs::property::Property;
use crate::structs::range::Range;
use crate::structs::reference_element::ReferenceElement;
use crate::structs::relationship_element::RelationshipElement;
use crate::structs::submodel::Submodel;
use crate::structs::submodel_element_collection::SubmodelElementCollection;
use crate::structs::submodel_element_list::SubmodelElementList;
use crate::traits::referable::TReferable;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "modelType")]
pub enum Referable {
    AnnotatedRelationshipElement(AnnotatedRelationshipElement),
    AssetAdministrationShell(AssetAdministrationShell),
    BasicEventElement(Box<BasicEventElement>),
    Blob(Blob),
    Capability(Capability),
    ConceptDescription(ConceptDescription),
    DataElement(DataElement),
    Entity(Entity),
    File(File),
    MultiLanguageProperty(MultiLanguageProperty),
    Operation(Operation),
    Property(Property),
    Range(Range),
    Referable(Box<Referable>),
    ReferenceElement(ReferenceElement),
    RelationshipElement(RelationshipElement),
    Submodel(Submodel),
    SubmodelElement(Box<SubmodelElement>),
    SubmodelElementCollection(SubmodelElementCollection),
    SubmodelElementList(SubmodelElementList)
}