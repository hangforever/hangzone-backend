use super::pagination::Paginate;
use super::user_hangers::row_to_user_hanger_json;
use crate::constants::SEARCH_METERS;
use crate::models::hangzones::Hangzone;
use crate::position::Position;
use chrono::Utc;
use rocket::serde::Deserialize;
use slug;
use sqlx::postgres::PgPool;
use sqlx::Row;

#[derive(Deserialize, Debug)]
pub struct HangzoneBody {
    pub name: String,
    pub description: Option<String>,
    pub address_1: String,
    pub address_2: Option<String>,
    pub address_3: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub country: String,
    pub postal_code: Option<String>,
    pub lat: f64,
    pub lng: f64,
}

pub async fn find_one(
    pool: &PgPool,
    slug: Option<&str>,
    hangzone_id: Option<i32>,
) -> Option<Hangzone> {
    if let Some(s) = slug {
        let hangzone = sqlx::query_as!(
                Hangzone,
                "SELECT id, slug, name, description, address_1, address_2, address_3, city, country, ST_AsGeoJson(geography) as geography, postal_code, state, created_at, updated_at FROM hangzones WHERE slug = $1",
                s
            )
            .fetch_one(pool)
            .await;
        if let Ok(hangzone) = hangzone {
            return Some(hangzone);
        }
    }
    if let Some(h_id) = hangzone_id {
        let hangzone = sqlx::query_as!(Hangzone, "SELECT id, slug, name, description, address_1, address_2, address_3, city, country, ST_AsGeoJson(geography) as geography, postal_code, state, created_at, updated_at FROM hangzones WHERE id = $1", h_id)
            .fetch_one(pool)
            .await;
        if let Ok(hangzone) = hangzone {
            return Some(hangzone);
        }
    }
    None
}

pub async fn find(
    pool: &PgPool,
    pos: Option<Position>,
    search: Option<String>,
    page: Option<i64>,
) -> Option<Vec<Hangzone>> {
    if let Some(pos) = pos {
        if let Ok(hangzones) = find_by_pos(pool, &pos).await {
            return Some(hangzones);
        }
    }
    if let Some(search) = search {
        let page = page.unwrap_or(1);
        let query = "SELECT id, slug, name, description, address_1, address_2, address_3, city, country, ST_AsGeoJson(geography) as geography, postal_code, state, created_at, updated_at FROM hangzones WHERE name ILIKE $1 || '%'";
        let pagination = query.paginate(page);
        let hangzones = sqlx::query(&pagination.paginated_query())
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

pub async fn find_by_pos(pool: &PgPool, pos: &Position) -> Result<Vec<Hangzone>, sqlx::Error> {
    let hangzones = sqlx::query_as!(
        Hangzone,
        "
SELECT id, slug, name, description, address_1, address_2, address_3, city, country, ST_AsGeoJson(geography) as geography, postal_code, state, created_at, updated_at
FROM hangzones 
WHERE ST_DWithin(geography, ST_SetSRID(ST_MakePoint($1, $2), 4326), $3)
        ",
        pos.lng,
        pos.lat,
        SEARCH_METERS,
    )
        .fetch_all(pool)
        .await?;
    Ok(hangzones)
}

pub async fn create_one(
    pool: &PgPool,
    hangzone_body: HangzoneBody,
) -> Result<sqlx::postgres::PgRow, sqlx::Error> {
    let slug = slug::slugify(&hangzone_body.name);
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
        state: row.get("state"),
        country: row.get("country"),
        postal_code: row.get("postal_code"),
        geography: row.get("geography"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
