use serde::{Deserialize, Serialize};

use crate::structs::extension::Extension;
use crate::structs::multi_language_name_type::MultiLanguageNameType;
use crate::structs::multi_language_text_type::MultiLanguageTextType;
use crate::structs::qualifier::Qualifier;
use crate::structs::reference::Reference;
use crate::traits::data_element::TDataElement;
use crate::traits::has_data_specification::THasDataSpecification;
use crate::traits::has_extensions::THasExtensions;
use crate::traits::has_semantics::THasSemantics;
use crate::traits::qualifiable::TQualifiable;
use crate::traits::referable::TReferable;
use crate::traits::submodel_element::TSubmodelElement;

///Data element that has a multi-language value.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct MultiLanguageProperty {
    ///Optional value of the property instance.
    value: Vec<MultiLanguageTextType>,
    ///Optional reference to the global unique ID of a coded value.
    #[serde(rename = "valueId")]
    value_id: Option<Reference>,
    category: Option<String>,
    #[serde(rename = "idShort")]
    id_short: Option<String>,
    #[serde(rename = "displayName")]
    display_name: Vec<MultiLanguageNameType>,
    description: Vec<MultiLanguageTextType>,
    extensions: Vec<Extension>,
    #[serde(rename = "semanticId")]
    semantic_id: Option<Reference>,
    #[serde(rename = "supplementalSemanticIds")]
    supplemental_semantic_ids: Vec<Reference>,
    qualifiers: Vec<Qualifier>,
    #[serde(rename = "embeddedDataSpecifications")]
    data_specifications: Vec<Reference>
}

impl MultiLanguageProperty {
    ///Creates a new instance of a struct of a multi-language property.
    pub fn new() -> MultiLanguageProperty {
        MultiLanguageProperty {
            value: Vec::new(),
            value_id: None,
            category: None,
            id_short: None,
            display_name: Vec::new(),
            description: Vec::new(),
            extensions: Vec::new(),
            semantic_id: None,
            supplemental_semantic_ids: Vec::new(),
            qualifiers: Vec::new(),
            data_specifications: Vec::new()
        }
    }

    ///Sets the value of the multi-language property instance.
    ///
    /// [value]: value of the multi-language property instance
    pub fn set_value(&mut self, value: Vec<MultiLanguageTextType>) {
        self.value = value;
    }

    ///Returns the value of the multi-language property instance.
    pub fn get_value(&self) -> &Vec<MultiLanguageTextType> {
        &self.value
    }

    ///Adds a value to the multi-language property instance.
    /// 
    /// [value]: the value to be added
    pub fn add_value(&mut self, value: MultiLanguageTextType) {
        self.value.push(value);
    }

    ///Removes a value from the multi-language property instance specified by an index.
    /// Returns the removed value.
    pub fn remove_value(&mut self, index: usize) -> MultiLanguageTextType {
        self.value.remove(index)
    }

    ///Sets the reference to the global unique ID of a coded value.
    ///
    /// [value_id]: reference to the global unique ID of a coded value
    pub fn set_value_id(&mut self, value_id: Reference) {
        self.value_id = Some(value_id);
    }

    ///Returns the reference to the global unique ID of a coded value.
    pub fn get_value_id(&self) -> Option<&Reference> {
        self.value_id.as_ref()
    }
}

impl TSubmodelElement for MultiLanguageProperty {}

impl TReferable for MultiLanguageProperty {
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

    fn set_display_name(&mut self, display_name: Vec<MultiLanguageNameType>) {
        self.display_name = display_name;
    }

    fn get_display_name(&self) -> &Vec<MultiLanguageNameType> {
        &self.display_name
    }

    fn add_display_name(&mut self, display_name: MultiLanguageNameType) {
        self.display_name.push(display_name);
    }

    fn remove_display_name(&mut self, index: usize) -> MultiLanguageNameType {
        self.display_name.remove(index)
    }

    fn set_description(&mut self, description: Vec<MultiLanguageTextType>) {
        self.description = description;
    }

    fn get_description(&self) -> &Vec<MultiLanguageTextType> {
        &self.description
    }

    fn add_description(&mut self, description: MultiLanguageTextType) {
        self.description.push(description);
    }

    fn remove_description(&mut self, index: usize) -> MultiLanguageTextType {
        self.description.remove(index)
    }
}

impl THasExtensions for MultiLanguageProperty {
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

impl THasSemantics for MultiLanguageProperty {
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

impl TQualifiable for MultiLanguageProperty {
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

impl THasDataSpecification for MultiLanguageProperty {
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

impl TDataElement for MultiLanguageProperty {

}