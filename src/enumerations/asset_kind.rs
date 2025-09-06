use serde::{Deserialize, Serialize};

///Enumeration for denoting whether an asset is a type asset or an instance asset or is a role or
/// whether this kind of classification is not applicable.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetKind {
    ///type asset
    Type,
    ///instance asset
    Instance,
    ///role asset
    Role,
    ///neither a type asset nor an instance asset nor a role asset
    NotApplicable
}