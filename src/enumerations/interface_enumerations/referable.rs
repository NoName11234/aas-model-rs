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

impl Referable {
    pub fn set_category(&mut self, category: String) {
        match self {
            Referable::AnnotatedRelationshipElement(elem) => elem.set_category(category),
            Referable::AssetAdministrationShell(elem) => elem.set_category(category),
            Referable::BasicEventElement(elem) => elem.set_category(category),
            Referable::Blob(elem) => elem.set_category(category),
            Referable::Capability(elem) => elem.set_category(category),
            Referable::ConceptDescription(elem) => elem.set_category(category),
            Referable::DataElement(elem) => {
                match elem {
                    DataElement::Property(data_elem) => data_elem.set_category(category),
                    DataElement::MultiLanguageProperty(data_elem) => data_elem.set_category(category),
                    DataElement::Range(data_elem) => data_elem.set_category(category),
                    DataElement::Blob(data_elem) => data_elem.set_category(category),
                    DataElement::File(data_elem) => data_elem.set_category(category),
                    DataElement::ReferenceElement(data_elem) => data_elem.set_category(category)
                }
            },
            Referable::Entity(elem) => elem.set_category(category),
            Referable::File(elem) => elem.set_category(category),
            Referable::MultiLanguageProperty(elem) => elem.set_category(category),
            Referable::Operation(elem) => elem.set_category(category),
            Referable::Property(elem) => elem.set_category(category),
            Referable::Range(elem) => elem.set_category(category),
            Referable::Referable(elem) => elem.set_category(category),
            Referable::ReferenceElement(elem) => elem.set_category(category),
            Referable::RelationshipElement(elem) => elem.set_category(category),
            Referable::Submodel(elem) => elem.set_category(category),
            Referable::SubmodelElement(elem) => {
                match elem.as_mut() {
                    SubmodelElement::RelationshipElement(elem) => elem.set_category(category),
                    SubmodelElement::AnnotatedRelationshipElement(elem) => elem.set_category(category),
                    SubmodelElement::Property(elem) => elem.set_category(category),
                    SubmodelElement::MultiLanguageProperty(elem) => elem.set_category(category),
                    SubmodelElement::Range(elem) => elem.set_category(category),
                    SubmodelElement::Blob(elem) => elem.set_category(category),
                    SubmodelElement::File(elem) => elem.set_category(category),
                    SubmodelElement::ReferenceElement(elem) => elem.set_category(category),
                    SubmodelElement::Capability(elem) => elem.set_category(category),
                    SubmodelElement::SubmodelElementList(elem) => elem.set_category(category),
                    SubmodelElement::SubmodelElementCollection(elem) => elem.set_category(category),
                    SubmodelElement::Entity(elem) => elem.set_category(category),
                    SubmodelElement::BasicEventElement(elem) => elem.set_category(category),
                    SubmodelElement::Operation(elem) => elem.set_category(category)
                }
            },
            Referable::SubmodelElementCollection(elem) => elem.set_category(category),
            Referable::SubmodelElementList(elem) => elem.set_category(category)
        }
    }

