use serde::{Deserialize, Serialize};

use crate::enumerations::entity_type::EntityType;
use crate::enumerations::interface_enumerations::submodel_element::SubmodelElement;
use crate::structs::extension::Extension;
use crate::structs::multi_language_name_type::MultiLanguageNameType;
use crate::structs::multi_language_text_type::MultiLanguageTextType;
use crate::structs::qualifier::Qualifier;
use crate::structs::reference::Reference;
use crate::structs::specific_asset_id::SpecificAssetId;
use crate::traits::has_data_specification::THasDataSpecification;
use crate::traits::has_extensions::THasExtensions;
use crate::traits::has_semantics::THasSemantics;
use crate::traits::qualifiable::TQualifiable;
use crate::traits::referable::TReferable;
use crate::traits::submodel_element::TSubmodelElement;

///An entity is a submodel element that is used to model entities.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Entity {
    ///Optional statements applicable to the entity, each statement described by submodel element -
    /// typically with a qualified value
    statements: Vec<SubmodelElement>,
    ///Describes whether the entity is a co-managed entity or a self-managed entity. Optional.
    #[serde(rename = "entityType")]
    entity_type: Option<EntityType>,
    ///Optional global identifier of the asset the entity is representing.
    #[serde(rename = "globalAssetId")]
    global_asset_id: Option<String>,
    ///Optional references to a specific asset ID representing a supplementary identifier of the asset
    /// represented by the Asset Administration Shell.
    #[serde(rename = "specificAssetIds")]
    specific_asset_ids: Vec<SpecificAssetId>,
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

impl Entity {
    ///Creates a new instance of the struct.
    pub fn new() -> Entity {
        Entity {
            statements: Vec::new(),
            entity_type: None,
            global_asset_id: None,
            specific_asset_ids: Vec::new(),
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

    ///Sets the list of statements applicable to the entity.
    ///
    /// [statements]: list of statements
    pub fn set_statements(&mut self, statements: Vec<SubmodelElement>) {
        self.statements = statements;
    }

    ///Returns list of statements applicable to the entity.
    pub fn get_statements(&self) -> &Vec<SubmodelElement> {
        &self.statements
    }

    ///Adds a statement applicable to the entity.
    ///
    /// [statement]: statement
    pub fn add_statement(&mut self, statement: SubmodelElement) {
        self.statements.push(statement);
    }

    ///Removes a statement applicable to the entity.
    pub fn remove_statement(&mut self, index: usize) -> SubmodelElement {
        self.statements.remove(index)
    }

    ///Sets the type of the entity.
    ///
    /// [entity_type]: type of entity
    pub fn set_entity_type(&mut self, entity_type: EntityType) {
        self.entity_type = Some(entity_type);
    }

    ///Returns the type of the entity.
    pub fn get_entity_type(&self) -> Option<&EntityType> {
        self.entity_type.as_ref()
    }

    ///Sets the global identifier of the asset the entity is representing.
    ///
    /// [global_asset_id]: global identifier
    pub fn set_global_asset_id(&mut self, global_asset_id: String) {
        self.global_asset_id = Some(global_asset_id);
    }

    ///Returns the global identifier of the asset the entity is representing.
    pub fn get_global_asset_id(&self) -> Option<&String> {
        self.global_asset_id.as_ref()
    }

    ///Sets the list of specific asset IDs representing a supplementary identifier.
    ///
    /// [specific_asset_ids]: specific asset IDs
    pub fn set_specific_asset_ids(&mut self, specific_asset_ids: Vec<SpecificAssetId>) {
        self.specific_asset_ids = specific_asset_ids;
    }

    ///Returns the list of specific asset IDs representing a supplementary identifier.
    pub fn get_specific_asset_ids(&self) -> &Vec<SpecificAssetId> {
        &self.specific_asset_ids
    }

    ///Adds a specific asset ID of a supplementary identifier.
    ///
    /// [specific_asset_id]: specific asset ID
    pub fn add_specific_asset_id(&mut self, specific_asset_id: SpecificAssetId) {
        self.specific_asset_ids.push(specific_asset_id);
    }

    ///Removes a specific asset ID of a supplementary identifier.
    pub fn remove_specific_asset_id(&mut self, index: usize) -> SpecificAssetId {
        self.specific_asset_ids.remove(index)
    }
}

impl TReferable for Entity {
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

impl THasExtensions for Entity {
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

impl THasSemantics for Entity {
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

impl TQualifiable for Entity {
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

impl THasDataSpecification for Entity {
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

impl TSubmodelElement for Entity {

}

