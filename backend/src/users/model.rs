use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub full_name : String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub email : String,
    pub full_name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest{
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

