use crate::structs::annotated_relationship_element::AnnotatedRelationshipElement;
use crate::structs::basic_event_element::BasicEventElement;
use crate::structs::blob::Blob;
use crate::structs::capability::Capability;
use crate::structs::entity::Entity;
use crate::structs::file::File;
use crate::structs::multi_language_property::MultiLanguageProperty;
use crate::structs::operation::Operation;
use crate::structs::property::Property;
use crate::structs::qualifier::Qualifier;
use crate::structs::range::Range;
use crate::structs::reference_element::ReferenceElement;
use crate::structs::relationship_element::RelationshipElement;
use crate::structs::submodel::Submodel;
use crate::structs::submodel_element_collection::SubmodelElementCollection;
use crate::structs::submodel_element_list::SubmodelElementList;
use crate::traits::qualifiable::TQualifiable;

#[derive(PartialEq, Clone)]
pub enum Qualifiable {
    AnnotatedRelationshipElement(AnnotatedRelationshipElement),
    BasicEventElement(BasicEventElement),
    Blob(Blob),
    Capability(Capability),
    Entity(Entity),
    File(File),
    MultiLanguageProperty(MultiLanguageProperty),
    Operation(Operation),
    Property(Property),
    Range(Range),
    ReferenceElement(ReferenceElement),
    RelationshipElement(RelationshipElement),
    Submodel(Submodel),
    SubmodelElementCollection(SubmodelElementCollection),
    SubmodelElementList(SubmodelElementList)
}

impl Qualifiable {
    pub fn set_qualifiers(&mut self, qualifiers: Vec<Qualifier>) {
        match self {
            Qualifiable::AnnotatedRelationshipElement(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::BasicEventElement(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::Blob(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::Capability(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::Entity(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::File(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::MultiLanguageProperty(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::Operation(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::Property(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::Range(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::ReferenceElement(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::RelationshipElement(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::Submodel(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::SubmodelElementCollection(elem) => elem.set_qualifiers(qualifiers),
            Qualifiable::SubmodelElementList(elem) => elem.set_qualifiers(qualifiers)
        }
    }

    pub fn get_qualifiers(&self) -> &Vec<Qualifier> {
        match self {
            Qualifiable::AnnotatedRelationshipElement(elem) => elem.get_qualifiers(),
            Qualifiable::BasicEventElement(elem) => elem.get_qualifiers(),
            Qualifiable::Blob(elem) => elem.get_qualifiers(),
            Qualifiable::Capability(elem) => elem.get_qualifiers(),
            Qualifiable::Entity(elem) => elem.get_qualifiers(),
            Qualifiable::File(elem) => elem.get_qualifiers(),
            Qualifiable::MultiLanguageProperty(elem) => elem.get_qualifiers(),
            Qualifiable::Operation(elem) => elem.get_qualifiers(),
            Qualifiable::Property(elem) => elem.get_qualifiers(),
            Qualifiable::Range(elem) => elem.get_qualifiers(),
            Qualifiable::ReferenceElement(elem) => elem.get_qualifiers(),
            Qualifiable::RelationshipElement(elem) => elem.get_qualifiers(),
            Qualifiable::Submodel(elem) => elem.get_qualifiers(),
            Qualifiable::SubmodelElementCollection(elem) => elem.get_qualifiers(),
            Qualifiable::SubmodelElementList(elem) => elem.get_qualifiers()
        }
    }

    pub fn add_qualifier(&mut self, qualifier: Qualifier) {
        match self {
            Qualifiable::AnnotatedRelationshipElement(elem) => elem.add_qualifier(qualifier),
            Qualifiable::BasicEventElement(elem) => elem.add_qualifier(qualifier),
            Qualifiable::Blob(elem) => elem.add_qualifier(qualifier),
            Qualifiable::Capability(elem) => elem.add_qualifier(qualifier),
            Qualifiable::Entity(elem) => elem.add_qualifier(qualifier),
            Qualifiable::File(elem) => elem.add_qualifier(qualifier),
            Qualifiable::MultiLanguageProperty(elem) => elem.add_qualifier(qualifier),
            Qualifiable::Operation(elem) => elem.add_qualifier(qualifier),
            Qualifiable::Property(elem) => elem.add_qualifier(qualifier),
            Qualifiable::Range(elem) => elem.add_qualifier(qualifier),
            Qualifiable::ReferenceElement(elem) => elem.add_qualifier(qualifier),
            Qualifiable::RelationshipElement(elem) => elem.add_qualifier(qualifier),
            Qualifiable::Submodel(elem) => elem.add_qualifier(qualifier),
            Qualifiable::SubmodelElementCollection(elem) => elem.add_qualifier(qualifier),
            Qualifiable::SubmodelElementList(elem) => elem.add_qualifier(qualifier)
        }
    }

    pub fn remove_qualifier(&mut self, index: usize) -> Qualifier {
        match self {
            Qualifiable::AnnotatedRelationshipElement(elem) => elem.remove_qualifier(index),
            Qualifiable::BasicEventElement(elem) => elem.remove_qualifier(index),
            Qualifiable::Blob(elem) => elem.remove_qualifier(index),
            Qualifiable::Capability(elem) => elem.remove_qualifier(index),
            Qualifiable::Entity(elem) => elem.remove_qualifier(index),
            Qualifiable::File(elem) => elem.remove_qualifier(index),
            Qualifiable::MultiLanguageProperty(elem) => elem.remove_qualifier(index),
            Qualifiable::Operation(elem) => elem.remove_qualifier(index),
            Qualifiable::Property(elem) => elem.remove_qualifier(index),
            Qualifiable::Range(elem) => elem.remove_qualifier(index),
            Qualifiable::ReferenceElement(elem) => elem.remove_qualifier(index),
            Qualifiable::RelationshipElement(elem) => elem.remove_qualifier(index),
            Qualifiable::Submodel(elem) => elem.remove_qualifier(index),
            Qualifiable::SubmodelElementCollection(elem) => elem.remove_qualifier(index),
            Qualifiable::SubmodelElementList(elem) => elem.remove_qualifier(index)
        }
    }
}