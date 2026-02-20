use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // username
    pub user_id: i64,
    pub is_admin: bool,
    pub exp: usize,       // expiry timestamp
    pub iat: usize,       // issued at
}

pub fn create_token(
    username: &str,
    user_id: i64,
    is_admin: bool,
    secret: &str,
    expiry_hours: u64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: username.to_string(),
        user_id,
        is_admin,
        exp: (now + chrono::Duration::hours(expiry_hours as i64)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn validate_token(
    token: &str,
    secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
