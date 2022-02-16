use hangzone_backend;
use rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    hangzone_backend::rocket().await.launch().await
}
