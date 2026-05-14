use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct CertificatesResponse {
    pub items: Vec<CertificateItem>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateItem {
    pub id: Uuid,
    pub user_name: String,
    pub team_name: String,
    pub tournament_name: String,
    pub overall_place: Option<i64>,
    pub issued_at: String,
    pub rounds: Vec<CertificateRound>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSnapshot {
    pub user_name: String,
    pub team_name: String,
    pub tournament_name: String,
    pub overall_place: Option<i64>,
    pub issued_at: String,
    pub rounds: Vec<CertificateRound>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CertificateRound {
    pub title: String,
    pub place: Option<i64>,
}

impl CertificateItem {
    pub fn from_snapshot(id: Uuid, snapshot: CertificateSnapshot) -> Self {
        Self {
            id,
            user_name: snapshot.user_name,
            team_name: snapshot.team_name,
            tournament_name: snapshot.tournament_name,
            overall_place: snapshot.overall_place,
            issued_at: snapshot.issued_at,
            rounds: snapshot.rounds,
        }
    }
}
