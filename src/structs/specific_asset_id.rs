use crate::structs::reference::Reference;
use crate::traits::has_semantics::THasSemantics;

///A struct for a specific asset ID which describes a generic supplementary identifying attribute
/// of the asset.
#[derive(PartialEq, Clone)]
pub struct SpecificAssetId {
    ///The name of the asset identifier.
    name: String,
    ///The value of the specific asset identifier with the corresponding name.
    value: String,
    ///The unique ID of the (external) subject the specific asset ID value belongs to or has meaning
    /// to.
    external_subject_id: Option<Reference>,
    semantic_id: Option<Reference>,
    supplemental_semantic_ids: Vec<Reference>
}

impl SpecificAssetId {
    ///Creates a new instance of the struct.
    /// [name]: name of the asset identifier
    /// [value]: value of the specific asset identifier
    pub fn new(name: String, value: String) -> SpecificAssetId {
        SpecificAssetId {
            name,
            value,
            external_subject_id: None,
            semantic_id: None,
            supplemental_semantic_ids: Vec::new()
        }
    }

    /// Sets the name of the asset identifier.
    /// [name]: name of the asset identifier
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    ///Returns the name of the asset identifier.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    ///Sets the value of the specific asset identifier with the corresponding name.
    /// [value]: value of the specific asset identifier
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    ///Returns the value of the specific asset identifier with the corresponding name.
    pub fn get_value(&self) -> &String {
        &self.value
    }

    ///Sets the unique ID of the (external) subject the specific asset ID value belongs to or has
    /// meaning to.
    pub fn set_external_subject_id(&mut self, external_subject_id: Reference) {
        self.external_subject_id = Some(external_subject_id);
    }

    ///Returns the optional unique ID of the (external) subject the specific asset ID value belongs
    /// to or has meaning to.
    pub fn get_external_subject_id(&self) -> Option<&Reference> {
        self.external_subject_id.as_ref()
    }
}

impl THasSemantics for SpecificAssetId {
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