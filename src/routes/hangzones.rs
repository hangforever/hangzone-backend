use rocket_contrib::json::{Json, JsonValue};

#[get("/hangzones")]
pub fn get_hangzones() -> JsonValue {
  println!("Not implemented!");
  json!({
    "hangzones": [
      {
        "placeholder": "hello"
      }
    ]
  })
}
