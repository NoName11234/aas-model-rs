use serde::{Deserialize, Serialize};

use crate::structs::extension::Extension;
use crate::structs::multi_language_name_type::MultiLanguageNameType;
use crate::structs::multi_language_text_type::MultiLanguageTextType;
use crate::structs::operation_variable::OperationVariable;
use crate::structs::qualifier::Qualifier;
use crate::structs::reference::Reference;
use crate::traits::has_data_specification::THasDataSpecification;
use crate::traits::has_extensions::THasExtensions;
use crate::traits::has_semantics::THasSemantics;
use crate::traits::qualifiable::TQualifiable;
use crate::traits::referable::TReferable;
use crate::traits::submodel_element::TSubmodelElement;

///An operation is a submodel element with input and output variables.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Operation {
    ///Optional input parameter of the operation.
    #[serde(rename = "inputVariables")]
    input_variables: Vec<OperationVariable>,
    ///Optional output parameters of the operation.
    #[serde(rename = "outputVariables")]
    output_variables: Vec<OperationVariable>,
    ///Optional parameter that is input and output of the operation.
    #[serde(rename = "inoutputVariables")]
    inoutput_variables: Vec<OperationVariable>,
    category: Option<String>,
    #[serde(rename = "idShort")]
    id_short: Option<String>,
    #[serde(rename = "displayName")]
    display_name: Option<MultiLanguageNameType>,
    description: Option<MultiLanguageTextType>,
    extensions: Vec<Extension>,
    #[serde(rename = "semanticId")]
    semantic_id: Option<Reference>,
    #[serde(rename = "supplementalSemanticIds")]
    supplemental_semantic_ids: Vec<Reference>,
    qualifiers: Vec<Qualifier>,
    #[serde(rename = "embeddedDataSpecifications")]
    data_specifications: Vec<Reference>
}

impl Operation {
    ///Creates a new instance of a struct of an operation.
    pub fn new() -> Operation {
        Operation {
            input_variables: Vec::new(),
            output_variables: Vec::new(),
            inoutput_variables: Vec::new(),
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            extensions: Vec::new(),
            semantic_id: None,
            supplemental_semantic_ids: Vec::new(),
            qualifiers: Vec::new(),
            data_specifications: Vec::new(),
        }
    }

    ///Sets the input variables of the operation.
    ///
    /// [input_variables]: input variables of the operation
    pub fn set_input_variables(&mut self, input_variables: Vec<OperationVariable>) {
        self.input_variables = input_variables;
    }

    ///Returns the optional input variables of the operation.
    pub fn get_input_variables(&self) -> &Vec<OperationVariable> {
        &self.input_variables
    }

    ///Adds an input variable of the operation.
    ///
    /// [input_variable]: input variable of the operation
    pub fn add_input_variable(&mut self, input_variable: OperationVariable) {
        self.input_variables.push(input_variable);
    }

    ///Removes the specified input variable.
    ///
    /// [index]: index of the input variable in the vector
    pub fn remove_input_variable(&mut self, index: usize) -> OperationVariable {
        self.input_variables.remove(index)
    }

    ///Sets the output variables of the operation.
    ///
    /// [output_variables]: output variables of the operation
    pub fn set_output_variables(&mut self, output_variables: Vec<OperationVariable>) {
        self.output_variables = output_variables;
    }

    ///Returns the output variables of the operation.
    pub fn get_output_variables(&self) -> &Vec<OperationVariable> {
        &self.output_variables
    }

    ///Adds an output variable of the operation.
    ///
    /// [output_variable]: output variable of the operation
    pub fn add_output_variable(&mut self, output_variable: OperationVariable) {
        self.output_variables.push(output_variable);
    }

    ///Removes the specified output variable.
    ///
    /// [index]: index of the output variable in the vector
    pub fn remove_output_variable(&mut self, index: usize) -> OperationVariable {
        self.output_variables.remove(index)
    }

    ///Sets a list of variables which are input and output of the operation.
    ///
    /// [inoutput_variables]: input and out variables of the operation
    pub fn set_inoutput_variables(&mut self, inoutput_variables: Vec<OperationVariable>) {
        self.inoutput_variables = inoutput_variables;
    }

