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

///A property is a data element that has a single value.
#[derive(PartialEq, Clone)]
pub struct Property {
    ///Data type of the value attribute.
    value_type: DataTypeDefXsd,
    ///The optional value of the property instance.
    value: Option<String>,
    ///The optional reference to the global unique ID of a coded value.
    value_id: Option<Reference>,
    category: Option<String>,
    id_short: Option<String>,
    display_name: Option<MultiLanguageNameType>,
    description: Option<MultiLanguageTextType>,
    extensions: Vec<Extension>,
    semantic_id: Option<Reference>,
    supplemental_semantic_ids: Vec<Reference>,
    qualifiers: Vec<Qualifier>,
    data_specifications: Vec<Reference>
}

impl Property {
    ///Creates a new instance of a struct of a property data element.
    /// 
    /// [value_type]: data type of the value attribute
    pub fn new (value_type: DataTypeDefXsd) -> Property {
        Property {
            value_type,
            value: None,
            value_id: None,
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

    ///Sets the data type of the value attribute.
    ///
    /// [value_type]: data type of the value attribute
    pub fn set_value_type(&mut self, value_type: DataTypeDefXsd) {
        self.value_type = value_type;
    }

    ///Returns the data type of the value attribute.
    pub fn get_value_type(&self) -> &DataTypeDefXsd {
        &self.value_type
    }

    ///Sets the value of the property instance.
    ///
    /// [value]: value of the property instance
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    ///Returns the optional value of the property instance.
    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
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

impl TSubmodelElement for Property {}

impl TReferable for Property {
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

impl THasExtensions for Property {
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

impl THasSemantics for Property {
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

impl TQualifiable for Property {
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

impl THasDataSpecification for Property {
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

impl TDataElement for Property {

}