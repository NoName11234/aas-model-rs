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

impl Identifiable {
    pub fn set_administration(&mut self, administrative_information: AdministrativeInformation) {
        match self {
            Identifiable::AssetAdministrationShell(elem) => elem.set_administration(administrative_information),
            Identifiable::Submodel(elem) => elem.set_administration(administrative_information),
            Identifiable::ConceptDescription(elem) => elem.set_administration(administrative_information)
        }
    }

    pub fn get_administration(&self) -> Option<&AdministrativeInformation> {
        match self {
            Identifiable::AssetAdministrationShell(elem) => elem.get_administration(),
            Identifiable::Submodel(elem) => elem.get_administration(),
            Identifiable::ConceptDescription(elem) => elem.get_administration()
        }
    }

    pub fn set_id(&mut self, id: String) {
        match self {
            Identifiable::AssetAdministrationShell(elem) => elem.set_id(id),
            Identifiable::Submodel(elem) => elem.set_id(id),
            Identifiable::ConceptDescription(elem) => elem.set_id(id)
        }
    }

    pub fn get_id(&self) -> &String {
        match self {
            Identifiable::AssetAdministrationShell(elem) => elem.get_id(),
            Identifiable::Submodel(elem) => elem.get_id(),
            Identifiable::ConceptDescription(elem) => elem.get_id()
        }
    }
}