use serde::{Deserialize, Serialize};

use crate::enumerations::data_type_def_xsd::DataTypeDefXsd;
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

///A range data element is a data element that defines a range with min and max.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Range {
    ///Data type of the min and max attributes.
    #[serde(rename = "valueType")]
    value_type: DataTypeDefXsd,
    ///The minimum value of the range.
    min: Option<String>,
    ///The maximum value of the range.
    max: Option<String>,
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

impl Range {
    ///Creates a new instance of a struct of a range data element.
    ///
    ///[value_type]: data type of the min and max attributes
    pub fn new(value_type: DataTypeDefXsd) -> Range {
        Range {
            value_type,
            min: None,
            max: None,
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

    ///Sets the data type of the min and max attributes.
    ///
    /// [value_type]: data type of min and max attribute
    pub fn set_value_type(&mut self, value_type: DataTypeDefXsd) {
        self.value_type = value_type;
    }

    ///Returns the data type of the min und max attributes.
    pub fn get_value_type(&self) -> &DataTypeDefXsd {
        &self.value_type
    }

    ///Sets the minimum value of the range.
    ///
    /// [min]: minimum value of the range
    pub fn set_min(&mut self, min: String) {
        self.min = Some(min);
    }

    ///Returns the optional minimum value of the range.
    pub fn get_min(&self) -> Option<&String> {
        self.min.as_ref()
    }

    ///Sets the maximum value of the range.
    ///
    /// [max]: maximum value of the range
    pub fn set_max(&mut self, max: String) {
        self.max = Some(max);
    }

    ///Returns the optional maximum value of the range.
    pub fn get_max(&self) -> Option<&String> {
        self.max.as_ref()
    }
}

impl TSubmodelElement for Range {}

impl TReferable for Range {
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

impl THasExtensions for Range {
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

impl THasSemantics for Range {
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

impl TQualifiable for Range {
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

impl THasDataSpecification for Range {
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

impl TDataElement for Range {

}