use sqlx::PgPool;
use uuid::Uuid;

use super::model::{LeaderboardScoreRow, LeaderboardTournament};

pub struct LeaderboardRepository;

impl LeaderboardRepository {
    pub async fn find_tournament(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<Option<LeaderboardTournament>, sqlx::Error> {
        sqlx::query_as::<_, LeaderboardTournament>(
            "SELECT id, title, status
            FROM tournaments
            WHERE id = $1",
        )
        .bind(tournament_id)
        .fetch_optional(db)
        .await
    }

    pub async fn is_organizer(
        db: &PgPool,
        tournament_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (
                SELECT 1
                FROM tournaments t
                WHERE t.id = $1
                    AND EXISTS (
                        SELECT 1
                        FROM users u
                        WHERE u.id = $2
                            AND u.account_role = 'organiser'
                    )
                    AND (
                        t.organizer_id = $2
                        OR EXISTS (
                            SELECT 1
                            FROM tournament_staff_roles tsr
                            WHERE tsr.tournament_id = t.id
                                AND tsr.user_id = $2
                                AND tsr.role = 'organizer'
                        )
                    )
            )",
        )
        .bind(tournament_id)
        .bind(user_id)
        .fetch_one(db)
        .await
    }

    pub async fn has_unpublished_submitted_rounds(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (
                SELECT 1
                FROM rounds r
                WHERE r.tournament_id = $1
                    AND r.status <> 'evaluated'
                    AND EXISTS (
                        SELECT 1 FROM submissions s WHERE s.round_id = r.id
                    )
            )",
        )
        .bind(tournament_id)
        .fetch_one(db)
        .await
    }

    pub async fn score_rows(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<Vec<LeaderboardScoreRow>, sqlx::Error> {
        sqlx::query_as::<_, LeaderboardScoreRow>(
            "WITH completed_reviews AS (
                SELECT
                    s.id AS submission_id,
                    s.team_id,
                    s.submitted_at,
                    ja.id AS assignment_id
                FROM submissions s
                JOIN rounds r ON r.id = s.round_id
                JOIN jury_assignments ja
                    ON ja.submission_id = s.id
                    AND ja.status = 'completed'
                WHERE r.tournament_id = $1
            ),
            review_counts AS (
                SELECT
                    team_id,
                    COUNT(DISTINCT assignment_id)::bigint AS reviews_count,
                    MIN(submitted_at) AS submitted_at
                FROM completed_reviews
                GROUP BY team_id
            )
            SELECT
                tm.id AS team_id,
                tm.name AS team_name,
                tm.organization,
                ec.code,
                CAST(AVG(es.score)::float8 * ec.weight::float8 AS float8) AS weighted_score,
                rc.submitted_at,
                rc.reviews_count
            FROM completed_reviews cr
            JOIN teams tm ON tm.id = cr.team_id
            JOIN evaluation_scores es ON es.assignment_id = cr.assignment_id
            JOIN evaluation_criteria ec ON ec.id = es.criterion_id
            JOIN review_counts rc ON rc.team_id = cr.team_id
            GROUP BY tm.id, tm.name, tm.organization, ec.code, ec.weight, rc.submitted_at, rc.reviews_count
            ORDER BY tm.name ASC, ec.code ASC",
        )
        .bind(tournament_id)
        .fetch_all(db)
        .await
    }
}