    pub fn get_category(&self) -> Option<&String> {
        match self {
            Referable::AnnotatedRelationshipElement(elem) => elem.get_category(),
            Referable::AssetAdministrationShell(elem) => elem.get_category(),
            Referable::BasicEventElement(elem) => elem.get_category(),
            Referable::Blob(elem) => elem.get_category(),
            Referable::Capability(elem) => elem.get_category(),
            Referable::ConceptDescription(elem) => elem.get_category(),
            Referable::DataElement(elem) => {
                match elem {
                    DataElement::Property(data_elem) => data_elem.get_category(),
                    DataElement::MultiLanguageProperty(data_elem) => data_elem.get_category(),
                    DataElement::Range(data_elem) => data_elem.get_category(),
                    DataElement::Blob(data_elem) => data_elem.get_category(),
                    DataElement::File(data_elem) => data_elem.get_category(),
                    DataElement::ReferenceElement(data_elem) => data_elem.get_category()
                }
            },
            Referable::Entity(elem) => elem.get_category(),
            Referable::File(elem) => elem.get_category(),
            Referable::MultiLanguageProperty(elem) => elem.get_category(),
            Referable::Operation(elem) => elem.get_category(),
            Referable::Property(elem) => elem.get_category(),
            Referable::Range(elem) => elem.get_category(),
            Referable::Referable(elem) => elem.get_category(),
            Referable::ReferenceElement(elem) => elem.get_category(),
            Referable::RelationshipElement(elem) => elem.get_category(),
            Referable::Submodel(elem) => elem.get_category(),
            Referable::SubmodelElement(elem) => {
                match elem.as_ref() {
                    SubmodelElement::RelationshipElement(elem) => elem.get_category(),
                    SubmodelElement::AnnotatedRelationshipElement(elem) => elem.get_category(),
                    SubmodelElement::Property(elem) => elem.get_category(),
                    SubmodelElement::MultiLanguageProperty(elem) => elem.get_category(),
                    SubmodelElement::Range(elem) => elem.get_category(),
                    SubmodelElement::Blob(elem) => elem.get_category(),
                    SubmodelElement::File(elem) => elem.get_category(),
                    SubmodelElement::ReferenceElement(elem) => elem.get_category(),
                    SubmodelElement::Capability(elem) => elem.get_category(),
                    SubmodelElement::SubmodelElementList(elem) => elem.get_category(),
                    SubmodelElement::SubmodelElementCollection(elem) => elem.get_category(),
                    SubmodelElement::Entity(elem) => elem.get_category(),
                    SubmodelElement::BasicEventElement(elem) => elem.get_category(),
                    SubmodelElement::Operation(elem) => elem.get_category()
                }
            },
            Referable::SubmodelElementCollection(elem) => elem.get_category(),
            Referable::SubmodelElementList(elem) => elem.get_category()
        }
    }

    pub fn set_id_short(&mut self, id_short: String) {
        match self {
            Referable::AnnotatedRelationshipElement(elem) => elem.set_id_short(id_short),
            Referable::AssetAdministrationShell(elem) => elem.set_id_short(id_short),
            Referable::BasicEventElement(elem) => elem.set_id_short(id_short),
            Referable::Blob(elem) => elem.set_id_short(id_short),
            Referable::Capability(elem) => elem.set_id_short(id_short),
            Referable::ConceptDescription(elem) => elem.set_id_short(id_short),
            Referable::DataElement(elem) => {
                match elem {
                    DataElement::Property(data_elem) => data_elem.set_id_short(id_short),
                    DataElement::MultiLanguageProperty(data_elem) => data_elem.set_id_short(id_short),
                    DataElement::Range(data_elem) => data_elem.set_id_short(id_short),
                    DataElement::Blob(data_elem) => data_elem.set_id_short(id_short),
                    DataElement::File(data_elem) => data_elem.set_id_short(id_short),
                    DataElement::ReferenceElement(data_elem) => data_elem.set_id_short(id_short)
                }
            },
            Referable::Entity(elem) => elem.set_id_short(id_short),
            Referable::File(elem) => elem.set_id_short(id_short),
            Referable::MultiLanguageProperty(elem) => elem.set_id_short(id_short),
            Referable::Operation(elem) => elem.set_id_short(id_short),
            Referable::Property(elem) => elem.set_id_short(id_short),
            Referable::Range(elem) => elem.set_id_short(id_short),
            Referable::Referable(elem) => elem.set_id_short(id_short),
            Referable::ReferenceElement(elem) => elem.set_id_short(id_short),
            Referable::RelationshipElement(elem) => elem.set_id_short(id_short),
            Referable::Submodel(elem) => elem.set_id_short(id_short),
            Referable::SubmodelElement(elem) => {
                match elem.as_mut() {
                    SubmodelElement::RelationshipElement(elem) => elem.set_id_short(id_short),
                    SubmodelElement::AnnotatedRelationshipElement(elem) => elem.set_id_short(id_short),
                    SubmodelElement::Property(elem) => elem.set_id_short(id_short),
                    SubmodelElement::MultiLanguageProperty(elem) => elem.set_id_short(id_short),
                    SubmodelElement::Range(elem) => elem.set_id_short(id_short),
                    SubmodelElement::Blob(elem) => elem.set_id_short(id_short),
                    SubmodelElement::File(elem) => elem.set_id_short(id_short),
                    SubmodelElement::ReferenceElement(elem) => elem.set_id_short(id_short),
                    SubmodelElement::Capability(elem) => elem.set_id_short(id_short),
                    SubmodelElement::SubmodelElementList(elem) => elem.set_id_short(id_short),
                    SubmodelElement::SubmodelElementCollection(elem) => elem.set_id_short(id_short),
                    SubmodelElement::Entity(elem) => elem.set_id_short(id_short),
                    SubmodelElement::BasicEventElement(elem) => elem.set_id_short(id_short),
                    SubmodelElement::Operation(elem) => elem.set_id_short(id_short)
                }
            },
            Referable::SubmodelElementCollection(elem) => elem.set_id_short(id_short),
            Referable::SubmodelElementList(elem) => elem.set_id_short(id_short)
        }
    }

