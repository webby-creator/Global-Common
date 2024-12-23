use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct CollectionName {
    pub id: String,
    pub ns: Option<String>,
}

impl<'de> Deserialize<'de> for CollectionName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        if let Some((a, b)) = value.split_once(":") {
            Ok(Self {
                id: b.to_string(),
                ns: Some(a.to_string()),
            })
        } else {
            Ok(Self {
                id: value,
                ns: None,
            })
        }
    }
}

impl Serialize for CollectionName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(ns) = self.ns.as_deref() {
            format!("{ns}:{}", self.id).serialize(serializer)
        } else {
            self.id.serialize(serializer)
        }
    }
}

impl std::fmt::Display for CollectionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ns) = self.ns.as_deref() {
            format!("{ns}:{}", self.id).fmt(f)
        } else {
            self.id.fmt(f)
        }
    }
}

pub enum UuidType {
    Site(Uuid),
    Addon(Uuid),
}

impl UuidType {
    pub fn get_uuid(&self) -> Uuid {
        match *self {
            UuidType::Site(uuid) => uuid,
            UuidType::Addon(uuid) => uuid,
        }
    }
}

impl<'de> Deserialize<'de> for UuidType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        if let Ok(uuid) = Uuid::parse_str(&value) {
            Ok(Self::Site(uuid))
        } else {
            let bytes = value.as_bytes();

            if bytes[0] == b's' {
                Ok(Self::Site(Uuid::try_parse_ascii(&bytes[2..]).unwrap()))
            } else if bytes[0] == b'a' {
                Ok(Self::Addon(Uuid::try_parse_ascii(&bytes[2..]).unwrap()))
            } else {
                Err(serde::de::Error::custom("Unknown"))
            }
        }
    }
}

impl Serialize for UuidType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            UuidType::Site(uuid) => format!("s:{uuid}").serialize(serializer),
            UuidType::Addon(uuid) => format!("a:{uuid}").serialize(serializer),
        }
    }
}
