use serde::{Deserialize, Serialize};

use crate::enumerations::data_type_def_xsd::DataTypeDefXsd;
use crate::structs::reference::Reference;
use crate::traits::has_semantics::THasSemantics;

///A single extension of an element.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Extension {
    ///The name of the extension.
    name: String,
    ///The optional data type of the value attribute of the extension.
    #[serde(rename = "valueType")]
    value_type: Option<DataTypeDefXsd>,
    ///The optional value of the extension.
    value: Option<String>,
    ///List of references to an element the extension refers to.
    #[serde(rename = "refersTo")]
    refers_to: Vec<Reference>,
    #[serde(rename = "semanticId")]
    semantic_id: Option<Reference>,
    #[serde(rename = "supplementalSemanticIds")]
    supplemental_semantic_ids: Vec<Reference>
}

impl Extension {
    ///Creates a new instance of the struct.
    /// [name]: name of the extension
    pub fn new(name: String) -> Extension {
        Extension {
            name,
            value_type: None,
            value: None,
            refers_to: Vec::new(),
            semantic_id: None,
            supplemental_semantic_ids: Vec::new()
        }
    }

    ///Sets the name of the extension.
    /// [name]: name of the extension
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    ///Returns the name of the extension.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    ///Returns the mutable name of the extension.
    pub fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }

    ///Sets the data type of the value attribute of the extension.
    /// [value_type]: data type of the value
    pub fn set_value_type(&mut self, value_type: DataTypeDefXsd) {
        self.value_type = Some(value_type);
    }

    ///Returns the data type of the value attribute of the extension.
    pub fn get_value_type(&self) -> Option<&DataTypeDefXsd> {
        self.value_type.as_ref()
    }

    ///Returns the mutable data type of the value attribute of the extension.
    pub fn get_mut_value_type(&mut self) -> Option<&mut DataTypeDefXsd> {
        self.value_type.as_mut()
    }

    ///Sets the value of the extension.
    /// [value]: value of extension
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    ///Returns the value of the extension.
    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    ///Returns the mutable value of the extension.
    pub fn get_mut_value(&mut self) -> Option<&mut String> {
        self.value.as_mut()
    }

    ///Sets the list of references to elements the extension refers to.
    /// [refers_to]: list of references
    pub fn set_refers_to(&mut self, refers_to: Vec<Reference>) {
        self.refers_to = refers_to;
    }

    ///Returns the list of references to elements the extension refers to.
    pub fn get_refers_to(&self) -> &Vec<Reference> {
        &self.refers_to
    }

    ///Returns the mutable list of references to elements the extension refers to.
    pub fn get_mut_refers_to(&mut self) -> &mut Vec<Reference> {
        &mut self.refers_to
    }

    ///Adds a reference to an element the extension refers to.
    /// [refers_to]: reference to an element
    pub fn add_refers_to(&mut self, refers_to: Reference) {
        self.refers_to.push(refers_to);
    }

    ///Removes a reference to an element from the list.
    /// [index]: index of the reference
    pub fn remove_refers_to(&mut self, index: usize) -> Reference {
        self.refers_to.remove(index)
    }
}

impl THasSemantics for Extension {
    fn set_semantic_id(&mut self, supplemental_semantic_id: Reference) {
        self.semantic_id = Some(supplemental_semantic_id);
    }

    fn get_semantic_id(&self) -> Option<&Reference> {
        self.semantic_id.as_ref()
    }

    fn get_mut_semantic_id(&mut self) -> Option<&mut Reference> {
        self.semantic_id.as_mut()
    }

    fn set_supplemental_semantic_ids(&mut self, supplemental_semantic_ids: Vec<Reference>) {
        self.supplemental_semantic_ids = supplemental_semantic_ids;
    }

    fn get_supplemental_semantic_ids(&self) -> &Vec<Reference> {
        &self.supplemental_semantic_ids
    }

    fn get_mut_supplemental_semantic_ids(&mut self) -> &mut Vec<Reference> {
        &mut self.supplemental_semantic_ids
    }

    fn add_supplemental_semantic_id(&mut self, semantic_id: Reference) {
        self.supplemental_semantic_ids.push(semantic_id);
    }

    fn remove_supplemental_semantic_id(&mut self, index: usize) -> Reference {
        self.supplemental_semantic_ids.remove(index)
    }
}