use crate::structs::administrative_information::AdministrativeInformation;
use crate::structs::annotated_relationship_element::AnnotatedRelationshipElement;
use crate::structs::asset_administration_shell::AssetAdministrationShell;
use crate::structs::basic_event_element::BasicEventElement;
use crate::structs::blob::Blob;
use crate::structs::capability::Capability;
use crate::structs::concept_description::ConceptDescription;
use crate::structs::entity::Entity;
use crate::structs::file::File;
use crate::structs::multi_language_property::MultiLanguageProperty;
use crate::structs::operation::Operation;
use crate::structs::property::Property;
use crate::structs::range::Range;
use crate::structs::reference::Reference;
use crate::structs::reference_element::ReferenceElement;
use crate::structs::relationship_element::RelationshipElement;
use crate::structs::submodel::Submodel;
use crate::structs::submodel_element_collection::SubmodelElementCollection;
use crate::structs::submodel_element_list::SubmodelElementList;
use crate::traits::has_data_specification::THasDataSpecification;

#[derive(PartialEq, Clone)]
pub enum HasDataSpecification {
    BasicEventElement(BasicEventElement),
    Capability(Capability),
    SubmodelElementCollection(SubmodelElementCollection),
    RelationshipElement(RelationshipElement),
    ReferenceElement(ReferenceElement),
    Property(Property),
    MultiLanguageProperty(MultiLanguageProperty),
    AnnotatedRelationshipElement(AnnotatedRelationshipElement),
    AssetAdministrationShell(AssetAdministrationShell),
    AdministrativeInformation(AdministrativeInformation),
    Entity(Entity),
    Operation(Operation),
    Range(Range),
    Blob(Blob),
    File(File),
    SubmodelElementList(SubmodelElementList),
    Submodel(Submodel),
    ConceptDescription(ConceptDescription)
}

impl HasDataSpecification {
    pub fn get_data_specifications(&self) -> &Vec<Reference> {
        match self { 
            HasDataSpecification::BasicEventElement(elem) => elem.get_data_specifications(),
            HasDataSpecification::Capability(elem) => elem.get_data_specifications(),
            HasDataSpecification::SubmodelElementCollection(elem) => elem.get_data_specifications(),
            HasDataSpecification::RelationshipElement(elem) => elem.get_data_specifications(),
            HasDataSpecification::ReferenceElement(elem) => elem.get_data_specifications(),
            HasDataSpecification::Property(elem) => elem.get_data_specifications(),
            HasDataSpecification::MultiLanguageProperty(elem) => elem.get_data_specifications(),
            HasDataSpecification::AnnotatedRelationshipElement(elem) => elem.get_data_specifications(),
            HasDataSpecification::AssetAdministrationShell(elem) => elem.get_data_specifications(),
            HasDataSpecification::AdministrativeInformation(elem) => elem.get_data_specifications(),
            HasDataSpecification::Entity(elem) => elem.get_data_specifications(),
            HasDataSpecification::Operation(elem) => elem.get_data_specifications(),
            HasDataSpecification::Range(elem) => elem.get_data_specifications(),
            HasDataSpecification::Blob(elem) => elem.get_data_specifications(),
            HasDataSpecification::File(elem) => elem.get_data_specifications(),
            HasDataSpecification::SubmodelElementList(elem) => elem.get_data_specifications(),
            HasDataSpecification::Submodel(elem) => elem.get_data_specifications(),
            HasDataSpecification::ConceptDescription(elem) => elem.get_data_specifications()
        }
    }
    
    pub fn set_data_specifications(&mut self, data_specifications: Vec<Reference>) {
        match self {
            HasDataSpecification::BasicEventElement(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::Capability(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::SubmodelElementCollection(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::RelationshipElement(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::ReferenceElement(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::Property(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::MultiLanguageProperty(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::AnnotatedRelationshipElement(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::AssetAdministrationShell(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::AdministrativeInformation(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::Entity(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::Operation(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::Range(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::Blob(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::File(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::SubmodelElementList(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::Submodel(elem) => elem.set_data_specifications(data_specifications),
            HasDataSpecification::ConceptDescription(elem) => elem.set_data_specifications(data_specifications)
        }
    }
    
    pub fn add_data_specification(&mut self, data_specification: Reference) {
        match self {
            HasDataSpecification::BasicEventElement(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::Capability(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::SubmodelElementCollection(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::RelationshipElement(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::ReferenceElement(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::Property(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::MultiLanguageProperty(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::AnnotatedRelationshipElement(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::AssetAdministrationShell(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::AdministrativeInformation(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::Entity(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::Operation(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::Range(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::Blob(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::File(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::SubmodelElementList(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::Submodel(elem) => elem.add_data_specification(data_specification),
            HasDataSpecification::ConceptDescription(elem) => elem.add_data_specification(data_specification)
        }
    }
    
    pub fn remove_data_specification(&mut self, index: usize) -> Reference {
        match self {
            HasDataSpecification::BasicEventElement(elem) => elem.remove_data_specification(index),
            HasDataSpecification::Capability(elem) => elem.remove_data_specification(index),
            HasDataSpecification::SubmodelElementCollection(elem) => elem.remove_data_specification(index),
            HasDataSpecification::RelationshipElement(elem) => elem.remove_data_specification(index),
            HasDataSpecification::ReferenceElement(elem) => elem.remove_data_specification(index),
            HasDataSpecification::Property(elem) => elem.remove_data_specification(index),
            HasDataSpecification::MultiLanguageProperty(elem) => elem.remove_data_specification(index),
            HasDataSpecification::AnnotatedRelationshipElement(elem) => elem.remove_data_specification(index),
            HasDataSpecification::AssetAdministrationShell(elem) => elem.remove_data_specification(index),
            HasDataSpecification::AdministrativeInformation(elem) => elem.remove_data_specification(index),
            HasDataSpecification::Entity(elem) => elem.remove_data_specification(index),
            HasDataSpecification::Operation(elem) => elem.remove_data_specification(index),
            HasDataSpecification::Range(elem) => elem.remove_data_specification(index),
            HasDataSpecification::Blob(elem) => elem.remove_data_specification(index),
            HasDataSpecification::File(elem) => elem.remove_data_specification(index),
            HasDataSpecification::SubmodelElementList(elem) => elem.remove_data_specification(index),
            HasDataSpecification::Submodel(elem) => elem.remove_data_specification(index),
            HasDataSpecification::ConceptDescription(elem) => elem.remove_data_specification(index)
        }
    }
}