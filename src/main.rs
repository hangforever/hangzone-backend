use hangzone_backend;
use rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("Starting hangzone backend server...");
    hangzone_backend::rocket().await.launch().await
}
