use serde::{Deserialize, Serialize};

use crate::enumerations::reference_type::ReferenceType;
use crate::structs::key::Key;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "type")]
    reference_type: ReferenceType,
    #[serde(rename = "referredSemanticId")]
    referred_semantic_id: Option<Box<Reference>>,
    keys: Vec<Key>
}

impl Reference {
    pub fn new(reference_type: ReferenceType, keys: Vec<Key>) -> Reference {
        Reference {
            reference_type,
            referred_semantic_id: None,
            keys
        }
    }

    pub fn set_reference_type(&mut self, reference_type: ReferenceType) {
        self.reference_type = reference_type;
    }

    pub fn get_reference_type(&self) -> &ReferenceType {
        &self.reference_type
    }
    
    pub fn get_mut_reference_type(&mut self) -> &mut ReferenceType {
        &mut self.reference_type
    }

    pub fn set_referred_semantic_id(&mut self, referred_semantic_id: Box<Reference>) {
        self.referred_semantic_id = Some(referred_semantic_id);
    }

    pub fn set_keys(&mut self, keys: Vec<Key>) {
        self.keys = keys;
    }

    pub fn get_keys(&self) -> &Vec<Key> {
        &self.keys
    }
    
    pub fn get_mut_keys(&mut self) -> &mut Vec<Key> {
        &mut self.keys
    }

    pub fn add_key(&mut self, key: Key) {
        self.keys.push(key);
    }

    pub fn remove_key(&mut self, index: usize) -> Key {
        self.keys.remove(index)
    }
}