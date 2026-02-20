use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Peer {
    pub id: i64,
    pub ab_guid: String,
    pub rustdesk_id: String,
    pub hash: String,
    pub username: String,
    pub hostname: String,
    pub platform: String,
    pub alias: String,
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Peer as returned to the RustDesk client.
#[derive(Debug, Serialize)]
pub struct PeerPayload {
    pub id: String,
    pub hash: String,
    pub username: String,
    pub hostname: String,
    pub platform: String,
    pub alias: String,
    pub tags: Vec<String>,
    pub note: String,
}

/// Request to add a peer.
#[derive(Debug, Deserialize)]
pub struct AddPeerRequest {
    pub id: String,
    #[serde(default)]
    pub hash: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub hostname: String,
    #[serde(default)]
    pub platform: String,
    #[serde(default)]
    pub alias: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub note: String,
}

/// Request to update a peer.
#[derive(Debug, Deserialize)]
pub struct UpdatePeerRequest {
    #[serde(default)]
    pub hash: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub platform: Option<String>,
    #[serde(default)]
    pub alias: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub note: Option<String>,
}

/// Response for GET /api/ab/peers.
#[derive(Debug, Serialize)]
pub struct PeersResponse {
    pub data: Vec<PeerPayload>,
    pub total: i64,
}

/// Request to delete peers (can be batch).
#[derive(Debug, Deserialize)]
pub struct DeletePeersRequest {
    #[serde(default)]
    pub ids: Vec<String>,
    /// Single ID variant (some client versions send this).
    #[serde(default)]
    pub id: Option<String>,
}
