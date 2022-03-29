pub async fn find_one(pool: &PgPool, user_hanger_id: i32) -> Option<Friend> {
    sqlx::query_as!(
        "
            SELECT 
        "
    )
}
