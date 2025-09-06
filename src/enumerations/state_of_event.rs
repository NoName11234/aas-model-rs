use serde::{Deserialize, Serialize};

///State of an event.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum StateOfEvent {
    ///Event is on.
    #[serde(rename = "on")]
    On,
    ///Event is off.
    #[serde(rename = "off")]
    Off
}