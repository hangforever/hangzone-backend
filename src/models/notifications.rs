use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

#[derive(sqlx::Type, Serialize, Debug)]
#[sqlx(type_name = "notification_type", rename_all = "snake_case")]
pub enum NotificationType {
    Hang,
    FriendRequest,
    FriendAdded,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: i32,
    pub user_hanger_id: i32,
    pub notification_type: NotificationType,
    pub read: bool,
    pub trash: bool,
    pub created_at: DateTime<Utc>,
}
