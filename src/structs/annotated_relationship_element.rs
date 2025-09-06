use serde::{de, Deserialize, Serialize};

use crate::enumerations::interface_enumerations::data_element::DataElement;
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
use crate::traits::relationship_element::TRelationshipElement;
use crate::traits::submodel_element::TSubmodelElement;

///An annotated relationship element is a relationship element that can be annotated with additional
/// data elements.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct AnnotatedRelationshipElement {
    ///Data elements that represent annotations that holds for the relationship between the two
    /// elements.
    annotations: Vec<DataElement>,
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
    data_specifications: Vec<Reference>,
    first: Option<Reference>,
    second: Option<Reference>,
}


impl AnnotatedRelationshipElement {
    ///Creates a new instance of the struct.
    pub fn new() -> AnnotatedRelationshipElement {
        AnnotatedRelationshipElement {
            annotations: Vec::new(),
            category: None,
            id_short: None,
            display_name: Vec::new(),
            description: Vec::new(),
            extensions: Vec::new(),
            semantic_id: None,
            supplemental_semantic_ids: Vec::new(),
            qualifiers: Vec::new(),
            data_specifications: Vec::new(),
            first: None,
            second: None,
        }
    }

    ///Sets the list of data elements representing annotations.
    ///
    /// [annotations]: list of annotations
    pub fn set_annotations(&mut self, annotations: Vec<DataElement>) {
        self.annotations = annotations;
    }

    ///Returns the list of data elements representing annotations.
    pub fn get_data_elements(&self) -> &Vec<DataElement> {
        &self.annotations
    }

    ///Adds a data element representing an annotation.
    /// [annotation]: annotation
    pub fn add_data_element(&mut self, annotation: DataElement) {
        self.annotations.push(annotation);
    }

    ///Removes the specified data element.
    pub fn remove_data_element(&mut self, index: usize) -> DataElement {
        self.annotations.remove(index)
    }
}

impl TSubmodelElement for AnnotatedRelationshipElement {}

impl TReferable for AnnotatedRelationshipElement {
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

impl THasExtensions for AnnotatedRelationshipElement {
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

impl THasSemantics for AnnotatedRelationshipElement {
    fn set_semantic_id(&mut self, supplemental_semantic_id: Reference) {
        self.semantic_id = Some(supplemental_semantic_id);
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

    fn add_supplemental_semantic_id(&mut self, semantic_id: Reference) {
        self.supplemental_semantic_ids.push(semantic_id);
    }

    fn remove_supplemental_semantic_id(&mut self, index: usize) -> Reference {
        self.supplemental_semantic_ids.remove(index)
    }
}

impl TQualifiable for AnnotatedRelationshipElement {
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

impl THasDataSpecification for AnnotatedRelationshipElement {
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

impl TRelationshipElement for AnnotatedRelationshipElement {
    fn set_first(&mut self, first: Reference) {
        self.first = Some(first);
    }

    fn get_first(&self) -> Option<&Reference> {
        self.first.as_ref()
    }

    fn set_second(&mut self, second: Reference) {
        self.second = Some(second);
    }

    fn get_second(&self) -> Option<&Reference> {
        self.second.as_ref()
    }
}
