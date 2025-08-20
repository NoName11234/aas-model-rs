use crate::structs::reference::Reference;

///Trait with functions for an element that can be extended by using data specification templates.
/// A data specification template defines a named set of additional attributes an element may or
/// shall have. The data specifications used are explicitly specified with their global ID.
pub trait THasDataSpecification {
    ///Returns a list of external references to the data specification templates used by the element.
    fn get_data_specifications(&self) -> &Vec<Reference>;
    ///Sets the list of external references to the data specification templates used by the element.
    /// [data_specifications]: list of external references
    fn set_data_specifications(&mut self, data_specifications: Vec<Reference>);
    ///Adds an external reference to a data specification template used by the element.
    /// [data_specification]: external reference
    fn add_data_specification(&mut self, data_specification: Reference);
    ///Removes an external reference to a data specification template used by the element.
    /// [index]: index of the reference to be removed
    fn remove_data_specification(&mut self, index: usize) -> Reference;
}