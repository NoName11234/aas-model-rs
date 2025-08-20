use crate::traits::submodel_element::TSubmodelElement;

///A data element is a submodel element that is not further composed of other submodel elements.
///
/// A data element is a submodel element that has a value. The type of value differs for different
/// subtypes of data elements.
pub trait TDataElement: TSubmodelElement {

}