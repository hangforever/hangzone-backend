use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hangzone {
    pub id: i32,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub address_1: String,
    pub address_2: Option<String>,
    pub address_3: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub country: String,
    pub postal_code: Option<String>,
    pub geography: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
