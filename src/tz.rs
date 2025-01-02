use std::sync::LazyLock;

use serde::{Deserialize, Deserializer};
use time::{macros::format_description, UtcOffset};

static TZ_DATABASE: LazyLock<ZoneContainer> = LazyLock::new(|| {
    let data = include_str!("../TimeZones-2024b.json");
    serde_json::from_str(data).unwrap()
});

pub fn find_offset_by_id(id: &str) -> Option<UtcOffset> {
    TZ_DATABASE
        .zones
        .iter()
        .find(|z| z.id == id || z.aliases.iter().any(|a| a == id))
        .map(|z| z.offsets[0])
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneContainer {
    // iana_version: String,
    // full_version_id: String,
    zones: Vec<Zone>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zone {
    id: String,
    aliases: Vec<String>,
    // location
    #[serde(deserialize_with = "from_str_to_utc_offset")]
    offsets: Vec<UtcOffset>,
    // current_offset: String,
    // nextTransition
}

fn from_str_to_utc_offset<'de, D>(v: D) -> Result<Vec<UtcOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Vec<String> = Deserialize::deserialize(v)?;

    let format = format_description!("[offset_hour]:[offset_minute]");
    let format_hour = format_description!("[offset_hour]");

    Ok(v.iter()
        .map(|s| {
            UtcOffset::parse(s, format)
                .or_else(|_| UtcOffset::parse(s, format_hour))
                .unwrap()
        })
        .collect())
}
