use serde::{Deserialize, Serialize};

use crate::enumerations::aas_submodel_elements::AasSubmodelElements;
use crate::enumerations::data_type_def_xsd::DataTypeDefXsd;
use crate::enumerations::interface_enumerations::submodel_element::SubmodelElement;
use crate::structs::extension::Extension;
use crate::structs::multi_language_name_type::MultiLanguageNameType;
use crate::structs::multi_language_text_type::MultiLanguageTextType;
use crate::structs::qualifier::Qualifier;
use crate::structs::reference::Reference;
use crate::traits::has_data_specification::THasDataSpecification;
use crate::traits::has_extensions::THasExtensions;
use crate::traits::has_semantics::THasSemantics;
use crate::traits::qualifiable::TQualifiable;
use crate::traits::referable::TReferable;
use crate::traits::submodel_element::TSubmodelElement;

///A submodel element list is an ordered list of submodel elements.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct SubmodelElementList {
    ///Defines whether order in list is relevant.
    #[serde(rename = "orderRelevant")]
    order_relevant: Option<bool>,
    ///Submodel elements contained in the list.
    value: Vec<SubmodelElement>,
    ///Optional semantic ID which the submodel elements contained in the list match.
    #[serde(rename = "semanticIdListElement")]
    semantic_id_list_element: Option<Reference>,
    ///The submodel element type of the submodel elements contained in the list.
    type_value_list_element: AasSubmodelElements,
    ///Optional value type of the submodel element contained in the list.
    #[serde(rename = "valueTypeListElement")]
    value_type_list_element: Option<DataTypeDefXsd>,
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

impl SubmodelElementList {
    ///Creates a new instance of the struct.
    pub fn new(type_value_list_element: AasSubmodelElements) -> SubmodelElementList {
        SubmodelElementList {
            order_relevant: None,
            value: Vec::new(),
            semantic_id_list_element: None,
            type_value_list_element,
            value_type_list_element: None,
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

    ///Sets whether order in list is relevant.
    ///
    /// [order_relevant]: relevance of order
    pub fn set_order_relevant(&mut self, order_relevant: bool) {
        self.order_relevant = Some(order_relevant);
    }

    ///Returns whether order in list is relevant.
    pub fn get_order_relevant(&self) -> Option<&bool> {
        self.order_relevant.as_ref()
    }

    ///Sets the list of submodel elements contained in the list.
    ///
    /// [values]: list of submodel elements
    pub fn set_value(&mut self, values: Vec<SubmodelElement>) {
        self.value = values;
    }

    ///Returns the list of submodel elements contained in the list.
    pub fn get_value(&self) -> &Vec<SubmodelElement> {
        &self.value
    }

    ///Adds a submodel element to the list.
    pub fn add_value(&mut self, value: SubmodelElement) {
        self.value.push(value);
    }

    ///Removes a submodel element from the list.
    pub fn remove_value(&mut self, index: usize) -> SubmodelElement {
        self.value.remove(index)
    }

    ///The semantic ID which the submodel elements contained in the list match.
    ///
    /// [semantic_id_list_element]: semantic ID
    pub fn set_semantic_id_list_element(&mut self, semantic_id_list_element: Reference) {
        self.semantic_id_list_element = Some(semantic_id_list_element);
    }

    ///Returns the semantic ID which the submodel elements contained in the list match.
    pub fn get_semantic_id_list_element(&self) -> Option<&Reference> {
        self.semantic_id_list_element.as_ref()
    }

    ///Sets the submodel element type of the submodel elements contained in the list.
    ///
    /// [type_value_list_element]: submodel element type
    pub fn set_type_value_list_element(&mut self, type_value_list_element: AasSubmodelElements) {
        self.type_value_list_element = type_value_list_element;
    }

    ///Returns the submodel element type of the submodel elements contained in the list.
    pub fn get_type_value_list_element(&self) -> &AasSubmodelElements {
        &self.type_value_list_element
    }

    ///Sets the value type of the submodel element contained in the list.
    ///
    /// [value_type_list_element]: value type
    pub fn set_value_type_list_element(&mut self, value_type_list_element: DataTypeDefXsd) {
        self.value_type_list_element = Some(value_type_list_element);
    }

    ///Returns the value type of the submodel element contained in the list.
    pub fn get_value_type_list_element(&self) -> Option<&DataTypeDefXsd> {
        self.value_type_list_element.as_ref()
    }
}

impl TReferable for SubmodelElementList {
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

impl THasExtensions for SubmodelElementList {
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

impl THasSemantics for SubmodelElementList {
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

impl TQualifiable for SubmodelElementList {
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

impl THasDataSpecification for SubmodelElementList {
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

impl TSubmodelElement for SubmodelElementList {

}