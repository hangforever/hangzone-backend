use rocket::local::asynchronous::{Client, LocalResponse};

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

// Retrieve a token registering a user if required
fn login(client: &Client) -> Token {
    try_login(client).unwrap_or_else(|| {
        register();
        try_login(client).expect("could not register a new user and login")
    })
}

fn try_login(client: &Client) -> Option<Token> {
    None
}
