use serde::{Deserialize, Serialize};

use crate::structs::administrative_information::AdministrativeInformation;
use crate::structs::asset_administration_shell::AssetAdministrationShell;
use crate::structs::concept_description::ConceptDescription;
use crate::structs::submodel::Submodel;
use crate::traits::identifiable::TIdentifiable;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "modelType")]
pub enum Identifiable {
    AssetAdministrationShell(AssetAdministrationShell),
    Submodel(Submodel),
    ConceptDescription(ConceptDescription)
}