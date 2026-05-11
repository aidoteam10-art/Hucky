use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AddJuryRequest {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct JuryListResponse {
    pub items: Vec<JuryListItem>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct JuryListItem {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateAssignmentsRequest {
    pub reviews_per_submission: i64,
    pub max_assignments_per_jury: i64,
}

#[derive(Debug, Serialize)]
pub struct GenerateAssignmentsResponse {
    pub created_assignments: usize,
}

#[derive(Debug, Serialize)]
pub struct AssignmentListResponse {
    pub items: Vec<AssignmentListItem>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AssignmentListItem {
    pub id: Uuid,
    pub status: String,
    pub team_name: String,
    pub tournament_title: String,
    pub round_title: String,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AssignmentDetailResponse {
    pub id: Uuid,
    pub status: String,
    pub team: AssignmentTeam,
    pub submission: AssignmentSubmission,
    pub criteria: Vec<AssignmentCriterion>,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AssignmentTeam {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct AssignmentSubmission {
    pub github_url: String,
    pub video_demo_url: String,
    pub live_demo_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AssignmentCriterion {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: String,
    pub max_score: i32,
    pub score: Option<i32>,
}
