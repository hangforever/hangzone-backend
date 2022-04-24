use rocket::serde::json::Value;
use rocket::{
    http::{ContentType, Header, Status},
    local::asynchronous::{Client, LocalResponse},
};

pub const USERNAME: &'static str = "hangin_guy";
pub const EMAIL: &'static str = "hangin_guy@protonmail.com";
pub const PASSWORD: &'static str = "hang_to_the_zone";

/// Utility macro for turning `json!` into string.
#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
    };
}

pub type Token = String;

pub fn token_header(token: Token) -> Header<'static> {
    Header::new("authorization", format!("Token {}", token))
}

pub async fn create_client() -> Client {
    let rocket = hangzone_backend::rocket().await;
    Client::tracked(rocket)
        .await
        .expect("Couldn't create rocket test client")
}

// Retrieve a token registering a user if required
pub async fn login(client: &Client) -> Token {
    println!("Attempting to login");
    let res = try_login(client).await;
    if let Some(token) = res {
        println!("Got token: {}", token);
        return token;
    }
    println!("No token. Registering...");
    register(client, USERNAME, EMAIL, PASSWORD).await;
    try_login(client)
        .await
        .expect("could not register a new user and login")
}

/// Helper function for converting response to json value.
pub async fn response_json_value<'a>(response: LocalResponse<'a>) -> Value {
    let body = response.into_string().await.unwrap();
    serde_json::from_str(&body).expect("can't parse value")
}

async fn try_login(client: &Client) -> Option<Token> {
    let response = client
        .post("/api/users/login")
        .header(ContentType::JSON)
        .body(json_string!({"user_hanger": {"email": EMAIL, "password": PASSWORD}}))
        .dispatch()
        .await;
    println!("status: {}", response.status());
    if response.status() == Status::NotFound {
        println!("Not found status returning None: {}", response.status());
        return None;
    }

    let value = response_json_value(response).await;
    println!("value: {}", value);
    let token = value
        .get("user_hanger")
        .and_then(|user| user.get("token"))
        .and_then(|token| token.as_str())
        .map(String::from)
        .expect("Cannot extract token");
    Some(token)
}

async fn register(client: &Client, username: &str, email: &str, password: &str) {
    let response = client
        .post("/api/users")
        .header(ContentType::JSON)
        .body(json_string!({ "alias": username, "email_address": email, "password": password }))
        .dispatch()
        .await;
}
