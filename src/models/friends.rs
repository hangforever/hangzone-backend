use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Friend {
    pub id: i32,
    pub user_hanger_id: i32,
    pub friend_user_hanger_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
