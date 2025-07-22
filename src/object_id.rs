use std::{fmt, ops::Deref};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub static MAIN_WEBSITE_OBJ_ID: &str = "main";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectIdTuple {
    pub id: ObjectId,
    pub guid: ObjectGuid,
}

impl PartialEq<ObjectId> for ObjectIdTuple {
    fn eq(&self, other: &ObjectId) -> bool {
        self.id == *other
    }
}

impl PartialEq<ObjectGuid> for ObjectIdTuple {
    fn eq(&self, other: &ObjectGuid) -> bool {
        self.guid == *other
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(String);

impl ObjectId {
    pub fn from_specific(value: String) -> Self {
        Self(value)
    }

    pub fn try_rename(&mut self, value: String) -> bool {
        let new_id = Self(value);

        *self = new_id;

        true
    }
}

impl ObjectId {
    pub fn website() -> Self {
        Self(MAIN_WEBSITE_OBJ_ID.to_string())
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for ObjectId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for ObjectId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ObjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Ok(ObjectId::from_specific(value))
    }
}

impl PartialEq<&str> for ObjectId {
    fn eq(&self, other: &&str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<String> for ObjectId {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl From<String> for ObjectId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectGuid(pub Uuid);

impl Default for ObjectGuid {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjectGuid {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// UUID is zeroed
    pub fn website() -> Self {
        Self(Uuid::nil())
    }

    pub fn is_website(&self) -> bool {
        *self == Self::website()
    }
}

impl fmt::Display for ObjectGuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for ObjectGuid {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
