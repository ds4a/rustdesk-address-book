use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub note: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct GroupListItem {
    pub id: i64,
    pub name: String,
    pub note: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    #[serde(default)]
    pub note: String,
}
