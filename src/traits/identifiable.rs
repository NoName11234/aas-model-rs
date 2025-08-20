use crate::structs::administrative_information::AdministrativeInformation;
use crate::traits::referable::TReferable;

///Trait with functions for an element that has a globally unique identifier.
pub trait TIdentifiable: TReferable {
    ///Sets the administrative information of an identifiable element.
    /// [administrative_information]: administrative information
    fn set_administration(&mut self, administrative_information: AdministrativeInformation);
    ///Returns the administrative information of an identifiable element.
    fn get_administration(&self) -> Option<&AdministrativeInformation>;
    ///Sets the globally unique identification of the element.
    /// [id]: globally unique identification
    fn set_id(&mut self, id: String);
    ///Returns the globally unique identification of the element.
    fn get_id(&self) -> &String;
}