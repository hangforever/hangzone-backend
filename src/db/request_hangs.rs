use crate::models::requests::{HangRequest, RequestStatus};
use sqlx::postgres::PgPool;

pub async fn find_one(pool: &PgPool, from_id: i32, to_id: i32) -> Result<HangRequest, sqlx::Error> {
    sqlx::query_as!(
        HangRequest,
        r#"
            SELECT id, from_user_hanger_id, to_user_hanger_id, message, status as "status: RequestStatus", hang_session_id, created_at, updated_at 
            FROM request_hangs
            WHERE from_user_hanger_id = $1
                AND to_user_hanger_id = $2
        "#,
        from_id,
        to_id
    )
    .fetch_one(pool)
    .await
}

pub async fn find(pool: &PgPool, to_id: i32) -> Result<Vec<HangRequest>, sqlx::Error> {
    sqlx::query_as!(
        HangRequest,
        r#"
            SELECT id, from_user_hanger_id, to_user_hanger_id, message, status as "status: RequestStatus", hang_session_id, created_at, updated_at 
            FROM request_hangs
            WHERE to_user_hanger_id = $1
        "#,
        to_id
    )
    .fetch_all(pool)
    .await
}

pub async fn create(
    pool: &PgPool,
    from_id: i32,
    to_id: i32,
    hang_session_id: i32,
    message: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO request_hangs
            (from_user_hanger_id, to_user_hanger_id, message, hang_session_id, status) 
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
        "#,
        from_id,
        to_id,
        message,
        hang_session_id,
        RequestStatus::AwaitingResponse as RequestStatus,
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}

pub async fn update(pool: &PgPool, id: i32, status: RequestStatus) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
            UPDATE request_hangs
            SET 
                status = $1
            WHERE id = $2 
            RETURNING id
        "#,
    )
    .bind(status)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(())
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            DELETE FROM request_hangs
            WHERE id = $1
            RETURNING id
        "#,
        id,
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}
