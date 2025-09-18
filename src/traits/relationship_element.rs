use crate::structs::reference::Reference;
use crate::traits::submodel_element::TSubmodelElement;

pub trait TRelationshipElement: TSubmodelElement {
    fn set_first(&mut self, first: Reference);
    fn get_first(&self) -> Option<&Reference>;
    fn get_mut_first(&mut self) -> Option<&mut Reference>;
    fn set_second(&mut self, second: Reference);
    fn get_second(&self) -> Option<&Reference>;
    fn get_mut_second(&mut self) -> Option<&mut Reference>;
}