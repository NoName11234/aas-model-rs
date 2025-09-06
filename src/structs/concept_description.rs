use serde::{Deserialize, Serialize};

use crate::structs::administrative_information::AdministrativeInformation;
use crate::structs::extension::Extension;
use crate::structs::multi_language_name_type::MultiLanguageNameType;
use crate::structs::multi_language_text_type::MultiLanguageTextType;
use crate::structs::reference::Reference;
use crate::traits::has_data_specification::THasDataSpecification;
use crate::traits::has_extensions::THasExtensions;
use crate::traits::identifiable::TIdentifiable;
use crate::traits::referable::TReferable;

///The semantics of a property or other elements that may have a semantic description is defined by
/// a concept description.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct ConceptDescription {
    ///List of references to external definitions the concept is compatible to or was derived from.
    #[serde(rename = "isCaseOf")]
    is_case_of: Vec<Reference>,
    #[serde(rename = "embeddedDataSpecifications")]
    data_specifications: Vec<Reference>,
    category: Option<String>,
    #[serde(rename = "idShort")]
    id_short: Option<String>,
    #[serde(rename = "displayName")]
    display_name: Option<MultiLanguageNameType>,
    description: Option<MultiLanguageTextType>,
    extensions: Vec<Extension>,
    #[serde(rename = "administration")]
    administrative_information: Option<AdministrativeInformation>,
    id: String
}

impl ConceptDescription {
    ///Creates a new instance of the struct.
    ///
    /// [id]: global identifier of the concept description
    pub fn new(id: String) -> ConceptDescription {
        ConceptDescription {
            is_case_of: Vec::new(),
            data_specifications: Vec::new(),
            category: None, id_short: None,
            display_name: None,
            description: None,
            extensions: Vec::new(),
            administrative_information: None,
            id
        }
    }

    ///Sets the list of references to external definitions the concept is compatible to or was
    /// derived from.
    ///
    /// [is_case_of]: list of references
    pub fn set_is_case_ofs(&mut self, is_case_of: Vec<Reference>) {
        self.is_case_of = is_case_of;
    }

    ///Returns the list of references to external definitions the concept is compatible to or was
    /// derived from.
    pub fn get_is_case_ofs(&self) -> &Vec<Reference> {
        &self.is_case_of
    }

    ///Adds a reference to an external definition the concept is compatible to or was derived from.
    ///
    /// [is_case_of]: reference to external definition
    pub fn add_is_case_of(&mut self, is_case_of: Reference) {
        self.is_case_of.push(is_case_of);
    }

    ///Removes a reference to an external definition the concept is compatible to or was derived
    /// from.
    pub fn remove_is_case_of(&mut self, index: usize) -> Reference {
        self.is_case_of.remove(index)
    }
}

impl THasDataSpecification for ConceptDescription {
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

impl TReferable for ConceptDescription {
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

impl THasExtensions for ConceptDescription {
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

impl TIdentifiable for ConceptDescription {
    fn set_administration(&mut self, administrative_information: AdministrativeInformation) {
        self.administrative_information = Some(administrative_information);
    }

    fn get_administration(&self) -> Option<&AdministrativeInformation> {
        self.administrative_information.as_ref()
    }

    fn set_id(&mut self, id: String) {
        self.id_short = Some(id);
    }

    fn get_id(&self) -> &String {
        &self.id
    }
}