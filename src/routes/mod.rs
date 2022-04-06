pub mod friends;
pub mod hangzones;
pub mod notifications;
pub mod requests;
pub mod user_hangers;

#[derive(FromForm)]
pub struct PaginationParams {
    page: Option<i64>,
}
