use serde::{Deserialize, Serialize};

/// Database row for a user.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub status: i32,
    pub created_at: String,
    pub updated_at: String,
}

/// RustDesk client login request.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    /// Client sends its RustDesk ID here.
    #[serde(default)]
    pub id: String,
    /// Client sends its UUID here.
    #[serde(default)]
    pub uuid: String,
}

/// RustDesk client login response — must match this exact shape.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    #[serde(rename = "type")]
    pub token_type: String,
    pub user: UserPayload,
}

/// User info returned to the client.
#[derive(Debug, Serialize)]
pub struct UserPayload {
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub note: String,
}

/// Admin API: create/update user request.
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub is_admin: bool,
}

/// Admin API: user list response item.
#[derive(Debug, Serialize)]
pub struct UserListItem {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub status: i32,
    pub created_at: String,
}
