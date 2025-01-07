//! General schema definitions for the API.

use std::{
    collections::HashMap,
    fmt::Display,
    hash::{Hash, Hasher},
    time::Duration,
};

use eyre::{anyhow, Result};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use time::{macros::format_description, Date, OffsetDateTime, PrimitiveDateTime, Time};
use url::Url;
use uuid::Uuid;

use crate::value::{Number, SimpleValue};

pub type SchemaFieldMap = HashMap<SchematicFieldKey, SchematicField>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schematic {
    pub id: String,
    /// What the schema is for: Forms, Members, Marketing, Billing, etc.
    pub namespace: String,
    /// The field to display if it's being referenced.
    pub primary_field: String,
    /// The name of the schema.
    pub display_name: String,
    /// The capabilities of the schema.
    pub permissions: SchematicPermissions,
    pub version: f64,
    /// The operations allowed on the schema.
    pub allowed_operations: Vec<String>,
    pub is_deleted: bool,
    pub owner_app_id: String,
    pub fields: SchemaFieldMap,
    // pub storage: String,
    /// Time to live
    pub ttl: Option<Duration>,
    pub default_sort: Option<DefaultSort>,
    // pub paging_mode: Vec<String>,
    pub views: Vec<SchemaView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaView {
    pub name: String,
    pub query: SchemaViewQuery,
    pub view_type: SchemaViewTypes,
}

impl Default for SchemaView {
    fn default() -> Self {
        Self {
            name: String::from("Default View"),
            query: SchemaViewQuery::default(),
            view_type: SchemaViewTypes::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SchemaViewTypes {
    pub form: SchemaViewItem,
    pub gallery: SchemaViewItem,
    pub list: SchemaViewItem,
    pub table: SchemaViewItem,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaViewItem {
    #[serde(default)]
    pub hidden_fields: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SchemaViewQuery {
    pub sort: Vec<DefaultSort>,
    pub filter: Vec<SchemaFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicPermissions {
    pub insert: PermissionsUser,
    pub update: PermissionsUser,
    pub remove: PermissionsUser,
    pub read: PermissionsUser,
}

impl Default for SchematicPermissions {
    fn default() -> Self {
        Self {
            insert: PermissionsUser::Admin,
            update: PermissionsUser::Admin,
            remove: PermissionsUser::Admin,
            read: PermissionsUser::Admin,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionsUser {
    Anyone,
    Admin,
    Owner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operations {
    BulkInsert,
    BulkSave,
    QueryReferenced,
    Truncate,
    ReplaceReferences,
    Count,
    Get,
    Find,
    RemoveReference,
    IsReferenced,
    Distinct,
    Remove,
    BulkUpdate,
    Insert,
    Save,
    Update,
    BulkRemove,
    Aggregate,
    InsertReference,
}

#[derive(Debug, Clone, Eq)]
pub enum SchematicFieldKey {
    Id,
    Owner,
    CreatedAt,
    UpdatedAt,
    Other(String),
    OtherStatic(&'static str),
}

impl SchematicFieldKey {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Id => "_id",
            Self::Owner => "_owner",
            Self::CreatedAt => "_createdAt",
            Self::UpdatedAt => "_updatedAt",
            Self::Other(s) => s,
            Self::OtherStatic(s) => s,
        }
    }

    pub fn is_other(&self) -> bool {
        matches!(self, Self::Other(_) | Self::OtherStatic(_))
    }
}

impl Hash for SchematicFieldKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl Display for SchematicFieldKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl PartialEq for SchematicFieldKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<&str> for SchematicFieldKey {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<String> for SchematicFieldKey {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

impl Serialize for SchematicFieldKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for SchematicFieldKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        Ok(match s.as_str() {
            "_id" => Self::Id,
            "_owner" => Self::Owner,
            "_createdAt" => Self::CreatedAt,
            "_updatedAt" => Self::UpdatedAt,
            _ => Self::Other(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchematicField {
    pub display_name: String,
    pub sortable: bool,
    pub is_deleted: bool,
    pub system_field: bool,
    pub field_type: SchematicFieldType,
    pub index: u16,

    // Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_schema: Option<String>,
    // TODO: Default value setter - used for when "duplicating another field"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaFilter {
    pub field: String,
    pub condition: String,
    pub value: SchematicFieldValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultSort {
    pub field: String,
    pub order: SortOrder,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchematicFieldBasicType {
    Text,
    Number,
    Boolean,
    DateTime,
    Date,
    Time,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TryFromPrimitive, IntoPrimitive,
)]
#[repr(i32)]
pub enum SchematicFieldType {
    /// A string of text.
    Text,
    /// A number.
    Number,
    /// A URL.
    URL,
    /// An email address.
    Email,
    /// An address.
    Address,
    /// A phone number.
    Phone,
    /// A boolean.
    Boolean,
    /// A date and time.
    DateTime,
    /// A date.
    Date,
    /// A time.
    Time,
    /// Rich content.
    RichContent,
    /// Rich text.
    RichText,
    /// A reference to another schema item.
    Reference,
    /// A reference to multiple schema items.
    MultiReference,
    /// A media gallery.
    MediaGallery,
    /// A document.
    Document,
    /// A multi-document.
    MultiDocument,
    /// An image.
    Image,
    /// A video.
    Video,
    /// An audio.
    Audio,
    /// An array of tags.
    Tags,
    /// An array
    Array,
    /// An object.
    Object,
}

impl SchematicFieldType {
    // TODO: Better Name. Used to determine if bytes being uploaded are a file or not.
    pub fn is_upload_file_type(&self) -> bool {
        matches!(
            self,
            SchematicFieldType::Document
                | SchematicFieldType::MultiDocument
                | SchematicFieldType::Audio
                | SchematicFieldType::Image
                | SchematicFieldType::Video
        )
    }

    pub fn max_bytes_length(&self) -> Option<usize> {
        match self {
            Self::Text => Some(1024 * 1024 * 1024),
            Self::Email => Some(100),
            Self::Number => Some(10),
            Self::URL => Some(1024),
            Self::Address => Some(1024),
            Self::Phone => Some(50),
            Self::Boolean => Some(1),
            Self::DateTime => Some(50),
            Self::Date => Some(50),
            Self::Time => Some(50),
            Self::RichContent => Some(1024 * 1024 * 10),
            Self::RichText => Some(1024 * 1024 * 10),
            Self::Reference => None,
            Self::MultiReference => None,
            Self::MediaGallery => Some(1024 * 1024 * 100),
            Self::Document => Some(1024 * 1024 * 100),
            Self::MultiDocument => Some(1024 * 1024 * 100),
            Self::Image => Some(1024 * 1024 * 100),
            Self::Video => Some(1024 * 1024 * 100),
            Self::Audio => Some(1024 * 1024 * 100),
            Self::Tags => None, // TODO
            Self::Array => None,
            Self::Object => None,
        }
    }

    pub fn parse_value_bytes(self, bytes: Vec<u8>) -> Result<SimpleValue> {
        match self {
            SchematicFieldType::Number => Ok(serde_json::from_slice(&bytes)?),
            SchematicFieldType::Text
            | SchematicFieldType::URL
            | SchematicFieldType::Email
            | SchematicFieldType::Address
            | SchematicFieldType::Phone
            | SchematicFieldType::Boolean
            | SchematicFieldType::DateTime
            | SchematicFieldType::Date
            | SchematicFieldType::Time
            | SchematicFieldType::RichContent
            | SchematicFieldType::RichText
            | SchematicFieldType::Reference
            | SchematicFieldType::Array
            | SchematicFieldType::Object => Ok(SimpleValue::Text(String::from_utf8(bytes)?)),
            SchematicFieldType::Document
            | SchematicFieldType::Image
            | SchematicFieldType::Video
            | SchematicFieldType::Audio => Ok(SimpleValue::ListNumber(
                bytes.into_iter().map(|v| v.into()).collect(),
            )),
            SchematicFieldType::MultiReference
            | SchematicFieldType::MediaGallery
            | SchematicFieldType::MultiDocument
            | SchematicFieldType::Tags => {
                todo!("{:?} {bytes:?}", String::from_utf8_lossy(&bytes));
            }
        }
    }

    pub fn parse_value(self, received: SimpleValue) -> Result<SchematicFieldValue> {
        Ok(match self {
            Self::Text => SchematicFieldValue::Text(received.try_as_text()?),
            Self::Number => SchematicFieldValue::Number(received.try_as_number()?),
            Self::URL => SchematicFieldValue::Url(Url::parse(&received.try_as_text()?)?),
            Self::Email => SchematicFieldValue::Email(received.try_as_text()?),
            Self::Phone => SchematicFieldValue::Phone(received.try_as_text()?),
            Self::Address => SchematicFieldValue::Address(received.try_as_text()?),
            Self::Boolean => SchematicFieldValue::Boolean(match received.try_as_text()?.as_str() {
                "1" | "on" | "true" => true,
                "0" | "off" | "false" => false,
                v => v.parse()?,
            }),
            // TODO: Optional seconds
            Self::DateTime => SchematicFieldValue::DateTime({
                if let Ok(v) = PrimitiveDateTime::parse(
                    &received.any_as_text()?,
                    format_description!("[year]-[month]-[day]T[hour]:[minute]"),
                ) {
                    v.assume_utc()
                } else {
                    PrimitiveDateTime::parse(
                        &received.any_as_text()?,
                        format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
                    )?
                    .assume_utc()
                }
            }),
            Self::Date => SchematicFieldValue::Date(Date::parse(
                &received.any_as_text()?,
                format_description!("[year]-[month]-[day]"),
            )?),
            Self::Time => SchematicFieldValue::Time({
                if let Ok(v) = Time::parse(
                    &received.any_as_text()?,
                    format_description!("[hour]:[minute]:[second]"),
                ) {
                    v
                } else {
                    Time::parse(
                        &received.any_as_text()?,
                        format_description!("[hour]:[minute]:[second].[subsecond]"),
                    )?
                }
            }),
            Self::RichContent => SchematicFieldValue::Text(received.try_as_text()?),
            Self::RichText => SchematicFieldValue::Text(received.try_as_text()?),
            Self::Reference => SchematicFieldValue::Reference(received.try_as_text()?.parse()?),
            Self::MultiReference => SchematicFieldValue::MultiReference(
                received
                    .try_as_list_string()?
                    .into_iter()
                    .map(|v| v.parse())
                    .collect::<std::result::Result<Vec<_>, _>>()?,
            ),
            Self::MediaGallery => SchematicFieldValue::MultiReference(
                received
                    .try_as_list_string()?
                    .into_iter()
                    .map(|v| v.parse())
                    .collect::<std::result::Result<Vec<_>, _>>()?,
            ),
            Self::Document | Self::Image | Self::Video | Self::Audio => {
                SchematicFieldValue::ListNumber(received.try_as_list_number()?)
            }
            Self::MultiDocument => todo!("Multi Document"),
            Self::Tags => SchematicFieldValue::ListNumber(received.try_as_list_number()?),
            Self::Array => {
                let value = match received {
                    SimpleValue::Text(v) => serde_json::from_str(&v)?,
                    v => serde_json::from_value(serde_json::to_value(v)?)?,
                };

                SchematicFieldValue::Array(value)
            }
            Self::Object => {
                let value = match received {
                    SimpleValue::Text(v) => serde_json::from_str(&v)?,
                    v => serde_json::from_value(serde_json::to_value(v)?)?,
                };

                SchematicFieldValue::Object(value)
            }
        })
    }

    pub fn as_name(self) -> &'static str {
        match self {
            Self::Text => "Text",
            Self::Number => "Number",
            Self::URL => "URL",
            Self::Email => "Email",
            Self::Address => "Address",
            Self::Phone => "Phone",
            Self::Boolean => "True/False",
            Self::DateTime => "Date & Time",
            Self::Date => "Date",
            Self::Time => "Time",
            Self::RichContent => "Rich Content",
            Self::RichText => "Rich Text",
            Self::Reference => "Reference",
            Self::MultiReference => "Multi Reference",
            Self::MediaGallery => "Media Gallery",
            Self::Document => "Document",
            Self::MultiDocument => "Multi Document",
            Self::Image => "Image",
            Self::Video => "Video",
            Self::Audio => "Audio",
            Self::Tags => "Tags",
            Self::Array => "Array",
            Self::Object => "Object",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum SchematicFieldValue {
    // Url gets serialized/deserialized to/from a String
    Text(String),
    Number(Number),
    Boolean(bool),

    Url(Url),
    Email(String),
    Phone(String),
    Address(String),

    DateTime(OffsetDateTime),
    Date(Date),
    Time(Time),
    Reference(Uuid),
    MultiReference(Vec<Uuid>),
    ListString(Vec<String>),
    ListNumber(Vec<Number>),

    Array(Vec<serde_json::Value>),
    Object(serde_json::Value),
}

impl SchematicFieldValue {
    pub fn try_as_reference(self) -> Result<Uuid> {
        if let Self::Reference(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to Reference"))?;
        }
    }

    pub fn try_as_text(self) -> Result<String> {
        if let Self::Text(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to Text"))?;
        }
    }

    pub fn try_as_number(&self) -> Result<Number> {
        if let Self::Number(v) = self {
            Ok(*v)
        } else {
            return Err(anyhow!("Unable to convert to Number"))?;
        }
    }

    pub fn try_as_boolean(&self) -> Result<bool> {
        if let Self::Boolean(v) = self {
            Ok(*v)
        } else {
            return Err(anyhow!("Unable to convert to Boolean"))?;
        }
    }

    pub fn try_as_url(self) -> Result<Url> {
        if let Self::Url(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to Url"))?;
        }
    }

    pub fn try_as_email(self) -> Result<String> {
        if let Self::Email(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to Email"))?;
        }
    }

    pub fn try_as_phone(self) -> Result<String> {
        if let Self::Phone(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to Phone"))?;
        }
    }

    pub fn try_as_address(self) -> Result<String> {
        if let Self::Address(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to Address"))?;
        }
    }

    pub fn try_as_date_time(&self) -> Result<OffsetDateTime> {
        if let Self::DateTime(v) = self {
            Ok(*v)
        } else {
            return Err(anyhow!("Unable to convert to DateTime"))?;
        }
    }

    pub fn try_as_date(&self) -> Result<Date> {
        if let Self::Date(v) = self {
            Ok(*v)
        } else {
            return Err(anyhow!("Unable to convert to Date"))?;
        }
    }

    pub fn try_as_time(&self) -> Result<Time> {
        if let Self::Time(v) = self {
            Ok(*v)
        } else {
            return Err(anyhow!("Unable to convert to Time"))?;
        }
    }

    pub fn try_as_list_string(self) -> Result<Vec<String>> {
        if let Self::ListString(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to String List"))?;
        }
    }

    pub fn try_as_list_number(self) -> Result<Vec<Number>> {
        if let Self::ListNumber(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to Number List"))?;
        }
    }

    pub fn try_as_list_reference(self) -> Result<Vec<Uuid>> {
        if let Self::MultiReference(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to Reference List"))?;
        }
    }

    pub fn try_as_array(self) -> Result<Vec<serde_json::Value>> {
        if let Self::Array(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to Object Array"))?;
        }
    }

    pub fn try_as_object(self) -> Result<serde_json::Value> {
        if let Self::Object(v) = self {
            Ok(v)
        } else {
            return Err(anyhow!("Unable to convert to Object"))?;
        }
    }
}

#[cfg(feature = "sqlx")]
const _: () = {
    use std::result::Result;

    use sqlx::{
        database::{HasArguments, HasValueRef},
        encode::IsNull,
        error::BoxDynError,
        sqlite::{SqliteRow, SqliteTypeInfo},
        Decode, Encode, FromRow, Row, Sqlite, Type,
    };

    impl FromRow<'_, SqliteRow> for SchematicFieldType {
        fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
            Ok(Self::try_from(row.try_get::<i32, _>(0)?).unwrap())
        }
    }

    impl Encode<'_, Sqlite> for SchematicFieldType {
        fn encode_by_ref(&self, buf: &mut <Sqlite as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
            Encode::<Sqlite>::encode_by_ref(&(*self as i32), buf)
        }
    }

    impl Decode<'_, Sqlite> for SchematicFieldType {
        fn decode(value: <Sqlite as HasValueRef<'_>>::ValueRef) -> Result<Self, BoxDynError> {
            Ok(Self::try_from(<i32 as Decode<Sqlite>>::decode(value)?)?)
        }
    }

    impl Type<Sqlite> for SchematicFieldType {
        fn type_info() -> SqliteTypeInfo {
            <i32 as Type<Sqlite>>::type_info()
        }
    }
};
