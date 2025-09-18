use crate::structs::qualifier::Qualifier;

///Trait with functions for a qualifiable element which may be further qualified by one or more
/// qualifiers.
pub trait TQualifiable {
    ///Sets additional qualifications of the qualifiable element.
    /// [qualifiers]: qualifications
    fn set_qualifiers(&mut self, qualifiers: Vec<Qualifier>);
    ///Returns additional qualifications of the qualifiable element.
    fn get_qualifiers(&self) -> &Vec<Qualifier>;
    ///Returns mutable additional qualifications of the qualifiable element.
    fn get_mut_qualifiers(&mut self) -> &mut Vec<Qualifier>;
    ///Adds an additional qualification to the qualifiable element.
    /// [qualifier]: qualification
    fn add_qualifier(&mut self, qualifier: Qualifier);
    ///Removes a qualification from the qualifiable element.
    /// [index]: index of the qualification to be removed
    fn remove_qualifier(&mut self, index: usize) -> Qualifier;
}