    ///Returns a list of variables which are input and output of the operation.
    pub fn get_inoutput_variables(&self) -> &Vec<OperationVariable> {
        &self.inoutput_variables
    }

    ///Adds a variable that is input and output of the operation.
    ///
    /// [inoutput_variable]: variable that is input and output of the operation
    pub fn add_inoutput_variable(&mut self, inoutput_variable: OperationVariable) {
        self.inoutput_variables.push(inoutput_variable);
    }

    ///Removes the specified inoutput variable.
    ///
    /// [index]: index of the inouput variable in the vector
    pub fn remove_inoutput_variable(&mut self, index: usize) -> OperationVariable {
        self.inoutput_variables.remove(index)
    }
}

impl TReferable for Operation {
    fn set_category(&mut self, category: String) {
        self.category = Some(category);
    }

    fn get_category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    fn set_id_short(&mut self, id_short: String) {
        self.id_short = Some(id_short);
    }

    fn get_id_short(&self) -> Option<&String> {
        self.id_short.as_ref()
    }

    fn set_display_name(&mut self, display_name: MultiLanguageNameType) {
        self.display_name = Some(display_name);
    }

    fn get_display_name(&self) -> Option<&MultiLanguageNameType> {
        self.display_name.as_ref()
    }

    fn set_description(&mut self, description: MultiLanguageTextType) {
        self.description = Some(description);
    }

    fn get_description(&self) -> Option<&MultiLanguageTextType> {
        self.description.as_ref()
    }
}

impl THasExtensions for Operation {
    fn get_extensions(&self) -> &Vec<Extension> {
        &self.extensions
    }

    fn set_extensions(&mut self, extensions: Vec<Extension>) {
        self.extensions = extensions;
    }

    fn add_extension(&mut self, extension: Extension) {
        self.extensions.push(extension);
    }

    fn remove_extension(&mut self, index: usize) -> Extension {
        self.extensions.remove(index)
    }
}

impl THasSemantics for Operation {
    fn set_semantic_id(&mut self, semantic_id: Reference) {
        self.semantic_id = Some(semantic_id);
    }

    fn get_semantic_id(&self) -> Option<&Reference> {
        self.semantic_id.as_ref()
    }

    fn set_supplemental_semantic_ids(&mut self, supplemental_semantic_ids: Vec<Reference>) {
        self.supplemental_semantic_ids = supplemental_semantic_ids;
    }

    fn get_supplemental_semantic_ids(&self) -> &Vec<Reference> {
        &self.supplemental_semantic_ids
    }

    fn add_supplemental_semantic_id(&mut self, supplemental_semantic_id: Reference) {
        self.supplemental_semantic_ids.push(supplemental_semantic_id);
    }

    fn remove_supplemental_semantic_id(&mut self, index: usize) -> Reference {
        self.supplemental_semantic_ids.remove(index)
    }
}

impl TQualifiable for Operation {
    fn set_qualifiers(&mut self, qualifiers: Vec<Qualifier>) {
        self.qualifiers = qualifiers;
    }

    fn get_qualifiers(&self) -> &Vec<Qualifier> {
        &self.qualifiers
    }

    fn add_qualifier(&mut self, qualifier: Qualifier) {
        self.qualifiers.push(qualifier);
    }

    fn remove_qualifier(&mut self, index: usize) -> Qualifier {
        self.qualifiers.remove(index)
    }
}

impl THasDataSpecification for Operation {
    fn get_data_specifications(&self) -> &Vec<Reference> {
        &self.data_specifications
    }

    fn set_data_specifications(&mut self, data_specifications: Vec<Reference>) {
        self.data_specifications = data_specifications;
    }

    fn add_data_specification(&mut self, data_specification: Reference) {
        self.data_specifications.push(data_specification);
    }

    fn remove_data_specification(&mut self, index: usize) -> Reference {
        self.data_specifications.remove(index)
    }
}

impl TSubmodelElement for Operation {

}