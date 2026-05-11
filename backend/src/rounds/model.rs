use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundStatus {
    Draft,
    Active,
    SubmissionClosed,
    Evaluated,
}

impl RoundStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            RoundStatus::Draft => "draft",
            RoundStatus::Active => "active",
            RoundStatus::SubmissionClosed => "submission_closed",
            RoundStatus::Evaluated => "evaluated",
        }
    }
}

impl FromStr for RoundStatus {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "draft" => Ok(RoundStatus::Draft),
            "active" => Ok(RoundStatus::Active),
            "submission_closed" => Ok(RoundStatus::SubmissionClosed),
            "evaluated" => Ok(RoundStatus::Evaluated),
            _ => Err(()),
        }
    }
}

impl fmt::Display for RoundStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Round {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub title: String,
    pub task_description: String,
    pub technology_requirements: Option<String>,
    pub status: String,
    pub starts_at: DateTime<Utc>,
    pub deadline_at: DateTime<Utc>,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct RoundRequirement {
    pub id: Uuid,
    pub text: String,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct RoundListItem {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub title: String,
    pub task_description: String,
    pub technology_requirements: Option<String>,
    pub status: String,
    pub starts_at: DateTime<Utc>,
    pub deadline_at: DateTime<Utc>,
    pub position: i32,
}

pub struct NewRound<'a> {
    pub tournament_id: Uuid,
    pub title: &'a str,
    pub task_description: &'a str,
    pub technology_requirements: Option<&'a str>,
    pub starts_at: DateTime<Utc>,
    pub deadline_at: DateTime<Utc>,
    pub position: i32,
}

pub struct UpdateRound<'a> {
    pub title: &'a str,
    pub task_description: &'a str,
    pub technology_requirements: Option<&'a str>,
    pub starts_at: DateTime<Utc>,
    pub deadline_at: DateTime<Utc>,
    pub position: i32,
}
