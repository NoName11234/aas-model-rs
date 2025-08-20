use crate::structs::asset_administration_shell::AssetAdministrationShell;
use crate::structs::concept_description::ConceptDescription;
use crate::structs::submodel::Submodel;

///Container for the sets of different identifiables.
#[derive(PartialEq, Clone)]
pub struct Environment {
    ///Asset administration shells.
    asset_administration_shells: Vec<AssetAdministrationShell>,
    ///Submodels.
    submodels: Vec<Submodel>,
    ///Concept descriptions.
    concept_descriptions: Vec<ConceptDescription>
}

impl Environment {
    ///Creates a new instance of the struct.
    pub fn new() -> Environment {
        Environment {
            asset_administration_shells: Vec::new(),
            submodels: Vec::new(),
            concept_descriptions: Vec::new()
        }
    }

    ///Sets the list of asset administration shells.
    ///
    /// [asset_administration_shells]: list of AAS
    pub fn set_asset_administration_shells(&mut self, asset_administration_shells: Vec<AssetAdministrationShell>) {
        self.asset_administration_shells = asset_administration_shells;
    }

    ///Returns the list of asset administration shells.
    pub fn get_asset_administration_shells(&self) -> &Vec<AssetAdministrationShell> {
        &self.asset_administration_shells
    }

    ///Adds an asset administration shell to the list.
    ///
    /// [aas]: asset administration shell
    pub fn add_asset_administration_shell(&mut self, aas: AssetAdministrationShell) {
        self.asset_administration_shells.push(aas);
    }

    ///Removes an asset administration shell from the list.
    pub fn remove_asset_administration_shell(&mut self, index: usize) -> AssetAdministrationShell {
        self.asset_administration_shells.remove(index)
    }

    ///Sets the list of submodels.
    ///
    /// [submodels]: list of submodels
    pub fn set_submodels(&mut self, submodels: Vec<Submodel>) {
        self.submodels = submodels;
    }

    ///Returns the list of submodels.
    pub fn get_submodels(&self) -> &Vec<Submodel> {
        &self.submodels
    }

    ///Adds a submodel to the list.
    ///
    /// [submodel]: submodel to add
    pub fn add_submodel(&mut self, submodel: Submodel) {
        self.submodels.push(submodel);
    }

    ///Removes a submodel from the list.
    pub fn remove_submodel(&mut self, index: usize) -> Submodel {
        self.submodels.remove(index)
    }

    ///Sets the list of concept descriptions.
    ///
    /// [concept_descriptions]: concept descriptions
    pub fn set_concept_descriptions(&mut self, concept_descriptions: Vec<ConceptDescription>) {
        self.concept_descriptions = concept_descriptions;
    }

    ///Returns the list of concept descriptions.
    pub fn get_concept_descriptions(&self) -> &Vec<ConceptDescription> {
        &self.concept_descriptions
    }

    ///Adds a concept description to the list.
    /// [concept_description]: concept description
    pub fn add_concept_description(&mut self, concept_description: ConceptDescription) {
        self.concept_descriptions.push(concept_description);
    }

    ///Removes a concept description from the list.
    pub fn remove_concept_description(&mut self, index: usize) -> ConceptDescription {
        self.concept_descriptions.remove(index)
    }
}