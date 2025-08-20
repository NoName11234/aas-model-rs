use crate::enumerations::key_type::KeyType;

#[derive(PartialEq, Clone)]
pub struct Key {
    key_type: KeyType,
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
    
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    
    pub fn get_value(&self) -> &String {
        &self.value
    }
}