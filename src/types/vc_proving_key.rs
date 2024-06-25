use ark_bn254::Bn254;
use ark_ec::pairing::Pairing;
use ark_groth16::{ProvingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use const_hex::{decode, encode};
use jsonrpc_http_server::hyper::body::Buf;
use serde::de::{self, Deserializer, Visitor};
use serde::ser::{Error, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

pub struct VcProvingKey(pub ProvingKey<Bn254>);

impl Default for VcProvingKey {
    fn default() -> Self {
        let vk = VerifyingKey::<Bn254>::default();
        let pk: ProvingKey<Bn254> = ProvingKey {
            vk,
            beta_g1: <Bn254 as Pairing>::G1Affine::default(),
            delta_g1: Default::default(),
            a_query: Default::default(),
            b_g1_query: Default::default(),
            b_g2_query: Default::default(),
            h_query: Default::default(),
            l_query: Default::default(),
        };
        VcProvingKey(pk)
    }
}

struct DataVisitor;

impl<'de> Visitor<'de> for DataVisitor {
    type Value = VcProvingKey;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a base64 encoded string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decoded = decode(value).map_err(de::Error::custom)?;
        let proof = ProvingKey::<Bn254>::deserialize_compressed(decoded.reader())
            .map_err(|e| E::custom(e.to_string()))?;

        Ok(VcProvingKey(proof))
    }
}

impl<'de> Deserialize<'de> for VcProvingKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DataVisitor)
    }
}

impl Serialize for VcProvingKey {
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
