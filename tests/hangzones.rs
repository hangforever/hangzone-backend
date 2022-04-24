use hangzone_backend;
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;

mod common;

use common::*;

#[rocket::async_test]
async fn get_hangzone_test() {
    let rocket = hangzone_backend::rocket().await;
    let client = Client::tracked(rocket).await.unwrap();
    let response = client.get("/api/hangzones/1").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}

#[rocket::async_test]
async fn get_hangzones_test() {
    let rocket = hangzone_backend::rocket().await;
    let client = Client::tracked(rocket).await.unwrap();
    let response = client.get("/api/hangzones").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}

#[rocket::async_test]
async fn create_hangzones_test() {
    let rocket = hangzone_backend::rocket().await;
    let client = Client::tracked(rocket).await.unwrap();
    let token = login(&client).await;
    let response = client
        .post("/api/hangzones")
        .header(ContentType::JSON)
        .header(token_header(token))
        .body(json_string!({
            "name": "test_hangzone",
            "description": null,
            "address_1": "a real place 120202",
            "address_2": null,
            "address_3": null,
            "city": "Tokyo",
            "state": null,
            "country": "Japan",
            "postal_code": null,
            "lat": 0.0,
            "lng": 0.0
        }))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Created);
}
