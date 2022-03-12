use crate::models::hangzones::Hangzone;
use chrono::Utc;
use rocket::serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::Row;

pub async fn find_one(
    pool: &PgPool,
    slug: Option<&str>,
    hangzone_id: Option<i32>,
) -> Option<Hangzone> {
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

pub async fn find(pool: &PgPool, params: FindHangzones) -> Option<Vec<Hangzone>> {
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

#[derive(Deserialize, Debug)]
pub struct HangzoneBody {
    name: String,
    description: Option<String>,
    address_1: String,
    address_2: Option<String>,
    address_3: Option<String>,
    city: String,
    state: Option<String>,
    country: String,
    postal_code: Option<String>,
    lat: f64,
    lng: f64,
}

pub async fn create_one(
    pool: &PgPool,
    hangzone_body: HangzoneBody,
) -> Result<sqlx::postgres::PgRow, sqlx::Error> {
    let slug = String::from("todo");
    sqlx::query(
        "
INSERT INTO hangzones
 (slug, 
  name, 
  description, 
  address_1, 
  address_2, 
  address_3, 
  city,
  state,
  country,
  postal_code,
  geography,
  created_at, 
  updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, ST_SetSRID(ST_MakePoint($11, $12), 4326), $13, $14)
RETURNING id
    ",
    )
    .bind(slug)
    .bind(hangzone_body.name)
    .bind(hangzone_body.description)
    .bind(hangzone_body.address_1)
    .bind(hangzone_body.address_2)
    .bind(hangzone_body.address_3)
    .bind(hangzone_body.city)
    .bind(hangzone_body.state)
    .bind(hangzone_body.country)
    .bind(hangzone_body.postal_code)
    .bind(hangzone_body.lat)
    .bind(hangzone_body.lng)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await
}

fn row_to_hangzone_json(row: sqlx::postgres::PgRow) -> Hangzone {
    Hangzone {
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
