use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::model::{Submission, SubmissionInput, SubmissionRound};

pub struct SubmissionRepository;

impl SubmissionRepository {
    pub async fn find_round(
        db: &PgPool,
        round_id: Uuid,
    ) -> Result<Option<SubmissionRound>, sqlx::Error> {
        sqlx::query_as::<_, SubmissionRound>(
            "SELECT id, tournament_id, status, deadline_at
            FROM rounds
            WHERE id = $1",
        )
        .bind(round_id)
        .fetch_optional(db)
        .await
    }

    pub async fn captain_team_for_round(
        db: &PgPool,
        round_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        sqlx::query_scalar::<_, Uuid>(
            "SELECT tm.team_id
            FROM team_memberships tm
            JOIN rounds r ON r.tournament_id = tm.tournament_id
            JOIN teams t ON t.id = tm.team_id AND t.tournament_id = r.tournament_id
            WHERE r.id = $1
                AND tm.user_id = $2
                AND tm.status = 'accepted'
                AND tm.role = 'captain'
            LIMIT 1",
        )
        .bind(round_id)
        .bind(user_id)
        .fetch_optional(db)
        .await
    }

    pub async fn count_accepted_members(db: &PgPool, team_id: Uuid) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)::bigint
            FROM team_memberships
            WHERE team_id = $1 AND status = 'accepted'",
        )
        .bind(team_id)
        .fetch_one(db)
        .await
    }

    pub async fn find_for_team_round(
        db: &PgPool,
        round_id: Uuid,
        team_id: Uuid,
    ) -> Result<Option<Submission>, sqlx::Error> {
        sqlx::query_as::<_, Submission>(
            "SELECT id, round_id, team_id, github_url, video_demo_url, live_demo_url,
                description, status, submitted_at, updated_at, locked_at
            FROM submissions
            WHERE round_id = $1 AND team_id = $2",
        )
        .bind(round_id)
        .bind(team_id)
        .fetch_optional(db)
        .await
    }

    pub async fn find_for_user_round(
        db: &PgPool,
        round_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Submission>, sqlx::Error> {
        sqlx::query_as::<_, Submission>(
            "SELECT s.id, s.round_id, s.team_id, s.github_url, s.video_demo_url,
                s.live_demo_url, s.description, s.status, s.submitted_at, s.updated_at,
                s.locked_at
            FROM submissions s
            JOIN rounds r ON r.id = s.round_id
            JOIN team_memberships tm
                ON tm.team_id = s.team_id
                AND tm.tournament_id = r.tournament_id
                AND tm.status = 'accepted'
            WHERE s.round_id = $1 AND tm.user_id = $2
            LIMIT 1",
        )
        .bind(round_id)
        .bind(user_id)
        .fetch_optional(db)
        .await
    }

    pub async fn upsert_submission(
        db: &PgPool,
        round_id: Uuid,
        team_id: Uuid,
        input: SubmissionInput<'_>,
    ) -> Result<Submission, sqlx::Error> {
        sqlx::query_as::<_, Submission>(
            "INSERT INTO submissions (
                round_id, team_id, github_url, video_demo_url, live_demo_url, description
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (round_id, team_id)
            DO UPDATE SET
                github_url = EXCLUDED.github_url,
                video_demo_url = EXCLUDED.video_demo_url,
                live_demo_url = EXCLUDED.live_demo_url,
                description = EXCLUDED.description,
                status = 'submitted',
                updated_at = NOW()
            WHERE submissions.locked_at IS NULL AND submissions.status <> 'locked'
            RETURNING id, round_id, team_id, github_url, video_demo_url, live_demo_url,
                description, status, submitted_at, updated_at, locked_at",
        )
        .bind(round_id)
        .bind(team_id)
        .bind(input.github_url)
        .bind(input.video_demo_url)
        .bind(input.live_demo_url)
        .bind(input.description)
        .fetch_one(db)
        .await
    }

    pub async fn is_tournament_organizer(
        db: &PgPool,
        tournament_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (
                SELECT 1
                FROM tournaments t
                WHERE t.id = $1
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

    pub async fn lock_round_submissions(
        tx: &mut Transaction<'_, Postgres>,
        round_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE submissions
            SET status = 'locked',
                locked_at = COALESCE(locked_at, NOW()),
                updated_at = NOW()
            WHERE round_id = $1",
        )
        .bind(round_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn update_round_status(
        tx: &mut Transaction<'_, Postgres>,
        round_id: Uuid,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE rounds SET status = $2, updated_at = NOW() WHERE id = $1")
            .bind(round_id)
            .bind(status)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
