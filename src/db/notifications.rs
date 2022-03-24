use crate::models::notifications::{Notification, NotificationType};
use sqlx::postgres::PgPool;
use sqlx::Row;

pub async fn find(pool: &PgPool, user_hanger_id: i32) -> Result<Vec<Notification>, sqlx::Error> {
    sqlx::query_as!(
        Notification,
        r#"
            SELECT id, user_hanger_id, notification_type as "notification_type: NotificationType", read, trash, created_at, updated_at, content 
            FROM notifications 
            WHERE user_hanger_id = $1
        "#,
        user_hanger_id
    )
    .fetch_all(pool)
    .await
}
