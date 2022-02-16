use sqlx::PgPool;
use std::env;

pub mod hangzones;

pub async fn get_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_default(String::from("DATABASE_URL=postgres://@localhost/hangzone"));
    sqlx::PgPool::connect(database_url)
}
