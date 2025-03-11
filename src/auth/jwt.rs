use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // User ID
    pub exp: i64,  // Expiration time
    pub iat: i64,  // Issued at
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24); // Token expires in 24 hours
        
        Self {
            sub: user_id,
            iat: now.timestamp(),
            exp: expires_at.timestamp(),
        }
    }
}

pub fn create_token(user_id: Uuid, secret: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn validate_token(token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
} 