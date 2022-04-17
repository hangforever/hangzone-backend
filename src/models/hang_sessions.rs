use crate::models::hangzones::Hangzone;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HangSession {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub hangzone_id: i32,
    pub starts_at: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HangzoneHangSession {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub hangzone: Hangzone,
    pub starts_at: DateTime<Utc>,
}
