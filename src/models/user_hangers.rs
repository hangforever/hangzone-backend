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

#[derive(FromRow, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
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
pub struct UserHangerAuth<'a> {
    pub id: i32,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub alias: &'a str,
    pub email_address: &'a Option<String>,
    pub status_hang: &'a StatusHang,
    pub status_description: &'a Option<String>,
    pub icon_url: &'a Option<String>,
    token: String,
}

#[derive(FromRow, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile<'a> {
    alias: &'a str,
    status_description: &'a Option<String>,
    icon_url: &'a Option<String>,
}

impl UserHanger {
    pub fn to_user_auth(&self, token: String) -> UserHangerAuth {
        UserHangerAuth {
            id: self.id,
            first_name: &self.first_name,
            last_name: &self.last_name,
            alias: &self.alias,
            email_address: &self.email_address,
            status_hang: &self.status_hang,
            status_description: &self.status_description,
            icon_url: &self.icon_url,
            token,
        }
    }
    pub fn to_profile(&self) -> Profile {
        Profile {
            alias: &self.alias,
            status_description: &self.status_description,
            icon_url: &self.icon_url,
        }
    }
}