    pub fn get_id_short(&self) -> Option<&String> {
        match self {
            Referable::AnnotatedRelationshipElement(elem) => elem.get_id_short(),
            Referable::AssetAdministrationShell(elem) => elem.get_id_short(),
            Referable::BasicEventElement(elem) => elem.get_id_short(),
            Referable::Blob(elem) => elem.get_id_short(),
            Referable::Capability(elem) => elem.get_id_short(),
            Referable::ConceptDescription(elem) => elem.get_id_short(),
            Referable::DataElement(elem) => {
                match elem {
                    DataElement::Property(data_elem) => data_elem.get_id_short(),
                    DataElement::MultiLanguageProperty(data_elem) => data_elem.get_id_short(),
                    DataElement::Range(data_elem) => data_elem.get_id_short(),
                    DataElement::Blob(data_elem) => data_elem.get_id_short(),
                    DataElement::File(data_elem) => data_elem.get_id_short(),
                    DataElement::ReferenceElement(data_elem) => data_elem.get_id_short()
                }
            },
            Referable::Entity(elem) => elem.get_id_short(),
            Referable::File(elem) => elem.get_id_short(),
            Referable::MultiLanguageProperty(elem) => elem.get_id_short(),
            Referable::Operation(elem) => elem.get_id_short(),
            Referable::Property(elem) => elem.get_id_short(),
            Referable::Range(elem) => elem.get_id_short(),
            Referable::Referable(elem) => elem.get_id_short(),
            Referable::ReferenceElement(elem) => elem.get_id_short(),
            Referable::RelationshipElement(elem) => elem.get_id_short(),
            Referable::Submodel(elem) => elem.get_id_short(),
            Referable::SubmodelElement(elem) => {
                match elem.as_ref() {
                    SubmodelElement::RelationshipElement(elem) => elem.get_id_short(),
                    SubmodelElement::AnnotatedRelationshipElement(elem) => elem.get_id_short(),
                    SubmodelElement::Property(elem) => elem.get_id_short(),
                    SubmodelElement::MultiLanguageProperty(elem) => elem.get_id_short(),
                    SubmodelElement::Range(elem) => elem.get_id_short(),
                    SubmodelElement::Blob(elem) => elem.get_id_short(),
                    SubmodelElement::File(elem) => elem.get_id_short(),
                    SubmodelElement::ReferenceElement(elem) => elem.get_id_short(),
                    SubmodelElement::Capability(elem) => elem.get_id_short(),
                    SubmodelElement::SubmodelElementList(elem) => elem.get_id_short(),
                    SubmodelElement::SubmodelElementCollection(elem) => elem.get_id_short(),
                    SubmodelElement::Entity(elem) => elem.get_id_short(),
                    SubmodelElement::BasicEventElement(elem) => elem.get_id_short(),
                    SubmodelElement::Operation(elem) => elem.get_id_short()
                }
            },
            Referable::SubmodelElementCollection(elem) => elem.get_id_short(),
            Referable::SubmodelElementList(elem) => elem.get_id_short()
        }
    }

