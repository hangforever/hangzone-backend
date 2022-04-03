use crate::models::requests::{FriendRequest, RequestStatus};
use sqlx::postgres::PgPool;

pub async fn find_one(pool: &PgPool, from_id: i32, to_id: i32) -> Option<FriendRequest> {
    sqlx::query_as!(
        FriendRequest,
        r#"
            SELECT id, from_user_hanger_id, to_user_hanger_id, message, status as "status: RequestStatus", created_at, updated_at 
            FROM request_friends
            WHERE from_user_hanger_id = $1
                AND to_user_hanger_id = $2
        "#,
        from_id,
        to_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| eprintln!("Could not find request_friend: {}", e))
    .ok()
}

pub async fn find(pool: &PgPool, to_id: i32) -> Option<Vec<FriendRequest>> {
    sqlx::query_as!(
        FriendRequest,
        r#"
            SELECT id, from_user_hanger_id, to_user_hanger_id, message, status as "status: RequestStatus", created_at, updated_at 
            FROM request_friends
            WHERE to_user_hanger_id = $1
        "#,
        to_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| eprintln!("Could not find request_friend: {}", e))
    .ok()
}

pub async fn create(
    pool: &PgPool,
    from_id: i32,
    to_id: i32,
    message: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO request_friends
            (from_user_hanger_id, to_user_hanger_id, message, status) 
            VALUES ($1, $2, $3, $4)
            RETURNING id
        "#,
        from_id,
        to_id,
        message,
        RequestStatus::AwaitingResponse as RequestStatus,
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}
