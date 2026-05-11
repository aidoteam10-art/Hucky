use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Submission {
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

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SubmissionRound {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub status: String,
    pub deadline_at: DateTime<Utc>,
}

pub struct SubmissionInput<'a> {
    pub github_url: &'a str,
    pub video_demo_url: &'a str,
    pub live_demo_url: Option<&'a str>,
    pub description: Option<&'a str>,
}
