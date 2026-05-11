use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub organization: Option<String>,
    pub contact: Option<String>,
    #[serde(default)]
    pub member_emails: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInvitationRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTeamRequest {
    pub name: String,
    pub organization: Option<String>,
    pub contact: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TeamSummaryResponse {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub name: String,
    pub organization: Option<String>,
    pub contact: Option<String>,
    pub members_count: i64,
    pub current_user_role: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TournamentTeamsResponse {
    pub items: Vec<TournamentTeamItem>,
}

#[derive(Debug, Serialize)]
pub struct TournamentTeamItem {
    pub id: Uuid,
    pub name: String,
    pub organization: Option<String>,
    pub members_count: i64,
    pub captain: Option<TeamCaptainPreview>,
}

#[derive(Debug, Serialize)]
pub struct TeamCaptainPreview {
    pub id: Uuid,
    pub full_name: String,
}

#[derive(Debug, Serialize)]
pub struct TeamDetailResponse {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub name: String,
    pub organization: Option<String>,
    pub contact: Option<String>,
    pub members: Vec<TeamMemberResponse>,
    pub pending_invitations: Vec<TeamInvitationResponse>,
}

#[derive(Debug, Serialize)]
pub struct TeamMemberResponse {
    pub user_id: Uuid,
    pub full_name: String,
    pub email: String,
    pub role: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct TeamInvitationResponse {
    pub id: Uuid,
    pub team_id: Uuid,
    pub tournament_id: Uuid,
    pub email: String,
    pub status: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct MyInvitationsResponse {
    pub items: Vec<MyInvitationItem>,
}

#[derive(Debug, Serialize)]
pub struct MyInvitationItem {
    pub id: Uuid,
    pub team_id: Uuid,
    pub team_name: String,
    pub tournament_id: Uuid,
    pub tournament_title: String,
    pub invited_by: InvitedByResponse,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct InvitedByResponse {
    pub id: Uuid,
    pub full_name: String,
}

#[derive(Debug, Serialize)]
pub struct MyTeamsResponse {
    pub items: Vec<MyTeamItem>,
}

#[derive(Debug, Serialize)]
pub struct MyTeamItem {
    pub team_id: Uuid,
    pub team_name: String,
    pub role: String,
    pub status: String,
    pub tournament: MyTeamTournament,
    pub members_count: i64,
}

#[derive(Debug, Serialize)]
pub struct MyTeamTournament {
    pub id: Uuid,
    pub title: String,
    pub status: String,
}
