use serde::{Deserialize, Serialize};

use crate::structs::administrative_information::AdministrativeInformation;
use crate::structs::asset_information::AssetInformation;
use crate::structs::extension::Extension;
use crate::structs::multi_language_name_type::MultiLanguageNameType;
use crate::structs::multi_language_text_type::MultiLanguageTextType;
use crate::structs::reference::Reference;
use crate::traits::has_data_specification::THasDataSpecification;
use crate::traits::has_extensions::THasExtensions;
use crate::traits::identifiable::TIdentifiable;
use crate::traits::referable::TReferable;

///A struct representing an asset administration shell.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetAdministrationShell {
    ///Optional reference to the asset administration shell which the asset administration shell
    /// was derived from.
    derived_from: Option<Reference>,
    ///Meta information about the asset, the asset administration shell is representing.
    #[serde(rename = "assetInformation")]
    asset_information: AssetInformation,
    ///List of references to submodels of the asset administration shell.
    submodels: Vec<Reference>,
    category: Option<String>,
    #[serde(rename = "idShort")]
    id_short: Option<String>,
    #[serde(rename = "displayName")]
    display_name: Vec<MultiLanguageNameType>,
    description: Vec<MultiLanguageTextType>,
    extensions: Vec<Extension>,
    #[serde(rename = "administration")]
    administrative_information: Option<AdministrativeInformation>,
    id: String,
    #[serde(rename = "embeddedDataSpecifications")]
    data_specifications: Vec<Reference>
}

impl AssetAdministrationShell {
    ///Creates a new instance of the struct.
    /// [id]: global identifier of the asset administration shell
    /// [asset_information]: meta information about the asset
    pub fn new(id: String, asset_information: AssetInformation) -> AssetAdministrationShell {
        AssetAdministrationShell {
            derived_from: None,
            asset_information,
            submodels: Vec::new(),
            category: None,
            id_short: None,
            display_name: Vec::new(),
            description: Vec::new(),
            extensions: Vec::new(),
            administrative_information: None,
            id,
            data_specifications: Vec::new(),
        }
    }

    ///Sets the reference to the asset administration shell which the asset administration shell
    /// was derived from.
    /// [derived_from]: reference to AAS
    pub fn set_derived_from(&mut self, derived_from: Reference) {
        self.derived_from = Some(derived_from);
    }

    ///Returns the reference to the asset administration shell which the asset administration shell
    /// was derived from.
    pub fn get_derived_from(&self) -> Option<&Reference> {
        self.derived_from.as_ref()
    }
    
    ///Returns the mutable reference to the asset administration shell which the asset
    /// administration shell was derived from.
    pub fn get_mut_derived_from(&mut self) -> Option<&mut Reference> {
        self.derived_from.as_mut()
    }

    ///Sets the meta information about the asset the AAS is representing.
    /// [asset_information]: meta information about the asset
    pub fn set_asset_information(&mut self, asset_information: AssetInformation) {
        self.asset_information = asset_information;
    }

    ///Returns the meta information about the asset the AAS is representing.
    pub fn get_asset_information(&self) -> &AssetInformation {
        &self.asset_information
    }

    ///Returns the mutable meta information about the asset the AAS is representing.
    pub fn get_mut_asset_information(&mut self) -> &mut AssetInformation {
        &mut self.asset_information
    }

    ///Sets the references to submodels of the AAS.
    /// [submodels]: references to submodels
    pub fn set_submodels(&mut self, submodels: Vec<Reference>) {
        self.submodels = submodels;
    }

    ///Returns the references to submodels of the AAS.
    pub fn get_submodels(&self) -> &Vec<Reference> {
        &self.submodels
    }

    ///Returns the mutable references to submodels of the AAS.
    pub fn get_mut_submodels(&mut self) -> &mut Vec<Reference> {
        &mut self.submodels
    }

    ///Adds a reference to a submodel of the AAS.
    /// [submodel]: reference to a submodel
    pub fn add_submodel(&mut self, submodel: Reference) {
        self.submodels.push(submodel);
    }

    ///Removes a reference to a submodel of the AAS.
    pub fn remove_submodel(&mut self, index: usize) -> Reference {
        self.submodels.remove(index)
    }
}

impl TReferable for AssetAdministrationShell {
    fn set_category(&mut self, category: String) {
        self.category = Some(category);
    }

    fn get_category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    fn get_mut_category(&mut self) -> Option<&mut String> {
        self.category.as_mut()
    }

    fn set_id_short(&mut self, id_short: String) {
        self.id_short = Some(id_short);
    }

    fn get_id_short(&self) -> Option<&String> {
        self.id_short.as_ref()
    }

    fn get_mut_id_short(&mut self) -> Option<&mut String> {
        self.id_short.as_mut()
    }

    fn set_display_name(&mut self, display_name: Vec<MultiLanguageNameType>) {
        self.display_name = display_name;
    }

    fn get_display_name(&self) -> &Vec<MultiLanguageNameType> {
        &self.display_name
    }

    fn get_mut_display_name(&mut self) -> &mut Vec<MultiLanguageNameType> {
        &mut self.display_name
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

    fn get_mut_description(&mut self) -> &mut Vec<MultiLanguageTextType> {
        &mut self.description
    }

    fn add_description(&mut self, description: MultiLanguageTextType) {
        self.description.push(description);
    }

    fn remove_description(&mut self, index: usize) -> MultiLanguageTextType {
        self.description.remove(index)
    }
}

impl THasExtensions for AssetAdministrationShell {
    fn get_extensions(&self) -> &Vec<Extension> {
        &self.extensions
    }

    fn get_mut_extensions(&mut self) -> &mut Vec<Extension> {
        &mut self.extensions
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

impl TIdentifiable for AssetAdministrationShell {
    fn set_administration(&mut self, administrative_information: AdministrativeInformation) {
        self.administrative_information = Some(administrative_information);
    }

    fn get_administration(&self) -> Option<&AdministrativeInformation> {
        self.administrative_information.as_ref()
    }

    fn get_mut_administration(&mut self) -> Option<&mut AdministrativeInformation> {
        self.administrative_information.as_mut()
    }

    fn set_id(&mut self, id: String) {
        self.id = id;
    }

    fn get_id(&self) -> &String {
        &self.id
    }
    
    fn get_mut_id(&mut self) -> &mut String {
        &mut self.id
    }
}

impl THasDataSpecification for AssetAdministrationShell {
    fn get_data_specifications(&self) -> &Vec<Reference> {
        &self.data_specifications
    }
    
    fn get_mut_data_specifications(&mut self) -> &mut Vec<Reference> {
        &mut self.data_specifications
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