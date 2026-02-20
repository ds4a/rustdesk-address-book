use axum::{
    extract::{Path, State},
    routing::{get, put},
    Json, Router,
};
use serde_json::{json, Value};

use crate::auth::middleware::AuthUser;
use crate::auth::password::hash_password;
use crate::error::ApiError;
use crate::models::user::*;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/users/{id}", put(update_user).delete(delete_user))
}

fn require_admin(claims: &crate::auth::jwt::Claims) -> Result<(), ApiError> {
    if !claims.is_admin {
        return Err(ApiError::Forbidden("Admin access required".to_string()));
    }
    Ok(())
}

async fn list_users(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
) -> Result<Json<Value>, ApiError> {
    require_admin(&claims)?;

    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY id")
        .fetch_all(&state.db)
        .await?;

    let items: Vec<UserListItem> = users
        .into_iter()
        .map(|u| UserListItem {
            id: u.id,
            username: u.username,
            name: u.name,
            email: u.email,
            is_admin: u.is_admin,
            status: u.status,
            created_at: u.created_at,
        })
        .collect();

    let total = items.len();
    Ok(Json(json!({ "data": items, "total": total })))
}

async fn create_user(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<Value>, ApiError> {
    require_admin(&claims)?;

    let password_hash = hash_password(&req.password)
        .map_err(|e| ApiError::Internal(format!("Failed to hash password: {}", e)))?;

    sqlx::query(
        "INSERT INTO users (username, password_hash, name, email, is_admin) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&req.username)
    .bind(&password_hash)
    .bind(&req.name)
    .bind(&req.email)
    .bind(req.is_admin)
    .execute(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.message().contains("UNIQUE") => {
            ApiError::Conflict(format!("Username '{}' already exists", req.username))
        }
        _ => ApiError::Internal(e.to_string()),
    })?;

    Ok(Json(json!({})))
}

async fn update_user(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    require_admin(&claims)?;

    // Allow partial updates
    if let Some(name) = req.get("name").and_then(|v| v.as_str()) {
        sqlx::query("UPDATE users SET name = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(&state.db)
            .await?;
    }
    if let Some(email) = req.get("email").and_then(|v| v.as_str()) {
        sqlx::query("UPDATE users SET email = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(email)
            .bind(id)
            .execute(&state.db)
            .await?;
    }
    if let Some(is_admin) = req.get("is_admin").and_then(|v| v.as_bool()) {
        sqlx::query("UPDATE users SET is_admin = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(is_admin)
            .bind(id)
            .execute(&state.db)
            .await?;
    }
    if let Some(status) = req.get("status").and_then(|v| v.as_i64()) {
        sqlx::query("UPDATE users SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(status as i32)
            .bind(id)
            .execute(&state.db)
            .await?;
    }
    if let Some(password) = req.get("password").and_then(|v| v.as_str()) {
        if !password.is_empty() {
            let hash = hash_password(password)
                .map_err(|e| ApiError::Internal(format!("Failed to hash password: {}", e)))?;
            sqlx::query(
                "UPDATE users SET password_hash = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
            )
            .bind(&hash)
            .bind(id)
            .execute(&state.db)
            .await?;
        }
    }

    Ok(Json(json!({})))
}

async fn delete_user(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    require_admin(&claims)?;

    if id == claims.user_id {
        return Err(ApiError::BadRequest("Cannot delete yourself".to_string()));
    }

    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({})))
}
