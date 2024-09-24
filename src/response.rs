use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    schema::{SchemaFieldMap, SchemaView, SchematicFieldKey, SchematicPermissions},
    upload::WebsiteUpload,
    value::SimpleValue,
};

// CMS

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CmsResponse {
    pub collection: PublicSchema,
    pub tags: Vec<SchemaTag>,

    // Optional Extensions
    pub form_id: Option<String>,
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
    pub row_id: String,

    pub name: String,
    pub color: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BasicCmsInfo {
    pub id: String,
    pub name: String,
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicSchema {
    pub schema_id: String,

    pub namespace: Option<String>,
    pub primary_field: String,
    pub display_name: String,

    pub permissions: SchematicPermissions,

    pub version: f64,

    pub allowed_operations: Vec<String>,

    pub fields: SchemaFieldMap,

    pub ttl: Option<i32>,
    pub default_sort: Option<String>,
    pub views: Vec<SchemaView>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}
