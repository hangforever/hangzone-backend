use crate::models::user_hangers::UserHangerJson;
use sqlx::postgres::PgPool;
use sqlx::Row;

async fn find_one(
    pool: &PgPool,
    slug: Option<&str>,
    user_hanger_id: Option<i32>,
) -> Option<UserHangerJson> {
    if let Some(s) = slug {
        let row = sqlx::query("SELECT * FROM user_hangers WHERE slug = $1")
            .bind(s)
            .fetch_one(pool)
            .await;
        if let Ok(r) = row {
            return Some(row_to_user_hanger_json(r));
        }
    }
    if let Some(h_id) = user_hanger_id {
        let row = sqlx::query("SELECT * FROM user_hangers WHERE id = $1")
            .bind(h_id)
            .fetch_one(pool)
            .await;
        if let Ok(r) = row {
            return Some(row_to_user_hanger_json(r));
        }
    }
    None
}

#[derive(FromForm)]
struct FindUserHangers {
    search: Option<String>,
    latlng: Option<(f32, f32)>,
}

async fn find(pool: &PgPool, params: FindUserHangers) -> Option<Vec<UserHangerJson>> {
    // TODO: support GPS coordinates with latlng

    if let Some(search) = params.search {
        let user_hangers = sqlx::query("SELECT * FROM user_hangers WHERE name ILIKE $1 || '%'")
            .bind(search)
            .map(|row| row_to_user_hanger_json(row))
            .fetch_all(pool)
            .await;
        match user_hangers {
            Ok(user_hangers) => {
                return Some(user_hangers);
            }
            Err(e) => eprintln!("{}", e),
        }
    }
    None
}

fn row_to_user_hanger_json(row: sqlx::postgres::PgRow) -> UserHangerJson {
    UserHangerJson {
        id: row.get("id"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        alias: row.get("alias"),
        email_address: row.get("email_address"),
        status_hang: row.get("status_hang"),
        status_description: row.get("status_description"),
        icon_url: row.get("icon_url"),
        current_hangzone_id: row.get("current_hangzone_id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
