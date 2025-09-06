use serde::{Deserialize, Serialize};

///Event direction.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Direction {
    ///Input direction.
    #[serde(rename = "input")]
    Input,
    ///Output direction.
    #[serde(rename = "output")]
    Output
}