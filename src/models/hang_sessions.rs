use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HangSession {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub geography: Option<String>,
    pub hangzone_id: Option<i32>,
    pub starts_at: DateTime<Utc>,
}
