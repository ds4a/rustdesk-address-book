use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Tag {
    pub id: i64,
    pub ab_guid: String,
    pub name: String,
    pub color: i64,
}

/// Tag as returned to the RustDesk client.
#[derive(Debug, Serialize)]
pub struct TagPayload {
    pub name: String,
    pub color: i64,
}

/// Response for GET /api/ab/tags/{guid}.
#[derive(Debug, Serialize)]
pub struct TagsResponse {
    pub data: Vec<TagPayload>,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct AddTagRequest {
    pub name: String,
    #[serde(default = "default_color")]
    pub color: i64,
}

fn default_color() -> i64 {
    4278190080 // 0xFF000000 (black)
}

#[derive(Debug, Deserialize)]
pub struct RenameTagRequest {
    pub old: String,
    pub new: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTagColorRequest {
    pub name: String,
    pub color: i64,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTagRequest {
    #[serde(default)]
    pub names: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
}
