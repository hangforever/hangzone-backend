use crate::models::notifications::{Notification, NotificationType};
use sqlx::postgres::PgPool;
use sqlx::Row;

pub async fn create_one(
    pool: &PgPool,
    user_hanger_id: i32,
    notification_type: NotificationType,
    content: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
        INSERT INTO notifications
        (user_hanger_id,
        notification_type,
        content)
        VALUES 
        ($1, $2, $3)
        RETURNING id
        ",
        user_hanger_id,
        notification_type as NotificationType,
        content
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}

pub async fn find(pool: &PgPool, user_hanger_id: i32) -> Result<Vec<Notification>, sqlx::Error> {
    sqlx::query_as!(
        Notification,
        r#"
            SELECT id, user_hanger_id, notification_type as "notification_type: NotificationType", read, trash, created_at, updated_at, content 
            FROM notifications 
            WHERE 
              user_hanger_id = $1 AND
              trash = false
        "#,
        user_hanger_id
    )
    .fetch_all(pool)
    .await
}

pub async fn mark_as_read(pool: &PgPool, ids: &[i32]) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            UPDATE notifications
            SET
                read = true
            WHERE id = ANY($1)
            RETURNING id
        "#,
        ids
    )
    .fetch_all(pool)
    .await?;

    Ok(())
}

pub async fn send_to_trash(pool: &PgPool, ids: &[i32]) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            UPDATE notifications
            SET
                trash = true
            WHERE id = ANY($1)
            RETURNING id
        "#,
        ids
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}
