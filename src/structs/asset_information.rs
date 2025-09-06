use serde::{Deserialize, Serialize};

use crate::enumerations::asset_kind::AssetKind;
use crate::structs::resource::Resource;
use crate::structs::specific_asset_id::SpecificAssetId;

///Struct for identifying metadata of the asset that is represented by an asset administration shell.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetInformation {
    ///Denotes whether the asset is of kind type, instance, role or none of these types.
    #[serde(rename = "assetKind")]
    asset_kind: AssetKind,
    ///Optional identifier of the asset, the asset administration shell is representing.
    #[serde(rename = "globalAssetId")]
    global_asset_id: Option<String>,
    ///Additional domain specific, typically proprietary identifiers for the asset like serial number,
    /// manufacturer part ID, customer part ID...
    #[serde(rename = "specificAssetIds")]
    specific_asset_ids: Vec<SpecificAssetId>,
    ///The type of asset.
    #[serde(rename = "assetType")]
    asset_type: Option<String>,
    ///Optional thumbnail of the asset represented by the asset administration shell.
    #[serde(rename = "defaultThumbnail")]
    default_thumbnail: Option<Resource>
}

impl AssetInformation {
    ///Creates a new instance of the struct.
    /// [asset_kind]: kind of the asset
    pub fn new(asset_kind: AssetKind) -> AssetInformation {
        AssetInformation {
            asset_kind,
            global_asset_id: None,
            specific_asset_ids: Vec::new(),
            asset_type: None,
            default_thumbnail: None
        }
    }

    ///Sets the kind of the asset.
    /// [asset_kind]: asset kind
    pub fn set_asset_kind(&mut self, asset_kind: AssetKind) {
        self.asset_kind = asset_kind;
    }

    ///Returns the kind of the asset.
    pub fn get_asset_kind(&self) -> &AssetKind {
        &self.asset_kind
    }

    ///Sets the identifier of the asset which is represented by the asset administration shell.
    /// [global_asset_id]: identifier of the asset
    pub fn set_global_asset_id(&mut self, global_asset_id: String) {
        self.global_asset_id = Some(global_asset_id);
    }

    ///Returns the identifier of the asset which is represented by the asset administration shell.
    pub fn get_global_asset_id(&self) -> Option<&String> {
        self.global_asset_id.as_ref()
    }

    ///Sets the additional domain specific, typically proprietary identifiers for the asset.
    /// [specific_asset_ids]: list of additional identifiers
    pub fn set_specific_asset_ids(&mut self, specific_asset_ids: Vec<SpecificAssetId>) {
        self.specific_asset_ids = specific_asset_ids;
    }

    ///Returns the additional domain specific, typically proprietary identifiers for the asset.
    pub fn get_specific_asset_ids(&self) -> &Vec<SpecificAssetId> {
        &self.specific_asset_ids
    }

    ///Adds an additional, domain specific, typically proprietary identifier for the asset.
    /// [specific_asset_id]: additional identifier
    pub fn add_specific_asset_id(&mut self, specific_asset_id: SpecificAssetId) {
        self.specific_asset_ids.push(specific_asset_id);
    }

    ///Removes an additional, domain specific, typically proprietary identifier for the asset.
    pub fn remove_specific_asset_id(&mut self, index: usize) {
        self.specific_asset_ids.remove(index);
    }

    ///Sets the type of the asset.
    /// [asset_type]: asset type
    pub fn set_asset_type(&mut self, asset_type: String) {
        self.asset_type = Some(asset_type);
    }

    ///Returns the type of the asset.
    pub fn get_asset_type(&self) -> Option<&String> {
        self.asset_type.as_ref()
    }

    ///Sets the thumbnail of the asset represented by the AAS.
    /// [thumbnail]: thumbnail
    pub fn set_default_thumbnail(&mut self, thumbnail: Resource) {
        self.default_thumbnail = Some(thumbnail);
    }

    ///Returns the optional thumbnail of the asset represented by the AAS.
    pub fn get_default_thumbnail(&self) -> Option<&Resource> {
        self.default_thumbnail.as_ref()
    }
}