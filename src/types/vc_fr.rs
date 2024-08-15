use ark_bn254::Fr;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use bytes::Buf;
use const_hex::{decode, encode};
use serde::de::{self, Deserializer, Visitor};
use serde::ser::{Error, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default)]
pub struct VcFr(pub Fr);

struct DataVisitor;

impl<'de> Visitor<'de> for DataVisitor {
    type Value = VcFr;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hex encoded string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decoded = decode(value).map_err(de::Error::custom)?;
        let proof =
            Fr::deserialize_compressed(decoded.reader()).map_err(|e| E::custom(e.to_string()))?;

        Ok(VcFr(proof))
    }
}

impl<'de> Deserialize<'de> for VcFr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DataVisitor)
    }
}

impl Serialize for VcFr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut compressed_bytes = Vec::new();
        self.0
            .serialize_compressed(&mut compressed_bytes)
            .map_err(|e| S::Error::custom(e.to_string()))?;

        let encoded = encode(compressed_bytes);
        serializer.serialize_str(&encoded)
    }
}
