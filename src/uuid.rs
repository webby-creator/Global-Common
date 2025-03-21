use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::id::{AddonUuid, WebsitePublicId};

#[derive(Debug, Clone)]
pub struct CollectionName {
    pub id: String,
    pub ns: Option<String>,
}

impl From<&str> for CollectionName {
    fn from(value: &str) -> Self {
        if let Some((a, b)) = value.split_once(":") {
            Self {
                id: b.to_string(),
                ns: Some(a.to_string()),
            }
        } else {
            Self {
                id: value.to_string(),
                ns: None,
            }
        }
    }
}

impl<'de> Deserialize<'de> for CollectionName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        // TODO: Also parse Local Namespaces like "Forms/Name" ??
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UuidType {
    Site(WebsitePublicId),
    Addon(AddonUuid),
}

impl UuidType {
    pub fn is_site(&self) -> bool {
        matches!(self, UuidType::Site(_))
    }

    pub fn is_addon(&self) -> bool {
        matches!(self, UuidType::Addon(_))
    }

    pub fn get_uuid(&self) -> Uuid {
        match *self {
            UuidType::Site(uuid) => *uuid,
            UuidType::Addon(uuid) => *uuid,
        }
    }
}

impl ToString for UuidType {
    fn to_string(&self) -> String {
        match self {
            UuidType::Site(uuid) => format!("s:{uuid}"),
            UuidType::Addon(uuid) => format!("a:{uuid}"),
        }
    }
}

impl<'de> Deserialize<'de> for UuidType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        if let Ok(uuid) = WebsitePublicId::from_str(&value) {
            Ok(Self::Site(uuid))
        } else {
            let bytes = value.as_bytes();

            if bytes[0] == b's' {
                Ok(Self::Site(WebsitePublicId::from(
                    Uuid::try_parse_ascii(&bytes[2..]).unwrap(),
                )))
            } else if bytes[0] == b'a' {
                Ok(Self::Addon(AddonUuid::from(
                    Uuid::try_parse_ascii(&bytes[2..]).unwrap(),
                )))
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
        self.to_string().serialize(serializer)
    }
}
