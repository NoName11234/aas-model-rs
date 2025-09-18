use serde::{Deserialize, Serialize};

use crate::structs::annotated_relationship_element::AnnotatedRelationshipElement;
use crate::structs::asset_administration_shell::AssetAdministrationShell;
use crate::structs::basic_event_element::BasicEventElement;
use crate::structs::blob::Blob;
use crate::structs::capability::Capability;
use crate::structs::concept_description::ConceptDescription;
use crate::structs::entity::Entity;
use crate::structs::extension::Extension;
use crate::structs::file::File;
use crate::structs::multi_language_property::MultiLanguageProperty;
use crate::structs::operation::Operation;
use crate::structs::property::Property;
use crate::structs::range::Range;
use crate::structs::reference_element::ReferenceElement;
use crate::structs::relationship_element::RelationshipElement;
use crate::structs::submodel::Submodel;
use crate::structs::submodel_element_collection::SubmodelElementCollection;
use crate::structs::submodel_element_list::SubmodelElementList;
use crate::traits::has_extensions::THasExtensions;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "modelType")]
pub enum HasExtensions {
    BasicEventElement(BasicEventElement),
    Capability(Capability),
    SubmodelElementCollection(SubmodelElementCollection),
    RelationshipElement(RelationshipElement),
    ReferenceElement(ReferenceElement),
    Property(Property),
    MultiLanguageProperty(MultiLanguageProperty),
    AnnotatedRelationshipElement(AnnotatedRelationshipElement),
    AssetAdministrationShell(AssetAdministrationShell),
    Entity(Entity),
    Operation(Operation),
    Range(Range),
    Blob(Blob),
    File(File),
    SubmodelElementList(SubmodelElementList),
    Submodel(Submodel),
    ConceptDescription(ConceptDescription)
}