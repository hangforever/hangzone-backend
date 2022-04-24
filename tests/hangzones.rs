use hangzone_backend;
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::{Client, LocalResponse};

mod common;

use common::*;

#[rocket::async_test]
async fn get_hangzone_test() {
    let client = create_client().await;
    let token = login(&client).await;
    let name = "my_hangzone_man".to_string();
    create_hangzone(&client, &token, &name).await;
    let slug = slug::slugify(&name);
    let response = client
        .get(format!("/api/hangzones/{}", slug))
        .dispatch()
        .await;
    let body = response_json_value(response).await;
    let h_name = body
        .get("hangzone")
        .and_then(|res| res.get("name"))
        .and_then(|res| res.as_str());
    assert_eq!(h_name, Some(name.as_ref()));
}

#[rocket::async_test]
async fn get_hangzones_test() {
    let client = create_client().await;
    let response = client.get("/api/hangzones").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}

#[rocket::async_test]
async fn create_hangzones_test() {
    let client = create_client().await;
    let token = login(&client).await;
    let name = "my_hangzone".to_string();
    let response = create_hangzone(&client, &token, &name).await;
    assert_eq!(response.status(), Status::Created);
}

async fn create_hangzone<'a>(
    client: &'a Client,
    token: &'a Token,
    name: &str,
) -> LocalResponse<'a> {
    client
        .post("/api/hangzones")
        .header(ContentType::JSON)
        .header(token_header(token.clone()))
        .body(json_string!({
            "name": name,
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
        .await
}
