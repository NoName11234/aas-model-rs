use crate::structs::annotated_relationship_element::AnnotatedRelationshipElement;
use crate::structs::reference::Reference;
use crate::traits::relationship_element::TRelationshipElement;

#[derive(PartialEq, Clone)]
pub enum RelationshipElement {
    AnnotatedRelationshipElement(AnnotatedRelationshipElement)
}

impl RelationshipElement {
    pub fn set_first(&mut self, first: Reference) {
        match self { 
            RelationshipElement::AnnotatedRelationshipElement(elem) => elem.set_first(first)
        }
    }
    
    pub fn get_first(&self) -> Option<&Reference> {
        match self { 
            RelationshipElement::AnnotatedRelationshipElement(elem) => elem.get_first()
        }
    }
    
    pub fn set_second(&mut self, second: Reference) {
        match self { 
            RelationshipElement::AnnotatedRelationshipElement(elem) => elem.set_second(second)
        }
    }
    
    pub fn get_second(&self) -> Option<&Reference> {
        match self { 
            RelationshipElement::AnnotatedRelationshipElement(elem) => elem.get_second()
        }
    }
}