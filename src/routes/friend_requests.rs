use super::PaginationParams;
use crate::auth::Auth;
use crate::db;
use crate::models::notifications::NotificationType;
use crate::models::requests::RequestStatus;
use rocket::http::Status;
use rocket::serde::json::{json, Value};
use rocket::State;
use sqlx::postgres::PgPool;

#[get("/requests/friends")]
pub async fn get_friend_requests(auth: Auth, pool: &State<PgPool>) -> Value {
    let requests = db::request_friends::find(pool, auth.id)
        .await
        .map_err(|e| {
            eprintln!("Error getting friend requests: {}", e);
        })
        .unwrap_or(vec![]);
    json!({ "friend_requests": requests })
}

#[post("/requests/friends/<friend_id>")]
pub async fn request_friend(
    auth: Auth,
    friend_id: i32,
    pool: &State<PgPool>,
) -> Result<Status, &str> {
    db::friends::find_one(pool, auth.id, friend_id)
        .await
        .ok_or("Friend already exists")?;
    let transaction = pool.begin().await.map_err(|e| "Transaction error")?;
    db::request_friends::create(pool, auth.id, friend_id)
        .await
        .map_err(|e| {
            eprintln!("Problem creating friend request: {}", e);
            "Problem creating friend request"
        })?;
    handle_friend_requested_notification(pool, friend_id).await?;
    transaction.commit().await.map_err(|e| {
        eprintln!("Err commiting transaction: {}", e);
        "Error with friend request"
    })?;
    Ok(Status::Created)
}

async fn handle_friend_requested_notification<'a, 'b>(
    pool: &'a State<PgPool>,
    friend_id: i32,
) -> Result<(), &'b str> {
    db::notifications::create_one(pool, friend_id, NotificationType::FriendAdded)
        .await
        .map_err(|e| {
            eprintln!("Err creating friend: {}", e);
            "Error creating notification for friend"
        })?;
    Ok(())
}

#[post("/requests/friends/cancel/<friend_id>")]
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

#[post("/requests/friends/decline/<friend_id>")]
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

#[post("/requests/friends/accept/<friend_id>")]
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
    handle_friend_added_notification(pool, friend_id).await?;
    transaction.commit().await.map_err(|e| {
        eprintln!("Transaction begin err: {}", e);
        "Error with friend request"
    })?;
    Ok(Status::Created)
}

async fn handle_friend_added_notification<'a, 'b>(
    pool: &'a State<PgPool>,
    friend_id: i32,
) -> Result<(), &'b str> {
    db::notifications::create_one(pool, friend_id, NotificationType::FriendAdded)
        .await
        .map_err(|e| {
            eprintln!("Err creating friend: {}", e);
            "Error creating notification"
        })
}
