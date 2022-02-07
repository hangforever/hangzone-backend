use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::Row;

pub struct Hangzone {
  pub id: i32,
  pub slug: String,
  pub name: String,
  pub description: String,
  pub address_1: String,
  pub address_2: String,
  pub address_3: String,
  pub city: String,
  pub state: String,
  pub country: String,
  pub postal_code: String,
  pub geography: (f32, f32),
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HangzoneJson {
  pub id: i32,
  pub slug: String,
  pub name: String,
  pub description: String,
  pub address_1: String,
  pub address_2: String,
  pub address_3: String,
  pub city: String,
  pub state: String,
  pub country: String,
  pub postal_code: String,
  pub geography: (f32, f32),
  pub created_at: String,
  pub updated_at: String,
}
