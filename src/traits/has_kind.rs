use crate::enumerations::modelling_kind::ModellingKind;

///Trait with functions for an element with a kind, that can either represent a template or an
/// instance.
pub trait THasKind {
    ///Sets the kind of the element.
    /// [kind]: element kind
    fn set_kind(&mut self, kind: ModellingKind);
    ///Returns the optional kind of the element.
    fn get_kind(&self) -> Option<&ModellingKind>;
}