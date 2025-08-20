use crate::structs::multi_language_name_type::MultiLanguageNameType;
use crate::structs::multi_language_text_type::MultiLanguageTextType;
use crate::traits::has_extensions::THasExtensions;

///Trait with functions for elements that are referable by its short ID.
pub trait TReferable: THasExtensions {
    ///Sets the category which is a value that gives further meta information.
    /// [category]: category of the element
    fn set_category(&mut self, category: String);
    ///Returns the category which is a value that gives further meta information.
    fn get_category(&self) -> Option<&String>;
    ///Sets the short ID of the element. In case of identifiables, this attribute is a short name of
    /// the element. In case of a referable, this ID is an identifying string of the element within
    /// its name space.
    /// [id_short]: short ID of the element
    fn set_id_short(&mut self, id_short: String);
    ///Returns the short ID of the element. In case of identifiables, this attribute is a short name
    /// of the element. In case of a referable, this ID is an identifying string of the element
    /// within its name space.
    fn get_id_short(&self) -> Option<&String>;
    ///Sets the display name of the element. Can be provided in several languages.
    /// [display_name]: display name of the element in several languages
    fn set_display_name(&mut self, display_name: MultiLanguageNameType);
    ///Returns the display name of the element. Can be provided in several languages.
    fn get_display_name(&self) -> Option<&MultiLanguageNameType>;
    ///Sets the description of the element. Can be provided in several languages.
    /// [description]: description of the element
    fn set_description(&mut self, description: MultiLanguageTextType);
    ///Returns the description of the element. Can be provided in several languages.
    fn get_description(&self) -> Option<&MultiLanguageTextType>;
}

