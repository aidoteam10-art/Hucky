use axum::{extract::{Path, State}, routing::get, Json, Router};
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;
use crate::state::AppState;


#[derive(Serialize)]
pub struct TournamentInfo {
    pub id: Uuid,
    pub title: String,
}

#[derive(Serialize)]
pub struct LeaderboardItem {
    pub rank: i64,
    pub team_id: Uuid,
    pub team_name: String,
    pub organization: Option<String>,
    pub scores: HashMap<String, f64>,
    pub total: f64,
    pub reviews_count: i64,
}

#[derive(Serialize)]
pub struct LeaderboardRes {
    pub tournament: TournamentInfo,
    pub items: Vec<LeaderboardItem>,
}

async fn get_leaderboard(
    State(state): State<AppState>, Path(tournament_id): Path<Uuid>,
) -> Result<Json<LeaderboardRes>, (axum::http::StatusCode, String)> {
    
    let tour_title = sqlx::query_scalar!("SELECT title FROM tournaments WHERE id = $1", tournament_id)
        .fetch_optional(&state.db).await
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?
        .unwrap_or_else(|| "Unknown Tournament".to_string());

    
    let records = sqlx::query!(
        r#"
        WITH TeamReviews AS (
            SELECT s2.team_id, count(ja2.id) as reviews_count
            FROM submissions s2
            JOIN jury_assignments ja2 ON ja2.submission_id = s2.id
            WHERE ja2.status = 'completed'
            GROUP BY s2.team_id
        )
        SELECT t.id as team_id, t.name as team_name, t.organization, ec.code, 
               CAST(AVG(es.score) AS FLOAT8) as avg_score, 
               CAST(ec.weight AS FLOAT8) as weight,
               MAX(s.submitted_at) as submitted_at,
               COALESCE(tr.reviews_count, 0) as reviews_count
        FROM teams t
        JOIN submissions s ON s.team_id = t.id
        JOIN rounds r ON s.round_id = r.id
        JOIN jury_assignments ja ON ja.submission_id = s.id AND ja.status = 'completed'
        JOIN evaluation_scores es ON es.assignment_id = ja.id
        JOIN evaluation_criteria ec ON ec.id = es.criterion_id
        LEFT JOIN TeamReviews tr ON tr.team_id = t.id
        WHERE r.tournament_id = $1
        GROUP BY t.id, t.name, t.organization, ec.code, ec.weight, tr.reviews_count
        "#,
        tournament_id
    )
    .fetch_all(&state.db).await
    .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Помилка бази даних".into()))?;

    struct TeamData {
        name: String,
        organization: Option<String>,
        scores: HashMap<String, f64>,
        total: f64,
        func_score: f64,
        req_score: f64,
        submitted_at: chrono::DateTime<chrono::Utc>,
        reviews_count: i64,
    }

    let mut teams_map: HashMap<Uuid, TeamData> = HashMap::new();

    for r in records {
        let avg = r.avg_score.unwrap_or(0.0);
        let weight = r.weight.unwrap_or(1.0);
        let weighted_score = avg * weight;
        let code = r.code;

        let entry = teams_map.entry(r.team_id).or_insert(TeamData {
            name: r.team_name,
            organization: r.organization,
            scores: HashMap::new(),
            total: 0.0,
            func_score: 0.0,
            req_score: 0.0,
            submitted_at: r.submitted_at.unwrap_or_else(chrono::Utc::now),
            reviews_count: r.reviews_count.unwrap_or(0),
        });

        entry.scores.insert(code.clone(), (weighted_score * 100.0).round() / 100.0);
        entry.total += weighted_score;

        if code == "functionality" { entry.func_score = weighted_score; }
        if code == "requirements" { entry.req_score = weighted_score; }
    }

    let mut sorted_teams: Vec<(Uuid, TeamData)> = teams_map.into_iter().collect();

    sorted_teams.sort_by(|a, b| {
        b.1.total.partial_cmp(&a.1.total).unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.1.func_score.partial_cmp(&a.1.func_score).unwrap_or(std::cmp::Ordering::Equal))
            .then_with(|| b.1.req_score.partial_cmp(&a.1.req_score).unwrap_or(std::cmp::Ordering::Equal))
            .then_with(|| a.1.submitted_at.cmp(&b.1.submitted_at))
    });

    let mut response_items = Vec::new();
    for (index, (team_id, data)) in sorted_teams.into_iter().enumerate() {
        response_items.push(LeaderboardItem {
            rank: (index + 1) as i64,
            team_id,
            team_name: data.name,
            organization: data.organization,
            scores: data.scores,
            total: (data.total * 100.0).round() / 100.0,
            reviews_count: data.reviews_count,
        });
    }

    Ok(Json(LeaderboardRes {
        tournament: TournamentInfo {
            id: tournament_id,
            title: tour_title,
        },
        items: response_items,
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/tournaments/:id/leaderboard", get(get_leaderboard))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaderboard_sorting_and_tie_breakers() {
        let mut teams = vec![
            (Uuid::new_v4(), TeamData {
                name: "Team A".into(), organization: None, scores: HashMap::new(),
                total: 500.0, func_score: 90.0, req_score: 80.0,
                submitted_at: chrono::Utc::now(), reviews_count: 3
            }),
            (Uuid::new_v4(), TeamData {
                name: "Team B (Tie Break Func)".into(), organization: None, scores: HashMap::new(),
                total: 500.0, func_score: 95.0, req_score: 80.0,
                submitted_at: chrono::Utc::now(), reviews_count: 3
            }),
            (Uuid::new_v4(), TeamData {
                name: "Team C (Tie Break Req)".into(), organization: None, scores: HashMap::new(),
                total: 500.0, func_score: 95.0, req_score: 85.0,
                submitted_at: chrono::Utc::now(), reviews_count: 3
            }),
            (Uuid::new_v4(), TeamData {
                name: "Team D (Winner)".into(), organization: None, scores: HashMap::new(),
                total: 510.0, func_score: 50.0, req_score: 50.0,
                submitted_at: chrono::Utc::now(), reviews_count: 3
            }),
        ];

        teams.sort_by(|a, b| {
            b.1.total.partial_cmp(&a.1.total).unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.1.func_score.partial_cmp(&a.1.func_score).unwrap_or(std::cmp::Ordering::Equal))
                .then_with(|| b.1.req_score.partial_cmp(&a.1.req_score).unwrap_or(std::cmp::Ordering::Equal))
                .then_with(|| a.1.submitted_at.cmp(&b.1.submitted_at))
        });

        assert_eq!(teams[0].1.name, "Team D (Winner)");
        assert_eq!(teams[1].1.name, "Team C (Tie Break Req)");
        assert_eq!(teams[2].1.name, "Team B (Tie Break Func)");
        assert_eq!(teams[3].1.name, "Team A");
    }
}