use crate::models::hangzones::HangzoneJson;
use sqlx::postgres::PgPool;
use sqlx::Row;

pub async fn find_one(
    pool: &PgPool,
    slug: Option<&str>,
    hangzone_id: Option<i32>,
) -> Option<HangzoneJson> {
    if let Some(s) = slug {
        let row = sqlx::query("SELECT * FROM hangzones WHERE slug = $1")
            .bind(s)
            .fetch_one(pool)
            .await;
        if let Ok(r) = row {
            return Some(row_to_hangzone_json(r));
        }
    }
    if let Some(h_id) = hangzone_id {
        let row = sqlx::query("SELECT * FROM hangzones WHERE id = $1")
            .bind(h_id)
            .fetch_one(pool)
            .await;
        if let Ok(r) = row {
            return Some(row_to_hangzone_json(r));
        }
    }
    None
}

#[derive(FromForm)]
pub struct FindHangzones {
    search: Option<String>,
    latlng: Option<(f32, f32)>,
}

pub async fn find(pool: &PgPool, params: FindHangzones) -> Option<Vec<HangzoneJson>> {
    // TODO: support GPS coordinates with latlng

    if let Some(search) = params.search {
        let hangzones = sqlx::query("SELECT * FROM hangzones WHERE name ILIKE $1 || '%'")
            .bind(search)
            .map(|row| row_to_hangzone_json(row))
            .fetch_all(pool)
            .await;
        match hangzones {
            Ok(hangzones) => {
                return Some(hangzones);
            }
            Err(e) => eprintln!("{}", e),
        }
    }
    None
}

fn row_to_hangzone_json(row: sqlx::postgres::PgRow) -> HangzoneJson {
    HangzoneJson {
        id: row.get("id"),
        slug: row.get("slug"),
        name: row.get("name"),
        description: row.get("description"),
        address_1: row.get("address_1"),
        address_2: row.get("address_2"),
        address_3: row.get("address_3"),
        city: row.get("city"),
        // state: row.get("state"),
        // country: row.get("country"),
        // postal_code: row.get("postal_code"),
        // geography: row.get("geography"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
