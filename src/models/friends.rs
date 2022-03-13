use super::user_hangers::StatusHang;
use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Friend {
    pub first_name: String,
    pub last_name: String,
    pub alias: String,
    pub email_address: Option<String>,
    pub status_hang: StatusHang,
    pub status_description: Option<String>,
    pub icon_url: Option<String>,
    // pub geography: (f32, f32),
    pub current_hangzone_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