    pub fn set_display_name(&mut self, display_name: MultiLanguageNameType) {
        match self {
            Referable::AnnotatedRelationshipElement(elem) => elem.set_display_name(display_name),
            Referable::AssetAdministrationShell(elem) => elem.set_display_name(display_name),
            Referable::BasicEventElement(elem) => elem.set_display_name(display_name),
            Referable::Blob(elem) => elem.set_display_name(display_name),
            Referable::Capability(elem) => elem.set_display_name(display_name),
            Referable::ConceptDescription(elem) => elem.set_display_name(display_name),
            Referable::DataElement(elem) => {
                match elem {
                    DataElement::Property(data_elem) => data_elem.set_display_name(display_name),
                    DataElement::MultiLanguageProperty(data_elem) => data_elem.set_display_name(display_name),
                    DataElement::Range(data_elem) => data_elem.set_display_name(display_name),
                    DataElement::Blob(data_elem) => data_elem.set_display_name(display_name),
                    DataElement::File(data_elem) => data_elem.set_display_name(display_name),
                    DataElement::ReferenceElement(data_elem) => data_elem.set_display_name(display_name),
                }
            },
            Referable::Entity(elem) => elem.set_display_name(display_name),
            Referable::File(elem) => elem.set_display_name(display_name),
            Referable::MultiLanguageProperty(elem) => elem.set_display_name(display_name),
            Referable::Operation(elem) => elem.set_display_name(display_name),
            Referable::Property(elem) => elem.set_display_name(display_name),
            Referable::Range(elem) => elem.set_display_name(display_name),
            Referable::Referable(elem) => elem.set_display_name(display_name),
            Referable::ReferenceElement(elem) => elem.set_display_name(display_name),
            Referable::RelationshipElement(elem) => elem.set_display_name(display_name),
            Referable::Submodel(elem) => elem.set_display_name(display_name),
            Referable::SubmodelElement(elem) => {
                match elem.as_mut() {
                    SubmodelElement::RelationshipElement(elem) => elem.set_display_name(display_name),
                    SubmodelElement::AnnotatedRelationshipElement(elem) => elem.set_display_name(display_name),
                    SubmodelElement::Property(elem) => elem.set_display_name(display_name),
                    SubmodelElement::MultiLanguageProperty(elem) => elem.set_display_name(display_name),
                    SubmodelElement::Range(elem) => elem.set_display_name(display_name),
                    SubmodelElement::Blob(elem) => elem.set_display_name(display_name),
                    SubmodelElement::File(elem) => elem.set_display_name(display_name),
                    SubmodelElement::ReferenceElement(elem) => elem.set_display_name(display_name),
                    SubmodelElement::Capability(elem) => elem.set_display_name(display_name),
                    SubmodelElement::SubmodelElementList(elem) => elem.set_display_name(display_name),
                    SubmodelElement::SubmodelElementCollection(elem) => elem.set_display_name(display_name),
                    SubmodelElement::Entity(elem) => elem.set_display_name(display_name),
                    SubmodelElement::BasicEventElement(elem) => elem.set_display_name(display_name),
                    SubmodelElement::Operation(elem) => elem.set_display_name(display_name)
                }
            },
            Referable::SubmodelElementCollection(elem) => elem.set_display_name(display_name),
            Referable::SubmodelElementList(elem) => elem.set_display_name(display_name)
        }
    }

