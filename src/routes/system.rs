use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::models::device::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/heartbeat", post(heartbeat))
        .route("/api/system/heartbeat", post(heartbeat))
        .route("/api/system/sysinfo", post(sysinfo))
        .route("/api/audit", post(audit))
}

async fn heartbeat(
    State(state): State<AppState>,
    Json(req): Json<HeartbeatRequest>,
) -> Result<Json<Value>, ApiError> {
    if !req.id.is_empty() {
        sqlx::query(
            "INSERT INTO devices (rustdesk_id, last_online)
             VALUES (?, CURRENT_TIMESTAMP)
             ON CONFLICT(rustdesk_id) DO UPDATE SET last_online = CURRENT_TIMESTAMP",
        )
        .bind(&req.id)
        .execute(&state.db)
        .await?;
    }

    // The client expects a modified_at field back
    Ok(Json(json!({ "modified_at": "" })))
}

async fn sysinfo(
    State(state): State<AppState>,
    Json(req): Json<SysinfoRequest>,
) -> Result<Json<Value>, ApiError> {
    if !req.id.is_empty() {
        sqlx::query(
            "INSERT INTO devices (rustdesk_id, hostname, platform, os, cpu, memory, version, last_online)
             VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
             ON CONFLICT(rustdesk_id) DO UPDATE SET
                 hostname = excluded.hostname,
                 platform = excluded.platform,
                 os = excluded.os,
                 cpu = excluded.cpu,
                 memory = excluded.memory,
                 version = excluded.version,
                 last_online = CURRENT_TIMESTAMP",
        )
        .bind(&req.id)
        .bind(&req.hostname)
        .bind(&req.platform)
        .bind(&req.os)
        .bind(&req.cpu)
        .bind(&req.memory)
        .bind(&req.version)
        .execute(&state.db)
        .await?;
    }

    Ok(Json(json!({})))
}

async fn audit(
    State(state): State<AppState>,
    Json(req): Json<AuditRequest>,
) -> Result<Json<Value>, ApiError> {
    let rustdesk_id = if req.rustdesk_id.is_empty() {
        &req.id
    } else {
        &req.rustdesk_id
    };

    sqlx::query(
        "INSERT INTO audit_log (action, rustdesk_id, peer_id, ip, note) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&req.action)
    .bind(rustdesk_id)
    .bind(&req.peer_id)
    .bind(&req.ip)
    .bind(&req.note)
    .execute(&state.db)
    .await?;

    Ok(Json(json!({})))
}
