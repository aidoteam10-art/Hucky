use std::{cmp::Ordering, collections::HashMap};

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::ApiResult,
    leaderboard::{repository::LeaderboardRepository, service::ranked_items},
    users::auth::AuthenticatedUser,
};

use super::{
    dto::{CertificateItem, CertificateRound, CertificateSnapshot, CertificatesResponse},
    model::{EligibleCertificateRow, RoundInfoRow, RoundScoreRow},
    repository::CertificateRepository,
};

pub struct CertificateService;

struct CertificateGenerationContext {
    overall_places: HashMap<Uuid, i64>,
    round_places: HashMap<(Uuid, Uuid), i64>,
    rounds: Vec<RoundInfoRow>,
}

impl CertificateService {
    pub async fn list_for_user(
        db: &PgPool,
        user: AuthenticatedUser,
    ) -> ApiResult<CertificatesResponse> {
        Self::repair_missing_for_user(db, user.user_id).await?;

        let items = CertificateRepository::list_for_user(db, user.user_id)
            .await?
            .into_iter()
            .map(|row| CertificateItem::from_snapshot(row.id, row.snapshot.0))
            .collect();

        Ok(CertificatesResponse { items })
    }

    pub async fn generate_for_tournament(db: &PgPool, tournament_id: Uuid) -> ApiResult<()> {
        let eligible = CertificateRepository::eligible_for_tournament(db, tournament_id).await?;
        if eligible.is_empty() {
            return Ok(());
        }

        let context = build_generation_context(db, tournament_id).await?;
        for item in eligible {
            let snapshot = build_certificate_snapshot(&item, &context);
            CertificateRepository::upsert(db, &item, snapshot).await?;
        }

        Ok(())
    }

    async fn repair_missing_for_user(db: &PgPool, user_id: Uuid) -> ApiResult<()> {
        let eligible = CertificateRepository::eligible_for_user(db, user_id).await?;
        let mut contexts = HashMap::<Uuid, CertificateGenerationContext>::new();

        for item in eligible {
            if !contexts.contains_key(&item.tournament_id) {
                let context = build_generation_context(db, item.tournament_id).await?;
                contexts.insert(item.tournament_id, context);
            }

            if let Some(context) = contexts.get(&item.tournament_id) {
                let snapshot = build_certificate_snapshot(&item, context);
                CertificateRepository::upsert(db, &item, snapshot).await?;
            }
        }

        Ok(())
    }
}

async fn build_generation_context(
    db: &PgPool,
    tournament_id: Uuid,
) -> ApiResult<CertificateGenerationContext> {
    let leaderboard_rows = LeaderboardRepository::score_rows(db, tournament_id).await?;
    let overall_places = ranked_items(crate::leaderboard::service::build_team_scores(
        leaderboard_rows,
    ))
    .into_iter()
    .map(|item| (item.team_id, item.rank))
    .collect();

    let rounds = CertificateRepository::round_infos(db, tournament_id).await?;
    let round_scores = CertificateRepository::round_scores(db, tournament_id).await?;
    let round_places = rank_round_scores(round_scores);

    Ok(CertificateGenerationContext {
        overall_places,
        round_places,
        rounds,
    })
}

fn build_certificate_snapshot(
    eligible: &EligibleCertificateRow,
    context: &CertificateGenerationContext,
) -> CertificateSnapshot {
    CertificateSnapshot {
        user_name: eligible.user_name.clone(),
        team_name: eligible.team_name.clone(),
        tournament_name: eligible.tournament_title.clone(),
        overall_place: context.overall_places.get(&eligible.team_id).copied(),
        issued_at: Utc::now().format("%d.%m.%Y").to_string(),
        rounds: context
            .rounds
            .iter()
            .map(|round| CertificateRound {
                title: round.title.clone(),
                place: context
                    .round_places
                    .get(&(round.round_id, eligible.team_id))
                    .copied(),
            })
            .collect(),
    }
}

