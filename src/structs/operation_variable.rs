use serde::{Deserialize, Serialize};

use crate::enumerations::interface_enumerations::submodel_element::SubmodelElement;

///The value of an operation variable is a submodel element that is used as input and/or output
/// variable of an operation.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct OperationVariable {
    ///Describes an argument or result of an operation via a submodel element.
    value: SubmodelElement
}

impl OperationVariable {
    ///Creates a new instance of a struct of an operation variable.
    pub fn new(value: SubmodelElement) -> OperationVariable {
        OperationVariable {
            value
        }
    }

    ///Sets the description of the operation variable.
    ///
    /// [value]: description of the operation variable
    pub fn set_value(&mut self, value: SubmodelElement) {
        self.value = value
    }

    ///Returns the description of the operation variable.
    pub fn get_value(&self) -> &SubmodelElement {
        &self.value
    }

    ///Returns the mutable description of the operation variable.
    pub fn get_mut_value(&mut self) -> &mut SubmodelElement {
        &mut self.value
    }
}