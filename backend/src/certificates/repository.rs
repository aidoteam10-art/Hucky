use sqlx::{PgPool, types::Json};
use uuid::Uuid;

use super::{
    dto::CertificateSnapshot,
    model::{EligibleCertificateRow, GeneratedCertificateRow, RoundInfoRow, RoundScoreRow},
};

pub struct CertificateRepository;

impl CertificateRepository {
    pub async fn eligible_for_tournament(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<Vec<EligibleCertificateRow>, sqlx::Error> {
        sqlx::query_as::<_, EligibleCertificateRow>(
            "SELECT
                t.id AS tournament_id,
                t.title AS tournament_title,
                tm.id AS team_id,
                tm.name AS team_name,
                u.id AS user_id,
                u.full_name AS user_name
            FROM tournaments t
            JOIN teams tm ON tm.tournament_id = t.id
            JOIN team_memberships ms
                ON ms.team_id = tm.id
                AND ms.tournament_id = t.id
                AND ms.status = 'accepted'
            JOIN users u ON u.id = ms.user_id
            WHERE t.id = $1 AND t.status = 'finished'
            ORDER BY tm.name ASC, u.full_name ASC",
        )
        .bind(tournament_id)
        .fetch_all(db)
        .await
    }

    pub async fn eligible_for_user(
        db: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<EligibleCertificateRow>, sqlx::Error> {
        sqlx::query_as::<_, EligibleCertificateRow>(
            "SELECT
                t.id AS tournament_id,
                t.title AS tournament_title,
                tm.id AS team_id,
                tm.name AS team_name,
                u.id AS user_id,
                u.full_name AS user_name
            FROM tournaments t
            JOIN teams tm ON tm.tournament_id = t.id
            JOIN team_memberships ms
                ON ms.team_id = tm.id
                AND ms.tournament_id = t.id
                AND ms.status = 'accepted'
            JOIN users u ON u.id = ms.user_id
            WHERE u.id = $1 AND t.status = 'finished'
            ORDER BY t.starts_at DESC, tm.name ASC",
        )
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn list_for_user(
        db: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<GeneratedCertificateRow>, sqlx::Error> {
        sqlx::query_as::<_, GeneratedCertificateRow>(
            "SELECT id, snapshot
            FROM generated_certificates
            WHERE user_id = $1
            ORDER BY issued_at DESC, created_at DESC",
        )
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn upsert(
        db: &PgPool,
        eligible: &EligibleCertificateRow,
        snapshot: CertificateSnapshot,
    ) -> Result<GeneratedCertificateRow, sqlx::Error> {
        let inserted = sqlx::query_as::<_, GeneratedCertificateRow>(
            "INSERT INTO generated_certificates
                (tournament_id, team_id, user_id, snapshot)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (tournament_id, team_id, user_id) DO NOTHING
            RETURNING id, snapshot",
        )
        .bind(eligible.tournament_id)
        .bind(eligible.team_id)
        .bind(eligible.user_id)
        .bind(Json(snapshot))
        .fetch_optional(db)
        .await?;

        if let Some(row) = inserted {
            return Ok(row);
        }

        sqlx::query_as::<_, GeneratedCertificateRow>(
            "SELECT id, snapshot
            FROM generated_certificates
            WHERE tournament_id = $1 AND team_id = $2 AND user_id = $3",
        )
        .bind(eligible.tournament_id)
        .bind(eligible.team_id)
        .bind(eligible.user_id)
        .fetch_one(db)
        .await
    }

    pub async fn round_infos(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<Vec<RoundInfoRow>, sqlx::Error> {
        sqlx::query_as::<_, RoundInfoRow>(
            "SELECT id AS round_id, title
            FROM rounds
            WHERE tournament_id = $1
            ORDER BY position ASC, starts_at ASC",
        )
        .bind(tournament_id)
        .fetch_all(db)
        .await
    }

    pub async fn round_scores(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<Vec<RoundScoreRow>, sqlx::Error> {
        sqlx::query_as::<_, RoundScoreRow>(
            "WITH criterion_scores AS (
                SELECT
                    r.id AS round_id,
                    s.team_id,
                    tm.name AS team_name,
                    s.submitted_at,
                    ec.code,
                    CAST(AVG(es.score)::float8 * ec.weight::float8 AS float8) AS weighted_score
                FROM rounds r
                JOIN submissions s ON s.round_id = r.id
                JOIN teams tm ON tm.id = s.team_id
                JOIN jury_assignments ja
                    ON ja.submission_id = s.id
                    AND ja.status = 'completed'
                JOIN evaluation_scores es ON es.assignment_id = ja.id
                JOIN evaluation_criteria ec ON ec.id = es.criterion_id
                WHERE r.tournament_id = $1
                GROUP BY r.id, s.team_id, tm.name, s.submitted_at, ec.code, ec.weight
            )
            SELECT
                round_id,
                team_id,
                team_name,
                CAST(SUM(weighted_score) AS float8) AS total,
                CAST(COALESCE(SUM(weighted_score) FILTER (WHERE code = 'functionality'), 0) AS float8) AS functionality,
                CAST(COALESCE(SUM(weighted_score) FILTER (WHERE code = 'requirements'), 0) AS float8) AS requirements,
                MIN(submitted_at) AS submitted_at
            FROM criterion_scores
            GROUP BY round_id, team_id, team_name
            ORDER BY round_id ASC, team_name ASC",
        )
        .bind(tournament_id)
        .fetch_all(db)
        .await
    }
}
