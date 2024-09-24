use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebsiteUpload {
    /// Where the upload is from.
    ///
    /// None = Main Website
    /// Some() = Addon
    pub namespace: Option<String>,

    pub public_id: String,
    pub upload_type: String,
    pub display_name: String,
    pub created_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
    pub media: Option<WebsiteUploadFile>,
    pub using_variant: Option<WebsiteUploadVariant>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebsiteUploadVariant {
    pub file_type: String,
    pub size: i64,

    pub width: i32,
    pub height: i32,
    pub ratio: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebsiteUploadFile {
    pub file_size: i64,
    pub file_type: String,

    pub media_width: Option<i32>,
    pub media_height: Option<i32>,
    pub media_duration: Option<i32>,

    pub is_editable: bool,
    pub has_thumbnail: bool,

    pub is_global: bool,
}
