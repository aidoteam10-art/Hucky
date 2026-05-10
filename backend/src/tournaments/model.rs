use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TournamentStatus {
    Draft,
    Registration,
    Running,
    Finished,
}

impl TournamentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TournamentStatus::Draft => "draft",
            TournamentStatus::Registration => "registration",
            TournamentStatus::Running => "running",
            TournamentStatus::Finished => "finished",
        }
    }
}

impl FromStr for TournamentStatus {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "draft" => Ok(TournamentStatus::Draft),
            "registration" => Ok(TournamentStatus::Registration),
            "running" => Ok(TournamentStatus::Running),
            "finished" => Ok(TournamentStatus::Finished),
            _ => Err(()),
        }
    }
}

impl fmt::Display for TournamentStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Tournament {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub rules: String,
    pub status: String,
    pub registration_starts_at: DateTime<Utc>,
    pub registration_ends_at: DateTime<Utc>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
    pub max_teams: Option<i32>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct TournamentListItem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub starts_at: DateTime<Utc>,
    pub rounds_count: i64,
    pub max_teams: Option<i32>,
    pub registered_teams: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ActiveRoundPreview {
    pub id: Uuid,
    pub title: String,
    pub deadline_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TournamentListFilter {
    pub status: Option<TournamentStatus>,
    pub search: Option<String>,
    pub page: i64,
    pub per_page: i64,
}

pub struct NewTournament<'a> {
    pub organizer_id: Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub rules: &'a str,
    pub registration_starts_at: DateTime<Utc>,
    pub registration_ends_at: DateTime<Utc>,
    pub starts_at: DateTime<Utc>,
    pub max_teams: Option<i32>,
}

pub struct UpdateTournament<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub rules: &'a str,
    pub registration_starts_at: DateTime<Utc>,
    pub registration_ends_at: DateTime<Utc>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
    pub max_teams: Option<i32>,
}
