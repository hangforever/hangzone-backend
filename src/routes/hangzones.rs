use super::PaginationParams;
use crate::auth::Auth;
use crate::db;
use crate::db::hangzones::{HangzoneBody, Position};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use sqlx::postgres::PgPool;

#[get("/hangzones?<search>&<pos>&<pagination>")]
pub async fn get_hangzones(
    search: Option<String>,
    pos: Option<Position>,
    pagination: PaginationParams,
    pool: &State<PgPool>,
) -> Value {
    println!("latlng: {:?}", pos);
    let hangzones = db::hangzones::find(pool, pos, search, pagination.page).await;

    json!({ "hangzones": hangzones })
}

#[post("/hangzones", data = "<hangzone_body>")]
pub async fn create_hangzone(
    auth: Auth,
    hangzone_body: Json<HangzoneBody>,
    pool: &State<PgPool>,
) -> Result<Status, Status> {
    match db::hangzones::create_one(pool, hangzone_body.into_inner()).await {
        Ok(_res) => Ok(Status::Created),
        Err(e) => {
            eprintln!("Could not create hangzone: {}", e.to_string());
            Err(Status::UnprocessableEntity)
        }
    }
}

#[get("/hangzones/<slug>")]
pub async fn get_hangzone(slug: String, pool: &State<PgPool>) -> Value {
    let hangzone = db::hangzones::find_one(pool, Some(&slug), None).await;

    if let Some(hangzone) = hangzone {
        let res = db::user_hangers::find(pool, &slug).await.map_err(|err| {
            eprint!("Couldn't get user profiles: {}", err);
        });
        if let Ok(user_hangers) = res {
            let profiles: Vec<_> = user_hangers.iter().map(|uh| uh.to_profile()).collect();
            return json!({
                "hangzone": hangzone,
                "profiles": profiles,
            });
        }
    }
    json!({ "hangzone": null })
}
