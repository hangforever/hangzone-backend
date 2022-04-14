pub mod friend_requests;
pub mod friends;
pub mod hang_requests;
pub mod hangzones;
pub mod notifications;
pub mod user_hangers;

#[derive(FromForm)]
pub struct PaginationParams {
    page: Option<i64>,
}
