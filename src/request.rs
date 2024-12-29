use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::filter::Filter;

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
