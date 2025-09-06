use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum DataTypeDefXsd {
    #[serde(rename = "xs:anyURI")]
    AnyUri,
    #[serde(rename = "xs:base64Binary")]
    Base64Binary,
    #[serde(rename = "xs:boolean")]
    Boolean,
    #[serde(rename = "xs:byte")]
    Byte,
    #[serde(rename = "xs:date")]
    Date,
    #[serde(rename = "xs:dateTime")]
    DateTime,
    #[serde(rename = "xs:decimal")]
    Decimal,
    #[serde(rename = "xs:double")]
    Double,
    #[serde(rename = "xs:duration")]
    Duration,
    #[serde(rename = "xs:float")]
    Float,
    #[serde(rename = "xs:gDay")]
    GDay,
    #[serde(rename = "xs:gMonth")]
    GMonth,
    #[serde(rename = "xs:gMonthDay")]
    GMonthDay,
    #[serde(rename = "xs:gYear")]
    GYear,
    #[serde(rename = "xs:gYearMonth")]
    GYearMonth,
    #[serde(rename = "xs:hexBinary")]
    HexBinary,
    #[serde(rename = "xs:int")]
    Int,
    #[serde(rename = "xs:integer")]
    Integer,
    #[serde(rename = "xs:long")]
    Long,
    #[serde(rename = "xs:negativeInteger")]
    NegativeInteger,
    #[serde(rename = "xs:nonNegativeInteger")]
    NonNegativeInteger,
    #[serde(rename = "xs:nonPositiveInteger")]
    NonPositiveInteger,
    #[serde(rename = "xs:positiveInteger")]
    PositiveInteger,
    #[serde(rename = "xs:short")]
    Short,
    #[serde(rename = "xs:string")]
    String,
    #[serde(rename = "xs:time")]
    Time,
    #[serde(rename = "xs:unsignedByte")]
    UnsignedByte,
    #[serde(rename = "xs:unsignedInt")]
    UnsignedInt,
    #[serde(rename = "xs:unsignedLong")]
    UnsignedLong,
    #[serde(rename = "xs:unsignedShort")]
    UnsignedShort
}