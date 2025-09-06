use serde::{Deserialize, Serialize};

use crate::enumerations::interface_enumerations::submodel_element::SubmodelElement;
use crate::enumerations::modelling_kind::ModellingKind;
use crate::structs::administrative_information::AdministrativeInformation;
use crate::structs::extension::Extension;
use crate::structs::multi_language_name_type::MultiLanguageNameType;
use crate::structs::multi_language_text_type::MultiLanguageTextType;
use crate::structs::qualifier::Qualifier;
use crate::structs::reference::Reference;
use crate::traits::has_data_specification::THasDataSpecification;
use crate::traits::has_extensions::THasExtensions;
use crate::traits::has_kind::THasKind;
use crate::traits::has_semantics::THasSemantics;
use crate::traits::identifiable::TIdentifiable;
use crate::traits::qualifiable::TQualifiable;
use crate::traits::referable::TReferable;

///A submodel defines a specific aspect of the asset represented by the Asset Administration Shell.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Submodel {
    ///List of submodel elements a submodel consists of.
    #[serde(rename = "submodelElements")]
    submodel_elements: Vec<SubmodelElement>,
    #[serde(rename = "administration")]
    administrative_information: Option<AdministrativeInformation>,
    id: String,
    kind: Option<ModellingKind>,
    #[serde(rename = "semanticId")]
    semantic_id: Option<Reference>,
    #[serde(rename = "supplementalSemanticIds")]
    supplemental_semantic_ids: Vec<Reference>,
    qualifiers: Vec<Qualifier>,
    #[serde(rename = "embeddedDataSpecifications")]
    data_specifications: Vec<Reference>,
    category: Option<String>,
    #[serde(rename = "idShort")]
    id_short: Option<String>,
    #[serde(rename = "displayName")]
    display_name: Option<MultiLanguageNameType>,
    description: Option<MultiLanguageTextType>,
    extensions: Vec<Extension>
}

impl Submodel {
    ///Creates a new instance of the struct.
    /// [id]: global identifier of the submodel
    pub fn new(id: String) -> Submodel {
        Submodel {
            submodel_elements: Vec::new(),
            administrative_information: None,
            id,
            kind: None,
            semantic_id: None,
            supplemental_semantic_ids: Vec::new(),
            qualifiers: Vec::new(),
            data_specifications: Vec::new(),
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            extensions: Vec::new()
        }
    }

    ///Sets the list of submodel elements of the submodel.
    ///
    /// [submodel_elements]: list of submodel elements
    pub fn set_submodel_elements(&mut self, submodel_elements: Vec<SubmodelElement>) {
        self.submodel_elements = submodel_elements;
    }

    ///Returns the list of submodel elements of the submodel.
    pub fn get_submodel_elements(&self) -> &Vec<SubmodelElement> {
        &self.submodel_elements
    }

    ///Adds a submodel element to the submodel.
    ///
    /// [submodel_element]: submodel element
    pub fn add_submodel_element(&mut self, submodel_element: SubmodelElement) {
        self.submodel_elements.push(submodel_element);
    }

    ///Removes the specified submodel element from the submodel.
    pub fn remove_submodel_element(&mut self, index: usize) -> SubmodelElement {
        self.submodel_elements.remove(index)
    }
}

impl TReferable for Submodel {
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

impl THasExtensions for Submodel {
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

impl TIdentifiable for Submodel {
    fn set_administration(&mut self, administrative_information: AdministrativeInformation) {
        self.administrative_information = Some(administrative_information);
    }

    fn get_administration(&self) -> Option<&AdministrativeInformation> {
        self.administrative_information.as_ref()
    }

    fn set_id(&mut self, id: String) {
        self.id = id;
    }

    fn get_id(&self) -> &String {
        &self.id
    }
}

impl THasKind for Submodel {
    fn set_kind(&mut self, kind: ModellingKind) {
        self.kind = Some(kind);
    }

    fn get_kind(&self) -> Option<&ModellingKind> {
        self.kind.as_ref()
    }
}

impl THasSemantics for Submodel {
    fn set_semantic_id(&mut self, semantic_id: Reference) {
        self.semantic_id = Some(semantic_id);
    }

    fn get_semantic_id(&self) -> Option<&Reference> {
        self.semantic_id.as_ref()
    }

    fn set_supplemental_semantic_ids(&mut self, semantic_ids: Vec<Reference>) {
        self.supplemental_semantic_ids = semantic_ids;
    }

    fn get_supplemental_semantic_ids(&self) -> &Vec<Reference> {
        &self.supplemental_semantic_ids
    }

    fn add_supplemental_semantic_id(&mut self, semantic_id: Reference) {
        self.supplemental_semantic_ids.push(semantic_id);
    }

    fn remove_supplemental_semantic_id(&mut self, index: usize) -> Reference {
        self.supplemental_semantic_ids.remove(index)
    }
}

impl TQualifiable for Submodel {
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

impl THasDataSpecification for Submodel {
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