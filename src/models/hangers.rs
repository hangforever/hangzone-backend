use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hanger {
    pub user_hanger_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub alias: String,
    pub icon_url: Option<String>,
    pub joined_hang_at: DateTime<Utc>,
}
