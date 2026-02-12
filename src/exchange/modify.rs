use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use crate::helpers::uuid_to_hex_string;

use super::{order::OrderRequest, ClientOrderRequest};

#[derive(Debug, Clone)]
pub enum OidOrCloid {
    Oid(u64),
    Cloid(Uuid),
}

impl From<u64> for OidOrCloid {
    fn from(oid: u64) -> Self {
        OidOrCloid::Oid(oid)
    }
}

impl From<Uuid> for OidOrCloid {
    fn from(cloid: Uuid) -> Self {
        OidOrCloid::Cloid(cloid)
    }
}

impl Serialize for OidOrCloid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            OidOrCloid::Oid(oid) => serializer.serialize_u64(*oid),
            OidOrCloid::Cloid(cloid) => serializer.serialize_str(&uuid_to_hex_string(*cloid)),
        }
    }
}

impl<'de> Deserialize<'de> for OidOrCloid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Number(n) => n
                .as_u64()
                .map(OidOrCloid::Oid)
                .ok_or_else(|| serde::de::Error::custom("invalid oid number")),
            serde_json::Value::String(s) => {
                let hex = s.strip_prefix("0x").unwrap_or(&s);
                Uuid::parse_str(hex)
                    .map(OidOrCloid::Cloid)
                    .map_err(serde::de::Error::custom)
            }
            _ => Err(serde::de::Error::custom("oid must be number or string")),
        }
    }
}

#[derive(Debug)]
pub struct ClientModifyRequest {
    pub oid: OidOrCloid,
    pub order: ClientOrderRequest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifyRequest {
    pub oid: OidOrCloid,
    pub order: OrderRequest,
}
