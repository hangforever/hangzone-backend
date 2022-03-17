use hangzone_backend;
use rocket::http::Status;
use rocket::local::asynchronous::Client;

#[rocket::async_test]
async fn get_hangzone_test() {
    let rocket = hangzone_backend::rocket().await;
    let client = Client::tracked(rocket).await.unwrap();
    let response = client.get("/api/hangzones/1").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}
