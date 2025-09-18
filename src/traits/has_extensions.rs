use crate::structs::extension::Extension;

///Trait with functions for an element that can be extended by proprietary extensions.
pub trait THasExtensions {
    ///Returns a list of extensions of the element.
    fn get_extensions(&self) -> &Vec<Extension>;
    ///Returns a mutable list of extensions of the element.
    fn get_mut_extensions(&mut self) -> &mut Vec<Extension>;
    ///Sets a list of extensions of the element.
    /// [extensions]: list of extensions
    fn set_extensions(&mut self, extensions: Vec<Extension>);
    ///Adds an extension to the element.
    /// [extension]: extension
    fn add_extension(&mut self, extension: Extension);
    ///Removes an extension from the element.
    /// [index]: index of the extension to be removed
    fn remove_extension(&mut self, index: usize) -> Extension;
}