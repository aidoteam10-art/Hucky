use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountRole {
    Participant,
    Organiser,
    Jury,
    Admin,
}

impl AccountRole {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountRole::Participant => "participant",
            AccountRole::Organiser => "organiser",
            AccountRole::Jury => "jury",
            AccountRole::Admin => "admin",
        }
    }

    pub fn can_create_tournaments(self) -> bool {
        matches!(self, AccountRole::Organiser)
    }
}

impl FromStr for AccountRole {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "participant" => Ok(AccountRole::Participant),
            "organiser" => Ok(AccountRole::Organiser),
            "jury" => Ok(AccountRole::Jury),
            "admin" => Ok(AccountRole::Admin),
            _ => Err(()),
        }
    }
}

impl fmt::Display for AccountRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub account_role: String,
    pub avatar_url: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub full_name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}
