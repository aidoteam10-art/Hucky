use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct LeaderboardResponse {
    pub tournament: LeaderboardTournamentInfo,
    pub items: Vec<LeaderboardItem>,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardTournamentInfo {
    pub id: Uuid,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardItem {
    pub rank: i64,
    pub team_id: Uuid,
    pub team_name: String,
    pub organization: Option<String>,
    pub scores: HashMap<String, f64>,
    pub total: f64,
    pub reviews_count: i64,
}
