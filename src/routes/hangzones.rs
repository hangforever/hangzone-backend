use crate::db;
use crate::models::hangzones::HangzoneJson;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::postgres::PgPool;

#[get("/hangzones/<slug>")]
pub async fn get_hangzone(slug: String, pool: &State<PgPool>) -> Json<HangzoneJson> {
  let hangzone = db::hangzones::find_one(&pool.inner(), Some(&slug), None)
    .await
    .unwrap();
  Json(hangzone)
}
