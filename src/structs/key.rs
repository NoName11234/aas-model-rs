use serde::{Deserialize, Serialize};

use crate::enumerations::key_type::KeyType;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Key {
    #[serde(rename = "type")]
    key_type: KeyType,
    #[serde(rename = "value")]
    value: String
}

impl Key {
    pub fn new(key_type: KeyType, value: String) -> Key {
        Key {
            key_type,
            value
        }
    }
    
    pub fn set_key_type(&mut self, key_type: KeyType) {
        self.key_type = key_type;
    }
    
    pub fn get_key_type(&self) -> &KeyType {
        &self.key_type
    }
    
    pub fn get_mut_key_type(&mut self) -> &mut KeyType {
        &mut self.key_type
    }
    
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    
    pub fn get_value(&self) -> &String {
        &self.value
    }
    
    pub fn get_mut_value(&mut self) -> &mut String {
        &mut self.value
    }
}