    pub fn get_display_name(&self) -> Option<&MultiLanguageNameType> {
        match self {
            Referable::AnnotatedRelationshipElement(elem) => elem.get_display_name(),
            Referable::AssetAdministrationShell(elem) => elem.get_display_name(),
            Referable::BasicEventElement(elem) => elem.get_display_name(),
            Referable::Blob(elem) => elem.get_display_name(),
            Referable::Capability(elem) => elem.get_display_name(),
            Referable::ConceptDescription(elem) => elem.get_display_name(),
            Referable::DataElement(elem) => {
                match elem {
                    DataElement::Property(data_elem) => data_elem.get_display_name(),
                    DataElement::MultiLanguageProperty(data_elem) => data_elem.get_display_name(),
                    DataElement::Range(data_elem) => data_elem.get_display_name(),
                    DataElement::Blob(data_elem) => data_elem.get_display_name(),
                    DataElement::File(data_elem) => data_elem.get_display_name(),
                    DataElement::ReferenceElement(data_elem) => data_elem.get_display_name()
                }
            },
            Referable::Entity(elem) => elem.get_display_name(),
            Referable::File(elem) => elem.get_display_name(),
            Referable::MultiLanguageProperty(elem) => elem.get_display_name(),
            Referable::Operation(elem) => elem.get_display_name(),
            Referable::Property(elem) => elem.get_display_name(),
            Referable::Range(elem) => elem.get_display_name(),
            Referable::Referable(elem) => elem.get_display_name(),
            Referable::ReferenceElement(elem) => elem.get_display_name(),
            Referable::RelationshipElement(elem) => elem.get_display_name(),
            Referable::Submodel(elem) => elem.get_display_name(),
            Referable::SubmodelElement(elem) => {
                match elem.as_ref() {
                    SubmodelElement::RelationshipElement(elem) => elem.get_display_name(),
                    SubmodelElement::AnnotatedRelationshipElement(elem) => elem.get_display_name(),
                    SubmodelElement::Property(elem) => elem.get_display_name(),
                    SubmodelElement::MultiLanguageProperty(elem) => elem.get_display_name(),
                    SubmodelElement::Range(elem) => elem.get_display_name(),
                    SubmodelElement::Blob(elem) => elem.get_display_name(),
                    SubmodelElement::File(elem) => elem.get_display_name(),
                    SubmodelElement::ReferenceElement(elem) => elem.get_display_name(),
                    SubmodelElement::Capability(elem) => elem.get_display_name(),
                    SubmodelElement::SubmodelElementList(elem) => elem.get_display_name(),
                    SubmodelElement::SubmodelElementCollection(elem) => elem.get_display_name(),
                    SubmodelElement::Entity(elem) => elem.get_display_name(),
                    SubmodelElement::BasicEventElement(elem) => elem.get_display_name(),
                    SubmodelElement::Operation(elem) => elem.get_display_name()
                }
            },
            Referable::SubmodelElementCollection(elem) => elem.get_display_name(),
            Referable::SubmodelElementList(elem) => elem.get_display_name()
        }
    }

    pub fn set_description(&mut self, description: MultiLanguageTextType) {
        match self {
            Referable::AnnotatedRelationshipElement(elem) => elem.set_description(description),
            Referable::AssetAdministrationShell(elem) => elem.set_description(description),
            Referable::BasicEventElement(elem) => elem.set_description(description),
            Referable::Blob(elem) => elem.set_description(description),
            Referable::Capability(elem) => elem.set_description(description),
            Referable::ConceptDescription(elem) => elem.set_description(description),
            Referable::DataElement(elem) => {
                match elem {
                    DataElement::Property(data_elem) => data_elem.set_description(description),
                    DataElement::MultiLanguageProperty(data_elem) => data_elem.set_description(description),
                    DataElement::Range(data_elem) => data_elem.set_description(description),
                    DataElement::Blob(data_elem) => data_elem.set_description(description),
                    DataElement::File(data_elem) => data_elem.set_description(description),
                    DataElement::ReferenceElement(data_elem) => data_elem.set_description(description),
                }
            },
            Referable::Entity(elem) => elem.set_description(description),
            Referable::File(elem) => elem.set_description(description),
            Referable::MultiLanguageProperty(elem) => elem.set_description(description),
            Referable::Operation(elem) => elem.set_description(description),
            Referable::Property(elem) => elem.set_description(description),
            Referable::Range(elem) => elem.set_description(description),
            Referable::Referable(elem) => elem.set_description(description),
            Referable::ReferenceElement(elem) => elem.set_description(description),
            Referable::RelationshipElement(elem) => elem.set_description(description),
            Referable::Submodel(elem) => elem.set_description(description),
            Referable::SubmodelElement(elem) => {
                match elem.as_mut() {
                    SubmodelElement::RelationshipElement(elem) => elem.set_description(description),
                    SubmodelElement::AnnotatedRelationshipElement(elem) => elem.set_description(description),
                    SubmodelElement::Property(elem) => elem.set_description(description),
                    SubmodelElement::MultiLanguageProperty(elem) => elem.set_description(description),
                    SubmodelElement::Range(elem) => elem.set_description(description),
                    SubmodelElement::Blob(elem) => elem.set_description(description),
                    SubmodelElement::File(elem) => elem.set_description(description),
                    SubmodelElement::ReferenceElement(elem) => elem.set_description(description),
                    SubmodelElement::Capability(elem) => elem.set_description(description),
                    SubmodelElement::SubmodelElementList(elem) => elem.set_description(description),
                    SubmodelElement::SubmodelElementCollection(elem) => elem.set_description(description),
                    SubmodelElement::Entity(elem) => elem.set_description(description),
                    SubmodelElement::BasicEventElement(elem) => elem.set_description(description),
                    SubmodelElement::Operation(elem) => elem.set_description(description),
                }
            },
            Referable::SubmodelElementCollection(elem) => elem.set_description(description),
            Referable::SubmodelElementList(elem) => elem.set_description(description)
        }
    }

