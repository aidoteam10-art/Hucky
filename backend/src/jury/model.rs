use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AssignmentRound {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub status: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AssignmentSubmissionRow {
    pub id: Uuid,
    pub team_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct AssignmentCandidate {
    pub submission_id: Uuid,
    pub excluded_user_ids: HashSet<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedAssignment {
    pub submission_id: Uuid,
    pub jury_user_id: Uuid,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AssignmentDetailRow {
    pub id: Uuid,
    pub status: String,
    pub team_id: Uuid,
    pub team_name: String,
    pub github_url: String,
    pub video_demo_url: String,
    pub live_demo_url: Option<String>,
    pub description: Option<String>,
    pub tournament_id: Uuid,
}
