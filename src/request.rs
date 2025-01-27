use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    filter::Filter,
    schema::{SchemaView, SchematicFieldType},
    uuid::CollectionName,
    value::SimpleValue,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CmsCreate {
    pub id: CollectionName,
    pub name: String,
    pub is_single: bool,
    #[serde(flatten)]
    pub update: CmsUpdate,
    pub columns: Option<Vec<CmsCreateDataColumn>>,
    pub data: Option<HashMap<String, Vec<SimpleValue>>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CmsQuery {
    pub filters: Option<Vec<Filter>>,
    // sort[name]=ASC
    pub sort: Option<HashMap<String, String>>,
    /// Columns which should be returned
    pub columns: Option<String>,

    pub limit: Option<u64>,
    pub offset: Option<u64>,
    #[serde(default, alias = "include_files")]
    pub include_files: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CmsUpdate {
    pub views: Option<Vec<SchemaView>>,
}

#[derive(Deserialize, Serialize)]
pub struct CmsUpdateDataCell {
    pub field_name: String,
    pub value: Option<SimpleValue>,
}

// Column

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CmsCreateDataColumn {
    pub id: String,
    pub name: String,
    pub type_of: SchematicFieldType,
    #[serde(alias = "referenced_schema")]
    pub referenced_schema: Option<String>,
}

// Tags

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmsCreateDataColumnTag {
    pub tag: String,
}
