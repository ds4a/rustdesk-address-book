use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_db_path")]
    pub db_path: String,
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
    #[serde(default = "default_admin_username")]
    pub admin_username: String,
    #[serde(default = "default_admin_password")]
    pub admin_password: String,
    #[serde(default = "default_token_expiry_hours")]
    pub token_expiry_hours: u64,
}

fn default_port() -> u16 {
    21114
}
fn default_db_path() -> String {
    "data/db.sqlite3".to_string()
}
fn default_jwt_secret() -> String {
    use rand::Rng;
    let secret: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    tracing::warn!("No JWT secret configured — using random secret (sessions won't survive restarts)");
    secret
}
fn default_admin_username() -> String {
    "admin".to_string()
}
fn default_admin_password() -> String {
    "admin".to_string()
}
fn default_token_expiry_hours() -> u64 {
    168 // 7 days
}

impl Config {
    pub fn load() -> Self {
        // Try loading from config.toml, fall back to defaults
        let config_path = PathBuf::from("config.toml");
        let mut config: Config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .expect("Failed to read config.toml");
            toml::from_str(&content).expect("Failed to parse config.toml")
        } else {
            toml::from_str("").unwrap()
        };

        // Environment variables override config file
        if let Ok(v) = std::env::var("RUSTDESK_AB_PORT") {
            config.port = v.parse().expect("Invalid RUSTDESK_AB_PORT");
        }
        if let Ok(v) = std::env::var("RUSTDESK_AB_DB_PATH") {
            config.db_path = v;
        }
        if let Ok(v) = std::env::var("RUSTDESK_AB_JWT_SECRET") {
            config.jwt_secret = v;
        }
        if let Ok(v) = std::env::var("RUSTDESK_AB_ADMIN_USERNAME") {
            config.admin_username = v;
        }
        if let Ok(v) = std::env::var("RUSTDESK_AB_ADMIN_PASSWORD") {
            config.admin_password = v;
        }

        config
    }
}
