use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::{ActiveRoundPreview, TournamentListItem, TournamentStatus};

#[derive(Debug, Deserialize)]
pub struct CreateTournamentRequest {
    pub title: String,
    pub description: String,
    pub rules: String,
    pub registration_starts_at: DateTime<Utc>,
    pub registration_ends_at: DateTime<Utc>,
    pub starts_at: DateTime<Utc>,
    pub max_teams: Option<i32>,
    pub first_round: CreateFirstRoundRequest,
}

#[derive(Debug, Deserialize)]
pub struct CreateFirstRoundRequest {
    pub title: String,
    pub task_description: String,
    pub technology_requirements: Option<String>,
    #[serde(default)]
    pub must_have: Vec<String>,
    pub starts_at: DateTime<Utc>,
    pub deadline_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CreateTournamentResponse {
    pub id: Uuid,
    pub status: TournamentStatus,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct TournamentListQuery {
    pub status: Option<TournamentStatus>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct TournamentListResponse {
    pub items: Vec<TournamentListItem>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct TournamentDetailResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub rules: String,
    pub status: TournamentStatus,
    pub registration_starts_at: DateTime<Utc>,
    pub registration_ends_at: DateTime<Utc>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
    pub max_teams: Option<i32>,
    pub registered_teams_count: i64,
    pub registered_teams: Vec<RegisteredTeamPreview>,
    pub active_round: Option<ActiveRoundPreview>,
}

#[derive(Debug, Serialize)]
pub struct RegisteredTeamPreview {
    pub id: Uuid,
    pub name: String,
    pub members_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTournamentRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub rules: Option<String>,
    pub registration_starts_at: Option<DateTime<Utc>>,
    pub registration_ends_at: Option<DateTime<Utc>>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub max_teams: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ChangeTournamentStatusRequest {
    pub status: TournamentStatus,
}
