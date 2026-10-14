//! Strict JSON parsing and stable, whitespace-free, sorted-key digests.
use crate::{Diagnostic, MAX_MESSAGE, Result};
use serde::de::{DeserializeOwned, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Number, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

struct StrictValue(Value);
impl<'de> Deserialize<'de> for StrictValue {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct StrictVisitor;
        impl<'de> Visitor<'de> for StrictVisitor {
            type Value = StrictValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("JSON with unique keys and unsigned 32-bit integer numbers")
            }
            fn visit_unit<E: serde::de::Error>(self) -> std::result::Result<Self::Value, E> {
                Ok(StrictValue(Value::Null))
            }
            fn visit_bool<E: serde::de::Error>(
                self,
                v: bool,
            ) -> std::result::Result<Self::Value, E> {
                Ok(StrictValue(Value::Bool(v)))
            }
            fn visit_str<E: serde::de::Error>(
                self,
                v: &str,
            ) -> std::result::Result<Self::Value, E> {
                Ok(StrictValue(Value::String(v.to_owned())))
            }
            fn visit_string<E: serde::de::Error>(
                self,
                v: String,
            ) -> std::result::Result<Self::Value, E> {
                Ok(StrictValue(Value::String(v)))
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> std::result::Result<Self::Value, E> {
                if v > u32::MAX as u64 {
                    return Err(E::custom("integer exceeds u32"));
                }
                Ok(StrictValue(Value::Number(Number::from(v))))
            }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<Self::Value, E> {
                if v < 0 {
                    return Err(E::custom("negative JSON numbers are not supported"));
                }
                self.visit_u64(v as u64)
            }
            fn visit_f64<E: serde::de::Error>(self, _: f64) -> std::result::Result<Self::Value, E> {
                Err(E::custom("floating JSON numbers are not supported"))
            }
            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> std::result::Result<Self::Value, A::Error> {
                let mut values = Vec::new();
                while let Some(StrictValue(v)) = seq.next_element()? {
                    values.push(v);
                }
                Ok(StrictValue(Value::Array(values)))
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> std::result::Result<Self::Value, A::Error> {
                let mut values = Map::new();
                let mut seen = BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    if !seen.insert(key.clone()) {
                        return Err(serde::de::Error::custom(format!("duplicate key: {key}")));
                    }
                    let StrictValue(v) = map.next_value()?;
                    values.insert(key, v);
                }
                Ok(StrictValue(Value::Object(values)))
            }
        }
        d.deserialize_any(StrictVisitor)
    }
}

pub fn parse<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    if bytes.len() > MAX_MESSAGE {
        return Err(Diagnostic::new(
            "canonical",
            "message-too-large",
            "JSON exceeds the message limit",
        ));
    }
    let StrictValue(value) = serde_json::from_slice(bytes)
        .map_err(|e| Diagnostic::new("canonical", "invalid-json", e.to_string()))?;
    serde_json::from_value(value)
        .map_err(|e| Diagnostic::new("canonical", "invalid-contract", e.to_string()))
}

pub fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let provisional = serde_json::to_vec(value)
        .map_err(|e| Diagnostic::new("canonical", "encoding-failed", e.to_string()))?;
    let canonical: Value = parse(&provisional)?;
    // serde_json's default map representation is sorted by key; no preserve_order feature.
    let bytes = serde_json::to_vec(&canonical)
        .map_err(|e| Diagnostic::new("canonical", "encoding-failed", e.to_string()))?;
    if bytes.len() > MAX_MESSAGE {
        return Err(Diagnostic::new(
            "canonical",
            "message-too-large",
            "JSON exceeds the message limit",
        ));
    }
    Ok(bytes)
}

pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn digest<T: Serialize>(value: &T) -> Result<String> {
    Ok(hash(&encode(value)?))
}
pub fn word_digest(words: &[u32]) -> String {
    let mut hash = Sha256::new();
    for word in words {
        hash.update(word.to_le_bytes());
    }
    format!("{:x}", hash.finalize())
}
pub fn is_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Request;
    #[test]
    fn parser_is_strict_and_encoding_is_stable() {
        assert!(parse::<Value>(br#"{"a":1,"a":2}"#).is_err());
        for value in [b"-1".as_slice(), b"1.0", b"4294967296"] {
            assert!(parse::<Value>(value).is_err());
        }
        assert!(parse::<Request>(br#"{"method":"describe","required_fact":true}"#).is_err());
        let value: Value = parse(br#"{"z":1,"a":2}"#).unwrap();
        assert_eq!(encode(&value).unwrap(), br#"{"a":2,"z":1}"#);
    }
}
