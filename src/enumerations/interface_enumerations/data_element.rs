use serde::{Deserialize, Serialize};

use crate::structs::blob::Blob;
use crate::structs::file::File;
use crate::structs::multi_language_property::MultiLanguageProperty;
use crate::structs::property::Property;
use crate::structs::range::Range;
use crate::structs::reference_element::ReferenceElement;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "modelType")]
pub enum DataElement {
    Property(Property),
    MultiLanguageProperty(MultiLanguageProperty),
    Range(Range),
    Blob(Blob),
    File(File),
    ReferenceElement(ReferenceElement)
}

impl DataElement {

}