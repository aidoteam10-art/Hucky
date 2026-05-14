use chrono::{DateTime, Utc};
use sqlx::types::Json;
use uuid::Uuid;

use super::dto::CertificateSnapshot;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct EligibleCertificateRow {
    pub tournament_id: Uuid,
    pub tournament_title: String,
    pub team_id: Uuid,
    pub team_name: String,
    pub user_id: Uuid,
    pub user_name: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GeneratedCertificateRow {
    pub id: Uuid,
    pub snapshot: Json<CertificateSnapshot>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RoundInfoRow {
    pub round_id: Uuid,
    pub title: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RoundScoreRow {
    pub round_id: Uuid,
    pub team_id: Uuid,
    pub team_name: String,
    pub total: f64,
    pub functionality: f64,
    pub requirements: f64,
    pub submitted_at: DateTime<Utc>,
}
