use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeamRole {
    Captain,
    Member,
}

impl TeamRole {
    pub fn as_str(self) -> &'static str {
        match self {
            TeamRole::Captain => "captain",
            TeamRole::Member => "member",
        }
    }
}

impl fmt::Display for TeamRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipStatus {
    Invited,
    Accepted,
    Declined,
    Removed,
}

impl MembershipStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            MembershipStatus::Invited => "invited",
            MembershipStatus::Accepted => "accepted",
            MembershipStatus::Declined => "declined",
            MembershipStatus::Removed => "removed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
    Cancelled,
}

impl InvitationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            InvitationStatus::Pending => "pending",
            InvitationStatus::Accepted => "accepted",
            InvitationStatus::Declined => "declined",
            InvitationStatus::Expired => "expired",
            InvitationStatus::Cancelled => "cancelled",
        }
    }
}

impl FromStr for InvitationStatus {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "pending" => Ok(InvitationStatus::Pending),
            "accepted" => Ok(InvitationStatus::Accepted),
            "declined" => Ok(InvitationStatus::Declined),
            "expired" => Ok(InvitationStatus::Expired),
            "cancelled" => Ok(InvitationStatus::Cancelled),
            _ => Err(()),
        }
    }
}

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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct TeamMembership {
    pub id: Uuid,
    pub team_id: Uuid,
    pub tournament_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub status: String,
    pub joined_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct TeamInvitation {
    pub id: Uuid,
    pub team_id: Uuid,
    pub tournament_id: Uuid,
    pub email: String,
    pub invited_user_id: Option<Uuid>,
    pub invited_by: Uuid,
    pub status: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserIdentity {
    pub id: Uuid,
    pub email: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TournamentTeamRow {
    pub id: Uuid,
    pub name: String,
    pub organization: Option<String>,
    pub members_count: i64,
    pub captain_id: Option<Uuid>,
    pub captain_full_name: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TeamMemberRow {
    pub user_id: Uuid,
    pub full_name: String,
    pub email: String,
    pub role: String,
    pub status: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MyInvitationRow {
    pub id: Uuid,
    pub team_id: Uuid,
    pub team_name: String,
    pub tournament_id: Uuid,
    pub tournament_title: String,
    pub invited_by_id: Uuid,
    pub invited_by_full_name: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MyTeamRow {
    pub team_id: Uuid,
    pub team_name: String,
    pub role: String,
    pub status: String,
    pub tournament_id: Uuid,
    pub tournament_title: String,
    pub tournament_status: String,
    pub members_count: i64,
}

pub struct NewTeam<'a> {
    pub tournament_id: Uuid,
    pub created_by: Uuid,
    pub name: &'a str,
    pub organization: Option<&'a str>,
    pub contact: Option<&'a str>,
}

pub struct NewInvitation<'a> {
    pub team_id: Uuid,
    pub tournament_id: Uuid,
    pub email: &'a str,
    pub invited_user_id: Option<Uuid>,
    pub invited_by: Uuid,
    pub token: &'a str,
    pub expires_at: DateTime<Utc>,
}
