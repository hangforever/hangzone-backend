use super::PaginationParams;
use crate::auth::Auth;
use crate::db;
use crate::models::notifications::NotificationType;
use crate::models::requests::RequestStatus;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::Deserialize;
use rocket::State;
use sqlx::postgres::PgPool;

#[get("/requests/hangs")]
pub async fn get_hang_requests(auth: Auth, pool: &State<PgPool>) -> Value {
    let requests = db::request_hangs::find(pool, auth.id)
        .await
        .map_err(|e| {
            eprintln!("Error getting hang requests: {}", e);
        })
        .unwrap_or(vec![]);
    json!({ "hang_requests": requests })
}

#[derive(Deserialize, Debug)]
pub struct HangRequestBody {
    message: Option<String>,
    hang_session_id: i32,
    friend_ids: Vec<i32>,
}

#[post("/requests/hangs", data = "<body>")]
pub async fn request_hang(
    auth: Auth,
    pool: &State<PgPool>,
    body: Json<HangRequestBody>,
) -> Result<Status, String> {
    let body = body.into_inner();
    let existing_reqs: Vec<i32> = db::request_hangs::find_all(pool, auth.id, &body.friend_ids)
        .await
        .unwrap_or(vec![])
        .iter()
        .map(|r| r.id)
        .collect();
    for friend_id in body.friend_ids {
        // Find if there is already a request from this person
        let friend = db::friends::find_one(pool, auth.id, friend_id).await;
        if friend.is_none() {
            continue;
        }
        if !&existing_reqs.contains(&friend_id) {
            // verify hang session exists
            if let Some(hang_session) =
                db::hang_sessions::find_one(pool, body.hang_session_id).await
            {
                let transaction = pool.begin().await.map_err(|e| "Transaction error")?;
                // create request
                db::request_hangs::create(
                    pool,
                    auth.id,
                    friend_id,
                    hang_session.id,
                    body.message.clone(),
                )
                .await
                .map_err(|e| e.to_string())?;
                // send notification
                handle_hang_requested_notification(pool, friend_id)
                    .await
                    .map_err(|e| e.to_string())?;
                transaction.commit().await.map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(Status::Created)
}

async fn handle_hang_requested_notification<'a, 'b>(
    pool: &'a State<PgPool>,
    friend_id: i32,
) -> Result<(), &'b str> {
    db::notifications::create_one(pool, friend_id, NotificationType::Hang)
        .await
        .map_err(|e| {
            eprintln!("Err creating hang request: {}", e);
            "Error creating hang request"
        })?;
    Ok(())
}

#[post("/requests/hangs/cancel/<friend_id>")]
pub async fn cancel_hang<'a, 'b>(
    pool: &'a State<PgPool>,
    auth: Auth,
    friend_id: i32,
) -> Result<(), &'b str> {
    let hang_request = db::request_hangs::find_one(pool, auth.id, friend_id)
        .await
        .map_err(|e| "Invalid request")?;
    db::request_hangs::update(pool, hang_request.id, RequestStatus::Cancelled)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            "Error updating hang request"
        })
}

#[post("/requests/hangs/decline/<friend_id>")]
pub async fn decline_hang<'a, 'b>(
    pool: &'a State<PgPool>,
    auth: Auth,
    friend_id: i32,
) -> Result<(), &'b str> {
    let hang_request = db::request_hangs::find_one(pool, friend_id, auth.id)
        .await
        .map_err(|e| "Invalid request")?;
    db::request_hangs::update(pool, hang_request.id, RequestStatus::Declined)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            "Error updating friend request"
        })
}

#[post("/requests/hangs/accept/<friend_id>")]
pub async fn accept_hang(auth: Auth, friend_id: i32, pool: &State<PgPool>) -> Result<Status, &str> {
    let hang_request = db::request_hangs::find_one(pool, friend_id, auth.id)
        .await
        .map_err(|e| {
            eprintln!("Error finding request_hangs: {}", e);
            "Could not find hang request"
        })?;
    db::request_hangs::update(pool, hang_request.id, RequestStatus::Accepted)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            "Error updating friend request"
        })?;
    Ok(Status::Created)
}
