use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Team {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub name: String,
    pub organization: Option<String>,
    pub contact: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamMembership {
    pub id: Uuid,
    pub team_id: Uuid,
    pub tournament_id: Uuid,
    pub user_id: Uuid,
    pub role: String, // "captain" | "member"
    pub status: String, // "invited" | "accepted" | "declined"
    pub joined_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamInvitation {
    pub id: Uuid,
    pub team_id: Uuid,
    pub tournament_id: Uuid,
    pub email: String,
    pub invited_user_id: Option<Uuid>,
    pub invited_by: Uuid,
    pub status: String, // "pending", "accepted", "declined"
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}