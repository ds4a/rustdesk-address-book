use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;

pub async fn init_pool(db_path: &str) -> SqlitePool {
    // Ensure parent directory exists
    if let Some(parent) = Path::new(db_path).parent() {
        std::fs::create_dir_all(parent).expect("Failed to create database directory");
    }

    let db_url = format!("sqlite:{}?mode=rwc", db_path);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    // Enable WAL mode for better concurrent read performance
    sqlx::query("PRAGMA journal_mode=WAL")
        .execute(&pool)
        .await
        .expect("Failed to set WAL mode");

    // Enable foreign keys
    sqlx::query("PRAGMA foreign_keys=ON")
        .execute(&pool)
        .await
        .expect("Failed to enable foreign keys");

    pool
}

pub async fn run_migrations(pool: &SqlitePool) {
    let migrations_dir = Path::new("migrations");
    if !migrations_dir.exists() {
        tracing::warn!("No migrations directory found");
        return;
    }

    let mut entries: Vec<_> = std::fs::read_dir(migrations_dir)
        .expect("Failed to read migrations directory")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "sql")
                .unwrap_or(false)
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    // Create migrations tracking table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS _migrations (
            name TEXT PRIMARY KEY,
            applied_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(pool)
    .await
    .expect("Failed to create migrations table");

    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();

        let already_applied: bool =
            sqlx::query_scalar("SELECT COUNT(*) > 0 FROM _migrations WHERE name = ?")
                .bind(&name)
                .fetch_one(pool)
                .await
                .unwrap_or(false);

        if already_applied {
            continue;
        }

        let sql = std::fs::read_to_string(entry.path())
            .unwrap_or_else(|_| panic!("Failed to read migration: {}", name));

        tracing::info!("Applying migration: {}", name);

        // Execute each statement separately (SQLite doesn't support multi-statement execute well)
        for statement in sql.split(';') {
            let trimmed = statement.trim();
            if trimmed.is_empty() {
                continue;
            }
            sqlx::query(trimmed)
                .execute(pool)
                .await
                .unwrap_or_else(|e| panic!("Failed to apply migration {}: {}", name, e));
        }

        sqlx::query("INSERT INTO _migrations (name) VALUES (?)")
            .bind(&name)
            .execute(pool)
            .await
            .unwrap_or_else(|_| panic!("Failed to record migration: {}", name));
    }
}
