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

impl HasExtensions {
    pub fn get_extensions(&self) -> &Vec<Extension> {
        match self {
            HasExtensions::BasicEventElement(elem) => elem.get_extensions(),
            HasExtensions::Capability(elem) => elem.get_extensions(),
            HasExtensions::SubmodelElementCollection(elem) => elem.get_extensions(),
            HasExtensions::RelationshipElement(elem) => elem.get_extensions(),
            HasExtensions::ReferenceElement(elem) => elem.get_extensions(),
            HasExtensions::Property(elem) => elem.get_extensions(),
            HasExtensions::MultiLanguageProperty(elem) => elem.get_extensions(),
            HasExtensions::AnnotatedRelationshipElement(elem) => elem.get_extensions(),
            HasExtensions::AssetAdministrationShell(elem) => elem.get_extensions(),
            HasExtensions::Entity(elem) => elem.get_extensions(),
            HasExtensions::Operation(elem) => elem.get_extensions(),
            HasExtensions::Range(elem) => elem.get_extensions(),
            HasExtensions::Blob(elem) => elem.get_extensions(),
            HasExtensions::File(elem) => elem.get_extensions(),
            HasExtensions::SubmodelElementList(elem) => elem.get_extensions(),
            HasExtensions::Submodel(elem) => elem.get_extensions(),
            HasExtensions::ConceptDescription(elem) => elem.get_extensions()
        }
    }

    pub fn set_extensions(&mut self, extensions: Vec<Extension>) {
        match self {
            HasExtensions::BasicEventElement(elem) => elem.set_extensions(extensions),
            HasExtensions::Capability(elem) => elem.set_extensions(extensions),
            HasExtensions::SubmodelElementCollection(elem) => elem.set_extensions(extensions),
            HasExtensions::RelationshipElement(elem) => elem.set_extensions(extensions),
            HasExtensions::ReferenceElement(elem) => elem.set_extensions(extensions),
            HasExtensions::Property(elem) => elem.set_extensions(extensions),
            HasExtensions::MultiLanguageProperty(elem) => elem.set_extensions(extensions),
            HasExtensions::AnnotatedRelationshipElement(elem) => elem.set_extensions(extensions),
            HasExtensions::AssetAdministrationShell(elem) => elem.set_extensions(extensions),
            HasExtensions::Entity(elem) => elem.set_extensions(extensions),
            HasExtensions::Operation(elem) => elem.set_extensions(extensions),
            HasExtensions::Range(elem) => elem.set_extensions(extensions),
            HasExtensions::Blob(elem) => elem.set_extensions(extensions),
            HasExtensions::File(elem) => elem.set_extensions(extensions),
            HasExtensions::SubmodelElementList(elem) => elem.set_extensions(extensions),
            HasExtensions::Submodel(elem) => elem.set_extensions(extensions),
            HasExtensions::ConceptDescription(elem) => elem.set_extensions(extensions)
        }
    }

    pub fn add_extension(&mut self, extension: Extension) {
        match self {
            HasExtensions::BasicEventElement(elem) => elem.add_extension(extension),
            HasExtensions::Capability(elem) => elem.add_extension(extension),
            HasExtensions::SubmodelElementCollection(elem) => elem.add_extension(extension),
            HasExtensions::RelationshipElement(elem) => elem.add_extension(extension),
            HasExtensions::ReferenceElement(elem) => elem.add_extension(extension),
            HasExtensions::Property(elem) => elem.add_extension(extension),
            HasExtensions::MultiLanguageProperty(elem) => elem.add_extension(extension),
            HasExtensions::AnnotatedRelationshipElement(elem) => elem.add_extension(extension),
            HasExtensions::AssetAdministrationShell(elem) => elem.add_extension(extension),
            HasExtensions::Entity(elem) => elem.add_extension(extension),
            HasExtensions::Operation(elem) => elem.add_extension(extension),
            HasExtensions::Range(elem) => elem.add_extension(extension),
            HasExtensions::Blob(elem) => elem.add_extension(extension),
            HasExtensions::File(elem) => elem.add_extension(extension),
            HasExtensions::SubmodelElementList(elem) => elem.add_extension(extension),
            HasExtensions::Submodel(elem) => elem.add_extension(extension),
            HasExtensions::ConceptDescription(elem) => elem.add_extension(extension)
        }
    }

    pub fn remove_extensions(&mut self, index: usize) -> Extension {
        match self {
            HasExtensions::BasicEventElement(elem) => elem.remove_extension(index),
            HasExtensions::Capability(elem) => elem.remove_extension(index),
            HasExtensions::SubmodelElementCollection(elem) => elem.remove_extension(index),
            HasExtensions::RelationshipElement(elem) => elem.remove_extension(index),
            HasExtensions::ReferenceElement(elem) => elem.remove_extension(index),
            HasExtensions::Property(elem) => elem.remove_extension(index),
            HasExtensions::MultiLanguageProperty(elem) => elem.remove_extension(index),
            HasExtensions::AnnotatedRelationshipElement(elem) => elem.remove_extension(index),
            HasExtensions::AssetAdministrationShell(elem) => elem.remove_extension(index),
            HasExtensions::Entity(elem) => elem.remove_extension(index),
            HasExtensions::Operation(elem) => elem.remove_extension(index),
            HasExtensions::Range(elem) => elem.remove_extension(index),
            HasExtensions::Blob(elem) => elem.remove_extension(index),
            HasExtensions::File(elem) => elem.remove_extension(index),
            HasExtensions::SubmodelElementList(elem) => elem.remove_extension(index),
            HasExtensions::Submodel(elem) => elem.remove_extension(index),
            HasExtensions::ConceptDescription(elem) => elem.remove_extension(index)
        }
    }
}