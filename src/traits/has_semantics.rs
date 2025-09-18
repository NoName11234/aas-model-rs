use crate::structs::reference::Reference;

///Trait with functions for an element that can have semantic definition plus some supplemental
/// semantic definitions.
pub trait THasSemantics {
    ///Sets the identifier of the semantic definition of the element called semantic ID or also
    /// main semantic ID of the element.
    /// [semantic_id]: identifier of the semantic definition
    fn set_semantic_id(&mut self, semantic_id: Reference);
    ///Returns the identifier of the semantic definition of the element called semantic ID or also
    /// main semantic ID of the element.
    fn get_semantic_id(&self) -> Option<&Reference>;
    ///Returns the mutable identifier of the semantic definition of the element called semantic ID
    /// or also main semantic ID of the element.
    fn get_mut_semantic_id(&mut self) -> Option<&mut Reference>;
    ///Sets the identifiers of supplemental semantic definitions of the element called supplemental
    /// semantic ID of the element.
    /// [supplemental_semantic_ids]: list of identifiers of supplemental semantic definitions
    fn set_supplemental_semantic_ids(&mut self, supplemental_semantic_ids: Vec<Reference>);
    ///Returns the identifiers of supplemental semantic definitions of the element called
    /// supplemental semantic ID of the element.
    fn get_supplemental_semantic_ids(&self) -> &Vec<Reference>;
    ///Returns the mutable identifiers of supplemental semantic definitions of the element called
    /// supplemental semantic ID of the element.
    fn get_mut_supplemental_semantic_ids(&mut self) -> &mut Vec<Reference>;
    ///Adds an identifier of a supplemental semantic definition of the element called supplemental
    /// semantic ID of the element.
    /// [supplemental_semantic_id]: identifier of a supplemental semantic definition
    fn add_supplemental_semantic_id(&mut self, supplemental_semantic_id: Reference);
    ///Removes an identifier of a supplemental semantic definition of the element called
    /// supplemental semantic ID of the element.
    /// [index]: index of the identifier to be removed
    fn remove_supplemental_semantic_id(&mut self, index: usize) -> Reference;
}