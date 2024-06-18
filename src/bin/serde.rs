use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};
use base64::{encode, decode};
use std::fmt;

#[derive(Debug, PartialEq)]
struct Data {
    bytes: Vec<u8>,
}

// Custom serialization
impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = encode(&self.bytes);
        serializer.serialize_str(&encoded)
    }
}

// Custom deserialization
impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DataVisitor)
    }
}

// Visitor for deserialization
struct DataVisitor;

impl<'de> Visitor<'de> for DataVisitor {
    type Value = Data;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a base64 encoded string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decoded = decode(value).map_err(de::Error::custom)?;
        Ok(Data { bytes: decoded })
    }
}

fn main() {
    let data = Data { bytes: vec![104, 101, 108, 108, 111] }; // "hello" in ASCII

    // Serialize the Data struct to JSON
    let serialized = serde_json::to_string(&data).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize the JSON back to a Data struct
    let deserialized: Data = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);

    assert_eq!(data, deserialized);
}