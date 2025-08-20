use crate::enumerations::data_type_def_xsd::DataTypeDefXsd;
use crate::enumerations::qualifier_kind::QualifierKind;
use crate::structs::reference::Reference;
use crate::traits::has_semantics::THasSemantics;

///Struct representing a qualifier which is a type-value-pair. Depending on the kind of qualifier it
/// makes additional statements about its value, concept or existence and other meta information.
#[derive(PartialEq, Clone)]
pub struct Qualifier {
    ///The optional qualifier kind describes the kind of qualifier that is applied to the element.
    kind: Option<QualifierKind>,
    ///The qualifier type describes the type of qualifier that is applied to the element.
    qualifier_type: String,
    ///The data type of the qualifier value.
    value_type: DataTypeDefXsd,
    ///The optional qualifier value is the value of the qualifier.
    value: Option<String>,
    ///An optional reference to the global unique ID of a coded value.
    value_id: Option<Reference>,
    semantic_id: Option<Reference>,
    supplemental_semantic_ids: Vec<Reference>
}

impl Qualifier {
    ///Creates a new instance of the struct.
    /// [qualifier_type]: type of qualifier
    /// [value_type]: data type of the qualifier value
    pub fn new(qualifier_type: String, value_type: DataTypeDefXsd) -> Qualifier {
        Qualifier {
            kind: None,
            qualifier_type,
            value_type,
            value: None,
            value_id: None,
            semantic_id: None,
            supplemental_semantic_ids: Vec::new()
        }
    }

    ///Sets the kind of the qualifier that is applied to the element.
    /// [kind]: qualifier kind
    pub fn set_kind(&mut self, kind: QualifierKind) {
        self.kind = Some(kind);
    }

    ///Returns the kind of the qualifier that is applied to the element.
    pub fn get_kind(&self) -> Option<&QualifierKind> {
        self.kind.as_ref()
    }

    ///Sets the qualifier type describing the type applied to the element.
    /// [qualifier_type]: qualifier type
    pub fn set_qualifier_type(&mut self, qualifier_type: String) {
        self.qualifier_type = qualifier_type;
    }

    ///Returns the qualifier type describing the type applied to the element.
    pub fn get_qualifier_type(&self) -> &String {
        &self.qualifier_type
    }

    ///Sets the data type of the qualifier value.
    /// [value_type]: data type
    pub fn set_value_type(&mut self, value_type: DataTypeDefXsd) {
        self.value_type = value_type;
    }

    ///Returns the data type of the qualifier value.
    pub fn get_value_type(&self) -> &DataTypeDefXsd {
        &self.value_type
    }

    ///Sets the qualifier value.
    /// [value]: value of the qualifier
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    ///Returns the qualifier value.
    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    ///Sets the reference to the global unique ID of a coded value.
    /// [value_id]: reference to global unique ID
    pub fn set_value_id(&mut self, value_id: Reference) {
        self.value_id = Some(value_id);
    }

    ///Returns the reference to the global unique ID of a coded value.
    pub fn get_value_id(&self) -> Option<&Reference> {
        self.value_id.as_ref()
    }
}

impl THasSemantics for Qualifier {
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