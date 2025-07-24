use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    id::{AddonInstanceUuid, FormPublicId, SchemaDataPublicId},
    schema::{SchemaFieldMap, SchemaView, SchematicFieldKey, SchematicPermissions},
    upload::WebsiteUpload,
    value::SimpleValue,
};

// Addon

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddonInstallResponse {
    pub instance_uuid: AddonInstanceUuid,
    // TODO: Replace with Vec<PublicPage>
    pub new_pages: serde_json::Value,
}

// CMS

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CmsCreateResponse {
    pub id: String,
    pub name: String,
    pub namespace: Option<String>,
    pub data_ids: Option<Vec<SchemaDataPublicId>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CmsResponse {
    pub collection: PublicSchema,
    pub tags: Vec<SchemaTag>,

    // Optional Extensions
    pub form_id: Option<FormPublicId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmsRowResponse {
    #[serde(default)]
    pub files: Vec<WebsiteUpload>,
    pub fields: HashMap<SchematicFieldKey, SimpleValue>,
}

// TODO: Remove - make public version

#[derive(Clone, Serialize, Deserialize)]
pub struct SchemaTag {
    pub id: i64,
    pub row_id: String,

    pub name: String,
    pub color: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicCmsInfo {
    pub id: String,
    pub name: String,
    pub namespace: Option<String>,
    pub is_single: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicSchema {
    pub schema_id: String,

    pub namespace: Option<String>,
    pub primary_field: String,
    pub display_name: String,

    pub permissions: SchematicPermissions,

    pub version: f32,

    pub allowed_operations: Vec<String>,
    pub is_single: bool,

    pub fields: SchemaFieldMap,

    pub ttl: Option<i32>,
    pub default_sort: Option<String>,
    pub views: Vec<SchemaView>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

// GENERAL

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub offset: i64,
    pub limit: i64,
    pub total: i64,
}

impl<T> ListResponse<T> {
    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            offset: 0,
            limit: 0,
            total: 0,
        }
    }

    pub fn all(value: Vec<T>) -> Self {
        Self {
            offset: 0,
            limit: value.len() as i64,
            total: value.len() as i64,
            items: value,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleValue<V>(pub V);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum WrappingResponse<V> {
    Resp(V),
    Error(ApiErrorResponse),
}

impl<V> WrappingResponse<V> {
    pub fn okay(value: V) -> Self {
        Self::Resp(value)
    }

    pub fn error<S: Into<String>>(value: S) -> Self {
        Self::Error(ApiErrorResponse::new(value))
    }

    pub fn ok(self) -> std::result::Result<V, ApiErrorResponse> {
        match self {
            Self::Resp(v) => Ok(v),
            Self::Error(e) => Err(e),
        }
    }

    pub fn as_ok(&self) -> std::result::Result<&V, &ApiErrorResponse> {
        match self {
            Self::Resp(v) => Ok(v),
            Self::Error(e) => Err(e),
        }
    }

    pub fn map<N, F: Fn(V) -> N>(self, func: F) -> WrappingResponse<N> {
        match self {
            Self::Resp(v) => WrappingResponse::Resp(func(v)),
            Self::Error(e) => WrappingResponse::Error(e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, thiserror::Error)]
pub struct ApiErrorResponse {
    pub description: String,
}

impl ApiErrorResponse {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self {
            description: value.into(),
        }
    }
}

impl std::fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Api Error Occurred: {}", self.description)
    }
}
