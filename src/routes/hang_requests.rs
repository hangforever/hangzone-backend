use super::PaginationParams;
use crate::auth::Auth;
use crate::db;
use crate::models::notifications::NotificationType;
use crate::models::requests::{FriendRequest, RequestStatus};
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
                handle_hang_requested_notification(pool, friend_id, &auth.alias)
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
    alias: &str,
) -> Result<(), &'b str> {
    db::notifications::create_one(
        pool,
        friend_id,
        NotificationType::Hang,
        &format!("{} wants to hang!", alias),
    )
    .await
    .map_err(|e| {
        eprintln!("Err creating hang request: {}", e);
        "Error creating hang request"
    })?;
    Ok(())
}

#[post("/requests/hangs/cancel/<friend_id>")]
pub async fn cancel_friend<'a, 'b>(
    pool: &'a State<PgPool>,
    auth: Auth,
    friend_id: i32,
) -> Result<(), &'b str> {
    let friend_request = db::request_friends::find_one(pool, auth.id, friend_id)
        .await
        .map_err(|e| "Invalid request")?;
    db::request_friends::update(pool, friend_request.id, RequestStatus::Cancelled)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            "Error updating friend request"
        })
}

#[post("/requests/hangs/decline/<friend_id>")]
pub async fn decline_friend<'a, 'b>(
    pool: &'a State<PgPool>,
    auth: Auth,
    friend_id: i32,
) -> Result<(), &'b str> {
    let friend_request = db::request_friends::find_one(pool, friend_id, auth.id)
        .await
        .map_err(|e| "Invalid request")?;
    db::request_friends::update(pool, friend_request.id, RequestStatus::Declined)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            "Error updating friend request"
        })
}

#[post("/requests/hangs/accept/<friend_id>")]
pub async fn accept_friend(
    auth: Auth,
    friend_id: i32,
    pool: &State<PgPool>,
) -> Result<Status, &str> {
    let transaction = pool.begin().await.map_err(|e| {
        eprintln!("Transaction err: {}", e);
        "Transaction error"
    })?;
    let friend_request = db::request_friends::find_one(pool, friend_id, auth.id)
        .await
        .map_err(|e| {
            eprintln!("Error finding friend request: {}", e);
            "Could not find friend request"
        })?;
    db::request_friends::update(pool, friend_request.id, RequestStatus::Accepted)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            "Error updating friend request"
        })?;
    db::friends::create_one(pool, auth.id, friend_id)
        .await
        .map_err(|e| "Error adding friend")?;
    handle_friend_added_notification(pool, friend_id, &auth.alias).await?;
    transaction.commit().await.map_err(|e| {
        eprintln!("Transaction begin err: {}", e);
        "Error with friend request"
    })?;
    Ok(Status::Created)
}

async fn handle_friend_added_notification<'a, 'b>(
    pool: &'a State<PgPool>,
    friend_id: i32,
    alias: &str,
) -> Result<(), &'b str> {
    db::notifications::create_one(
        pool,
        friend_id,
        NotificationType::FriendAdded,
        &format!("{} added you as a friend!", alias),
    )
    .await
    .map_err(|e| {
        eprintln!("Err creating friend: {}", e);
        "Error creating notification"
    })
}
