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

#[derive(Deserialize)]
pub struct AddonInstallResponse {
    pub instance_uuid: AddonInstanceUuid,
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
