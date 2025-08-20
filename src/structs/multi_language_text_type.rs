#[derive(PartialEq, Clone)]
pub struct MultiLanguageTextType {
    language: String,
    text: String
}

impl MultiLanguageTextType {
    pub fn new(language: String, text: String) -> MultiLanguageTextType {
        MultiLanguageTextType {
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

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }
}