use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::{RoundListItem, RoundRequirement, RoundStatus};

#[derive(Debug, Deserialize)]
pub struct CreateRoundRequest {
    pub title: String,
    pub task_description: String,
    pub technology_requirements: Option<String>,
    #[serde(default)]
    pub must_have: Vec<String>,
    pub starts_at: DateTime<Utc>,
    pub deadline_at: DateTime<Utc>,
    pub position: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoundRequest {
    pub title: Option<String>,
    pub task_description: Option<String>,
    pub technology_requirements: Option<String>,
    pub must_have: Option<Vec<String>>,
    pub starts_at: Option<DateTime<Utc>>,
    pub deadline_at: Option<DateTime<Utc>>,
    pub position: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ChangeRoundStatusRequest {
    pub status: RoundStatus,
}

#[derive(Debug, Serialize)]
pub struct RoundListResponse {
    pub items: Vec<RoundListItem>,
}

#[derive(Debug, Serialize)]
pub struct RoundDetailResponse {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub title: String,
    pub task_description: String,
    pub technology_requirements: Option<String>,
    pub status: RoundStatus,
    pub starts_at: DateTime<Utc>,
    pub deadline_at: DateTime<Utc>,
    pub time_left_seconds: i64,
    pub position: i32,
    pub must_have: Vec<RoundRequirement>,
}
