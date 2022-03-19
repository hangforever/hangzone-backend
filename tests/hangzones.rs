use hangzone_backend;
use hangzone_backend::db::hangzones::HangzoneBody;
use rocket::http::Status;
use rocket::local::asynchronous::Client;

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
    let body = HangzoneBody {
        name: "test_hangzone".to_string(),
        description: None,
        address_1: "a real place 120202".to_string(),
        address_2: None,
        address_3: None,
        city: "Tokyo".to_string(),
        state: None,
        country: "Japan".to_string(),
        postal_code: None,
        lat: 0.0,
        lng: 0.0,
    };
    let rocket = hangzone_backend::rocket().await;
    let client = Client::tracked(rocket).await.unwrap();
    let response = client.post("/api/hangzones").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}
