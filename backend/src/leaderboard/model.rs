use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LeaderboardTournament {
    pub id: Uuid,
    pub title: String,
    pub status: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LeaderboardScoreRow {
    pub team_id: Uuid,
    pub team_name: String,
    pub organization: Option<String>,
    pub code: String,
    pub weighted_score: f64,
    pub submitted_at: DateTime<Utc>,
    pub reviews_count: i64,
}

#[derive(Debug, Clone)]
pub struct TeamScore {
    pub team_id: Uuid,
    pub team_name: String,
    pub organization: Option<String>,
    pub scores: HashMap<String, f64>,
    pub total: f64,
    pub functionality: f64,
    pub requirements: f64,
    pub submitted_at: DateTime<Utc>,
    pub reviews_count: i64,
}
