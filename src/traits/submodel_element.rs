use crate::traits::has_data_specification::THasDataSpecification;
use crate::traits::has_semantics::THasSemantics;
use crate::traits::qualifiable::TQualifiable;
use crate::traits::referable::TReferable;

///A submodel element is an element suitable for the description and differentiation of assets.
pub trait TSubmodelElement: TReferable + THasSemantics + TQualifiable + THasDataSpecification {

}