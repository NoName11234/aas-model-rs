use serde::{Deserialize, Serialize};

use crate::structs::annotated_relationship_element::AnnotatedRelationshipElement;
use crate::structs::basic_event_element::BasicEventElement;
use crate::structs::blob::Blob;
use crate::structs::capability::Capability;
use crate::structs::entity::Entity;
use crate::structs::extension::Extension;
use crate::structs::file::File;
use crate::structs::multi_language_property::MultiLanguageProperty;
use crate::structs::operation::Operation;
use crate::structs::property::Property;
use crate::structs::qualifier::Qualifier;
use crate::structs::range::Range;
use crate::structs::reference::Reference;
use crate::structs::reference_element::ReferenceElement;
use crate::structs::relationship_element::RelationshipElement;
use crate::structs::specific_asset_id::SpecificAssetId;
use crate::structs::submodel::Submodel;
use crate::structs::submodel_element_collection::SubmodelElementCollection;
use crate::structs::submodel_element_list::SubmodelElementList;
use crate::traits::has_semantics::THasSemantics;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "modelType")]
pub enum HasSemantics {
    Submodel(Submodel),
    Qualifier(Qualifier),
    Extension(Extension),
    SubmodelElementCollection(SubmodelElementCollection),
    RelationshipElement(RelationshipElement),
    ReferenceElement(ReferenceElement),
    Property(Property),
    MultiLanguageProperty(MultiLanguageProperty),
    AnnotatedRelationshipElement(AnnotatedRelationshipElement),
    Entity(Entity),
    Operation(Operation),
    Range(Range),
    Blob(Blob),
    File(File),
    SubmodelElementList(SubmodelElementList),
    SpecificAssetId(SpecificAssetId),
    Capability(Capability),
    BasicEventElement(BasicEventElement)
}

impl HasSemantics {
    pub fn set_semantic_id(&mut self, semantic_id: Reference) {
        match self {
            HasSemantics::Submodel(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::Qualifier(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::Extension(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::SubmodelElementCollection(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::RelationshipElement(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::ReferenceElement(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::Property(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::MultiLanguageProperty(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::AnnotatedRelationshipElement(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::Entity(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::Operation(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::Range(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::Blob(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::File(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::SubmodelElementList(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::SpecificAssetId(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::Capability(elem) => elem.set_semantic_id(semantic_id),
            HasSemantics::BasicEventElement(elem) => elem.set_semantic_id(semantic_id)
        }
    }

    pub fn get_semantic_id(&self) -> Option<&Reference> {
        match self {
            HasSemantics::Submodel(elem) => elem.get_semantic_id(),
            HasSemantics::Qualifier(elem) => elem.get_semantic_id(),
            HasSemantics::Extension(elem) => elem.get_semantic_id(),
            HasSemantics::SubmodelElementCollection(elem) => elem.get_semantic_id(),
            HasSemantics::RelationshipElement(elem) => elem.get_semantic_id(),
            HasSemantics::ReferenceElement(elem) => elem.get_semantic_id(),
            HasSemantics::Property(elem) => elem.get_semantic_id(),
            HasSemantics::MultiLanguageProperty(elem) => elem.get_semantic_id(),
            HasSemantics::AnnotatedRelationshipElement(elem) => elem.get_semantic_id(),
            HasSemantics::Entity(elem) => elem.get_semantic_id(),
            HasSemantics::Operation(elem) => elem.get_semantic_id(),
            HasSemantics::Range(elem) => elem.get_semantic_id(),
            HasSemantics::Blob(elem) => elem.get_semantic_id(),
            HasSemantics::File(elem) => elem.get_semantic_id(),
            HasSemantics::SubmodelElementList(elem) => elem.get_semantic_id(),
            HasSemantics::SpecificAssetId(elem) => elem.get_semantic_id(),
            HasSemantics::Capability(elem) => elem.get_semantic_id(),
            HasSemantics::BasicEventElement(elem) => elem.get_semantic_id()
        }
    }

    pub fn set_supplemental_semantic_ids(&mut self, supplemental_semantic_ids: Vec<Reference>) {
        match self {
            HasSemantics::Submodel(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::Qualifier(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::Extension(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::SubmodelElementCollection(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::RelationshipElement(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::ReferenceElement(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::Property(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::MultiLanguageProperty(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::AnnotatedRelationshipElement(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::Entity(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::Operation(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::Range(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::Blob(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::File(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::SubmodelElementList(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::SpecificAssetId(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::Capability(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids),
            HasSemantics::BasicEventElement(elem) => elem.set_supplemental_semantic_ids(supplemental_semantic_ids)
        }
    }

    pub fn get_supplemental_semantic_ids(&self) -> &Vec<Reference> {
        match self {
            HasSemantics::Submodel(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::Qualifier(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::Extension(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::SubmodelElementCollection(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::RelationshipElement(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::ReferenceElement(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::Property(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::MultiLanguageProperty(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::AnnotatedRelationshipElement(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::Entity(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::Operation(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::Range(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::Blob(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::File(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::SubmodelElementList(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::SpecificAssetId(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::Capability(elem) => elem.get_supplemental_semantic_ids(),
            HasSemantics::BasicEventElement(elem) => elem.get_supplemental_semantic_ids()
        }
    }

    pub fn add_supplemental_semantic_id(&mut self, supplemental_semantic_id: Reference) {
        match self {
            HasSemantics::Submodel(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::Qualifier(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::Extension(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::SubmodelElementCollection(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::RelationshipElement(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::ReferenceElement(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::Property(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::MultiLanguageProperty(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::AnnotatedRelationshipElement(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::Entity(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::Operation(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::Range(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::Blob(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::File(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::SubmodelElementList(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::SpecificAssetId(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::Capability(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id),
            HasSemantics::BasicEventElement(elem) => elem.add_supplemental_semantic_id(supplemental_semantic_id)
        }
    }

    pub fn remove_supplemental_semantic_id(&mut self, index: usize) -> Reference {
        match self {
            HasSemantics::Submodel(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::Qualifier(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::Extension(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::SubmodelElementCollection(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::RelationshipElement(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::ReferenceElement(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::Property(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::MultiLanguageProperty(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::AnnotatedRelationshipElement(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::Entity(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::Operation(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::Range(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::Blob(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::File(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::SubmodelElementList(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::SpecificAssetId(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::Capability(elem) => elem.remove_supplemental_semantic_id(index),
            HasSemantics::BasicEventElement(elem) => elem.remove_supplemental_semantic_id(index),
        }
    }
}