    pub fn get_description(&self) -> Option<&MultiLanguageTextType> {
        match self {
            Referable::AnnotatedRelationshipElement(elem) => elem.get_description(),
            Referable::AssetAdministrationShell(elem) => elem.get_description(),
            Referable::BasicEventElement(elem) => elem.get_description(),
            Referable::Blob(elem) => elem.get_description(),
            Referable::Capability(elem) => elem.get_description(),
            Referable::ConceptDescription(elem) => elem.get_description(),
            Referable::DataElement(elem) => {
                match elem {
                    DataElement::Property(data_elem) => data_elem.get_description(),
                    DataElement::MultiLanguageProperty(data_elem) => data_elem.get_description(),
                    DataElement::Range(data_elem) => data_elem.get_description(),
                    DataElement::Blob(data_elem) => data_elem.get_description(),
                    DataElement::File(data_elem) => data_elem.get_description(),
                    DataElement::ReferenceElement(data_elem) => data_elem.get_description()
                }
            },
            Referable::Entity(elem) => elem.get_description(),
            Referable::File(elem) => elem.get_description(),
            Referable::MultiLanguageProperty(elem) => elem.get_description(),
            Referable::Operation(elem) => elem.get_description(),
            Referable::Property(elem) => elem.get_description(),
            Referable::Range(elem) => elem.get_description(),
            Referable::Referable(elem) => elem.get_description(),
            Referable::ReferenceElement(elem) => elem.get_description(),
            Referable::RelationshipElement(elem) => elem.get_description(),
            Referable::Submodel(elem) => elem.get_description(),
            Referable::SubmodelElement(elem) => {
                match elem.as_ref() {
                    SubmodelElement::RelationshipElement(elem) => elem.get_description(),
                    SubmodelElement::AnnotatedRelationshipElement(elem) => elem.get_description(),
                    SubmodelElement::Property(elem) => elem.get_description(),
                    SubmodelElement::MultiLanguageProperty(elem) => elem.get_description(),
                    SubmodelElement::Range(elem) => elem.get_description(),
                    SubmodelElement::Blob(elem) => elem.get_description(),
                    SubmodelElement::File(elem) => elem.get_description(),
                    SubmodelElement::ReferenceElement(elem) => elem.get_description(),
                    SubmodelElement::Capability(elem) => elem.get_description(),
                    SubmodelElement::SubmodelElementList(elem) => elem.get_description(),
                    SubmodelElement::SubmodelElementCollection(elem) => elem.get_description(),
                    SubmodelElement::Entity(elem) => elem.get_description(),
                    SubmodelElement::BasicEventElement(elem) => elem.get_description(),
                    SubmodelElement::Operation(elem) => elem.get_description()
                }
            },
            Referable::SubmodelElementCollection(elem) => elem.get_description(),
            Referable::SubmodelElementList(elem) => elem.get_description()
        }
    }
}