use serde::{Deserialize, Serialize};

use crate::structs::reference::Reference;
use crate::traits::has_data_specification::THasDataSpecification;

///Administrative information for an element like version information.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct AdministrativeInformation {
    ///The optional version of the element.
    version: Option<String>,
    ///The optional revision of the element.
    revision: Option<String>,
    ///The optional subject ID of the subject responsible for making the element.
    creator: Option<Reference>,
    ///The optional identifier of the template that guided the creation of the element.
    #[serde(rename = "templateId")]
    template_id: Option<String>,
    #[serde(rename = "embeddedDataSpecifications")]
    data_specifications: Vec<Reference>,
}

impl AdministrativeInformation {
    ///Creates a new instance of the struct.
    pub fn new() -> AdministrativeInformation {
        AdministrativeInformation {
            version: None,
            revision: None,
            creator: None,
            template_id: None,
            data_specifications: Vec::new(),
        }
    }

    ///Sets the version of the element.
    /// [version]: version of the element
    pub fn set_version(&mut self, version: String) {
        self.version = Some(version);
    }

    ///Returns the optional version of the element.
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    ///Returns the optional mutable version of the element.
    pub fn get_mut_version(&mut self) -> Option<&mut String> {
        self.version.as_mut()
    }

    ///Sets the revision of the element.
    /// [revision]: revision of the element
    pub fn set_revision(&mut self, revision: String) {
        self.revision = Some(revision);
    }

    ///Returns the optional revision of the element.
    pub fn get_revision(&self) -> Option<&String> {
        self.revision.as_ref()
    }

    ///Returns the optional mutable revision of the element.
    pub fn get_mut_revision(&mut self) -> Option<&mut String> {
        self.revision.as_mut()
    }

    ///Sets the creator subject ID of the subject responsible for making the element.
    /// [creator]: creator subject ID
    pub fn set_creator(&mut self, creator: Reference) {
        self.creator = Some(creator);
    }

    ///Returns the optional creator subject ID of the subject responsible for making the element.
    pub fn get_creator(&self) -> Option<&Reference> {
        self.creator.as_ref()
    }

    ///Returns the optional mutable creator subject ID of the subject responsible for making the
    /// element.
    pub fn get_mut_creator(&mut self) -> Option<&mut Reference> {
        self.creator.as_mut()
    }

    ///Sets the identifier of the template that guided the creation of the element.
    /// [template_id]: identifier of the template
    pub fn set_template_id(&mut self, template_id: String) {
        self.template_id = Some(template_id);
    }

    ///Returns the identifier of the template that guided the creation of the element.
    pub fn get_template_id(&self) -> Option<&String> {
        self.template_id.as_ref()
    }

    ///Returns the mutable identifier of the template that guided the creation of the element.
    pub fn get_mut_template_id(&mut self) -> Option<&mut String> {
        self.template_id.as_mut()
    }
}

impl THasDataSpecification for AdministrativeInformation {
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