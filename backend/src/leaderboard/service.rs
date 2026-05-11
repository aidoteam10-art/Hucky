use std::{cmp::Ordering, collections::HashMap};

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult},
    users::auth::AuthenticatedUser,
};

use super::{
    dto::{LeaderboardItem, LeaderboardResponse, LeaderboardTournamentInfo},
    model::{LeaderboardScoreRow, TeamScore},
    repository::LeaderboardRepository,
};

pub struct LeaderboardService;

impl LeaderboardService {
    pub async fn get(
        db: &PgPool,
        user: Option<AuthenticatedUser>,
        tournament_id: Uuid,
    ) -> ApiResult<LeaderboardResponse> {
        let tournament = LeaderboardRepository::find_tournament(db, tournament_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Tournament not found".to_string()))?;

        let is_organizer = match user {
            Some(user) => {
                LeaderboardRepository::is_organizer(db, tournament_id, user.user_id).await?
            }
            None => false,
        };

        let has_unpublished =
            LeaderboardRepository::has_unpublished_submitted_rounds(db, tournament_id).await?;
        if has_unpublished && !is_organizer {
            return Err(ApiError::Forbidden(
                "Leaderboard is available after evaluation is complete".to_string(),
            ));
        }

        let rows = LeaderboardRepository::score_rows(db, tournament_id).await?;
        let teams = build_team_scores(rows);

        Ok(LeaderboardResponse {
            tournament: LeaderboardTournamentInfo {
                id: tournament.id,
                title: tournament.title,
            },
            items: ranked_items(teams),
        })
    }
}

pub(crate) fn build_team_scores(rows: Vec<LeaderboardScoreRow>) -> Vec<TeamScore> {
    let mut teams = HashMap::<Uuid, TeamScore>::new();

    for row in rows {
        let entry = teams.entry(row.team_id).or_insert_with(|| TeamScore {
            team_id: row.team_id,
            team_name: row.team_name,
            organization: row.organization,
            scores: HashMap::new(),
            total: 0.0,
            functionality: 0.0,
            requirements: 0.0,
            submitted_at: row.submitted_at,
            reviews_count: row.reviews_count,
        });

        let rounded_score = round2(row.weighted_score);
        entry.scores.insert(row.code.clone(), rounded_score);
        entry.total += row.weighted_score;
        entry.submitted_at = entry.submitted_at.min(row.submitted_at);
        entry.reviews_count = entry.reviews_count.max(row.reviews_count);

        if row.code == "functionality" {
            entry.functionality = row.weighted_score;
        }
        if row.code == "requirements" {
            entry.requirements = row.weighted_score;
        }
    }

    teams.into_values().collect()
}

pub(crate) fn ranked_items(mut teams: Vec<TeamScore>) -> Vec<LeaderboardItem> {
    teams.sort_by(compare_team_scores);

    teams
        .into_iter()
        .enumerate()
        .map(|(index, team)| LeaderboardItem {
            rank: (index + 1) as i64,
            team_id: team.team_id,
            team_name: team.team_name,
            organization: team.organization,
            scores: team.scores,
            total: round2(team.total),
            reviews_count: team.reviews_count,
        })
        .collect()
}

pub(crate) fn compare_team_scores(a: &TeamScore, b: &TeamScore) -> Ordering {
    b.total
        .partial_cmp(&a.total)
        .unwrap_or(Ordering::Equal)
        .then_with(|| {
            b.functionality
                .partial_cmp(&a.functionality)
                .unwrap_or(Ordering::Equal)
        })
        .then_with(|| {
            b.requirements
                .partial_cmp(&a.requirements)
                .unwrap_or(Ordering::Equal)
        })
        .then_with(|| a.submitted_at.cmp(&b.submitted_at))
        .then_with(|| a.team_name.cmp(&b.team_name))
}

fn round2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;

    fn team(
        name: &str,
        total: f64,
        functionality: f64,
        requirements: f64,
        submitted_day: u32,
    ) -> TeamScore {
        TeamScore {
            team_id: Uuid::new_v4(),
            team_name: name.to_string(),
            organization: None,
            scores: HashMap::new(),
            total,
            functionality,
            requirements,
            submitted_at: Utc
                .with_ymd_and_hms(2026, 5, submitted_day, 10, 0, 0)
                .unwrap(),
            reviews_count: 3,
        }
    }

    #[test]
    fn sorts_by_total_then_tie_breakers() {
        let items = ranked_items(vec![
            team("A", 500.0, 90.0, 90.0, 1),
            team("B", 500.0, 95.0, 80.0, 1),
            team("C", 500.0, 95.0, 85.0, 1),
            team("D", 510.0, 50.0, 50.0, 1),
        ]);

        assert_eq!(items[0].team_name, "D");
        assert_eq!(items[1].team_name, "C");
        assert_eq!(items[2].team_name, "B");
        assert_eq!(items[3].team_name, "A");
    }

    #[test]
    fn earlier_submission_wins_after_score_ties() {
        let items = ranked_items(vec![
            team("Late", 500.0, 90.0, 80.0, 3),
            team("Early", 500.0, 90.0, 80.0, 1),
        ]);

        assert_eq!(items[0].team_name, "Early");
    }
}
