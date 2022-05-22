use sqlx::PgPool;
use std::env;

pub mod friends;
pub mod hang_sessions;
pub mod hangers;
pub mod hangzones;
pub mod notifications;
pub mod pagination;
pub mod request_friends;
pub mod request_hangs;
pub mod user_hangers;

pub async fn get_pool() -> Result<PgPool, sqlx::Error> {
    let database_url =
        env::var("DATABASE_URL").unwrap_or(String::from("postgres://@localhost/hangzone"));
    println!("connecting with database url: {}", database_url);
    sqlx::PgPool::connect(&database_url).await
}
