use rocket::serde::Deserialize;

#[derive(Deserialize, FromForm, Debug)]
pub struct Position {
    pub lat: f64,
    pub lng: f64,
}