fn rank_round_scores(rows: Vec<RoundScoreRow>) -> HashMap<(Uuid, Uuid), i64> {
    let mut by_round = HashMap::<Uuid, Vec<RoundScoreRow>>::new();
    for row in rows {
        by_round.entry(row.round_id).or_default().push(row);
    }

    let mut places = HashMap::new();
    for (round_id, mut scores) in by_round {
        scores.sort_by(compare_round_scores);
        for (index, score) in scores.into_iter().enumerate() {
            places.insert((round_id, score.team_id), (index + 1) as i64);
        }
    }

    places
}

fn compare_round_scores(a: &RoundScoreRow, b: &RoundScoreRow) -> Ordering {
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

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;

    #[test]
    fn snapshot_contains_certificate_identity_and_places() {
        let tournament_id = Uuid::new_v4();
        let team_id = Uuid::new_v4();
        let first_round_id = Uuid::new_v4();
        let second_round_id = Uuid::new_v4();
        let eligible = eligible_row(tournament_id, team_id);
        let context = CertificateGenerationContext {
            overall_places: HashMap::from([(team_id, 2)]),
            round_places: HashMap::from([((first_round_id, team_id), 1)]),
            rounds: vec![
                RoundInfoRow {
                    round_id: first_round_id,
                    title: "Раунд 1".to_string(),
                },
                RoundInfoRow {
                    round_id: second_round_id,
                    title: "Раунд 2".to_string(),
                },
            ],
        };

        let snapshot = build_certificate_snapshot(&eligible, &context);

        assert_eq!(snapshot.user_name, "Ada Lovelace");
        assert_eq!(snapshot.team_name, "Code Warriors");
        assert_eq!(snapshot.tournament_name, "Hackathon Ukraine 2026");
        assert_eq!(snapshot.overall_place, Some(2));
        assert_eq!(snapshot.rounds[0].place, Some(1));
        assert_eq!(snapshot.rounds[1].place, None);
    }

    #[test]
    fn round_rank_uses_leaderboard_tie_breakers() {
        let round_id = Uuid::new_v4();
        let early = score_row(round_id, "Early", 500.0, 90.0, 80.0, 1);
        let late = score_row(round_id, "Late", 500.0, 90.0, 80.0, 3);
        let stronger_requirements = score_row(round_id, "Strong", 500.0, 90.0, 85.0, 2);
        let winner = score_row(round_id, "Winner", 510.0, 50.0, 50.0, 4);
        let winner_id = winner.team_id;
        let strong_id = stronger_requirements.team_id;
        let early_id = early.team_id;
        let late_id = late.team_id;

        let ranks = rank_round_scores(vec![early, late, stronger_requirements, winner]);

        assert_eq!(ranks.get(&(round_id, winner_id)), Some(&1));
        assert_eq!(ranks.get(&(round_id, strong_id)), Some(&2));
        assert_eq!(ranks.get(&(round_id, early_id)), Some(&3));
        assert_eq!(ranks.get(&(round_id, late_id)), Some(&4));
    }

    fn eligible_row(tournament_id: Uuid, team_id: Uuid) -> EligibleCertificateRow {
        EligibleCertificateRow {
            tournament_id,
            tournament_title: "Hackathon Ukraine 2026".to_string(),
            team_id,
            team_name: "Code Warriors".to_string(),
            user_id: Uuid::new_v4(),
            user_name: "Ada Lovelace".to_string(),
        }
    }

    fn score_row(
        round_id: Uuid,
        team_name: &str,
        total: f64,
        functionality: f64,
        requirements: f64,
        submitted_day: u32,
    ) -> RoundScoreRow {
        RoundScoreRow {
            round_id,
            team_id: Uuid::new_v4(),
            team_name: team_name.to_string(),
            total,
            functionality,
            requirements,
            submitted_at: Utc
                .with_ymd_and_hms(2026, 5, submitted_day, 10, 0, 0)
                .unwrap(),
        }
    }
}
