use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    pub id: Uuid,
    #[validate(email)]
    pub email: String,
    pub password_hash: String,
    pub username: String,
    pub balance: i64, // Store balance in satoshis
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 3, max = 30))]
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
} 