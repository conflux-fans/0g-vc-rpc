use ark_bn254::Bn254;
use ark_groth16::Proof;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use bytes::Buf;
use const_hex::{decode, encode};
use serde::de::{self, Deserializer, Visitor};
use serde::ser::{Error, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct VcProof(pub Proof<Bn254>);

struct DataVisitor;

impl<'de> Visitor<'de> for DataVisitor {
    type Value = VcProof;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hex encoded string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decoded = decode(value).map_err(de::Error::custom)?;
        let proof = Proof::<Bn254>::deserialize_compressed(decoded.reader())
            .map_err(|e| E::custom(e.to_string()))?;

        Ok(VcProof(proof))
    }
}

impl<'de> Deserialize<'de> for VcProof {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DataVisitor)
    }
}

impl Serialize for VcProof {
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

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_PROOF: &str = "\"0000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000040\"";

    #[test]
    fn test_serialize() {
        let a = VcProof::default();
        let b = serde_json::to_string(&a).unwrap();
        assert_eq!(b, DEFAULT_PROOF);
    }

    #[test]
    fn test_deserialize() {
        let deserialized: VcProof = serde_json::from_str(DEFAULT_PROOF).unwrap();

        println!("deserialized = {:?}", deserialized);
    }
}
