use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct MultiLanguageNameType {
    language: String,
    text: String
}

impl MultiLanguageNameType {
    pub fn new(language: String, text: String) -> MultiLanguageNameType {
        MultiLanguageNameType {
            language,
            text
        }
    }
    
    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }
    
    pub fn get_language(&self) -> &String {
        &self.language
    }
    
    pub fn get_mut_language(&mut self) -> &mut String {
        &mut self.language
    }
    
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
    
    pub fn get_text(&self) -> &String {
        &self.text
    }
    
    pub fn get_mut_text(&mut self) -> &mut String {
        &mut self.text
    }
}