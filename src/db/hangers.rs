use crate::models::hangers::Hanger;
use sqlx::postgres::PgPool;

pub async fn get(pool: &PgPool, hanger_id: i32) -> Result<Hanger, sqlx::Error> {
    sqlx::query_as!(
        Hanger,
        "
        SELECT 
          hangers.user_hanger_id,
          user_hangers.first_name,
          user_hangers.last_name,
          user_hangers.alias,
          user_hangers.icon_url,
          hangers.joined_at as joined_hang_at
        FROM hangers
        INNER JOIN user_hangers ON user_hangers.id = hangers.user_hanger_id 
        WHERE hangers.id = $1
        ",
        hanger_id
    )
    .fetch_one(pool)
    .await
}

pub async fn find(pool: &PgPool, user_hanger_id: i32) -> Result<Hanger, sqlx::Error> {
    let hanger = sqlx::query!(
        "SELECT id FROM hangers WHERE user_hanger_id = $1",
        user_hanger_id
    )
    .fetch_one(pool)
    .await?;
    get(pool, hanger.id).await
}

pub async fn create(
    pool: &PgPool,
    hang_session_id: i32,
    user_hanger_id: i32,
    host: bool,
) -> Result<Hanger, sqlx::Error> {
    let hanger = sqlx::query!(
        "
        INSERT INTO hangers 
        (hang_session_id, user_hanger_id, host) 
        VALUES ($1, $2, $3)
        RETURNING id
        ",
        hang_session_id,
        user_hanger_id,
        host,
    )
    .fetch_one(pool)
    .await?;

    get(pool, hanger.id).await
}

pub async fn delete(pool: &PgPool, user_hanger_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
        DELETE FROM hangers
        WHERE user_hanger_id = $1
        ",
        user_hanger_id
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}
