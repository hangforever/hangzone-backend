use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[repr(i32)]
pub enum StatusHang {
    Hanging = 0,
    WantHanging = 1,
    Busy = 2,
}

pub struct UserHanger {
    pub id: i32,
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

#[derive(FromRow, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserHangerJson {
    pub id: i32,
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
