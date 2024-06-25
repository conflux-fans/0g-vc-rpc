use num_bigint::BigInt;
use num_traits::Num;
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug)]
pub struct VcBigInt(pub BigInt);

struct DataVisitor;

impl<'de> Visitor<'de> for DataVisitor {
    type Value = VcBigInt;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a base64 encoded string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let val =
            BigInt::from_str_radix(value, 16).map_err(|e| de::Error::custom(e.to_string()))?;

        Ok(VcBigInt(val))
    }
}

impl<'de> Deserialize<'de> for VcBigInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DataVisitor)
    }
}

impl Serialize for VcBigInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_str = format!("{:x}", self.0);
        serializer.serialize_str(&hex_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let a = VcBigInt(BigInt::from(64));
        let b = serde_json::to_string(&a).unwrap();
        assert_eq!(b, "\"0x40\"");
    }

    #[test]
    fn test_deserialize() {
        let deserialized: VcBigInt = serde_json::from_str("\"40\"").unwrap();
        assert_eq!(deserialized.0, BigInt::from(64));
    }
}
