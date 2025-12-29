pub struct User {
    pub id: Option<i32>,
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: i64,
    pub updated_at: i64,
}