use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
#[sqlx(type = "request_status", rename_all = "snake_case")]
enum RequestStatus {
    AwaitingResponse,
    Accepted,
    Declined,
    Cancelled,
}

enum Request {
    Hang(HangRequest),
    Friend(FriendRequest),
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HangRequest {
    pub id: i32,
    pub from_user_hanger_id: i32,
    pub to_user_hanger_id: i32,
    pub message: String,
    pub status: RequestStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequest {
    pub id: i32,
    pub from_user_hanger_id: i32,
    pub to_user_hanger_id: i32,
    pub message: String,
    pub status: RequestStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
