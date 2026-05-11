use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SubmissionRequest {
    pub github_url: String,
    pub video_demo_url: String,
    pub live_demo_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SubmissionResponse {
    pub id: Uuid,
    pub round_id: Uuid,
    pub team_id: Uuid,
    pub github_url: String,
    pub video_demo_url: String,
    pub live_demo_url: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub submitted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub locked_at: Option<DateTime<Utc>>,
}
