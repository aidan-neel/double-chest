pub struct Refresh {
    pub id: Option<i32>,
    pub token_hash: String,
    pub user_id: i32,
    pub expires_at: i64,
    pub created_at: i64
}