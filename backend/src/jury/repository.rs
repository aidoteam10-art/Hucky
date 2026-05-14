use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::{
    dto::{AssignmentCriterion, AssignmentListItem, JuryListItem, OrganizerJuryTournament},
    model::{AssignmentDetailRow, AssignmentRound, AssignmentSubmissionRow},
};

pub struct JuryRepository;

impl JuryRepository {
    pub async fn find_user_by_email(db: &PgPool, email: &str) -> Result<Option<Uuid>, sqlx::Error> {
        sqlx::query_scalar::<_, Uuid>(
            "SELECT id
            FROM users
            WHERE lower(email) = lower($1)",
        )
        .bind(email)
        .fetch_optional(db)
        .await
    }

    pub async fn list_organizer_manageable_tournaments(
        db: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<OrganizerJuryTournament>, sqlx::Error> {
        sqlx::query_as::<_, OrganizerJuryTournament>(
            "SELECT
                t.id,
                t.title,
                t.status,
                COALESCE(
                    array_agg(tsr.user_id ORDER BY u.full_name ASC)
                        FILTER (WHERE tsr.role = 'jury'),
                    ARRAY[]::uuid[]
                ) AS jury_ids
            FROM tournaments t
            LEFT JOIN tournament_staff_roles tsr
                ON tsr.tournament_id = t.id
                AND tsr.role = 'jury'
            LEFT JOIN users u ON u.id = tsr.user_id
            WHERE t.status <> 'finished'
                AND (
                    t.organizer_id = $1
                    OR EXISTS (
                        SELECT 1
                        FROM tournament_staff_roles organizer_role
                        WHERE organizer_role.tournament_id = t.id
                            AND organizer_role.user_id = $1
                            AND organizer_role.role = 'organizer'
                    )
                )
            GROUP BY t.id, t.title, t.status, t.created_at
            ORDER BY t.created_at DESC",
        )
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn list_jury_candidates(db: &PgPool) -> Result<Vec<JuryListItem>, sqlx::Error> {
        sqlx::query_as::<_, JuryListItem>(
            "SELECT id, email, full_name
            FROM users
            WHERE account_role = 'jury'
            ORDER BY full_name ASC, email ASC",
        )
        .fetch_all(db)
        .await
    }

    pub async fn add_jury_role(
        db: &PgPool,
        tournament_id: Uuid,
        user_id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO tournament_staff_roles (tournament_id, user_id, role)
            VALUES ($1, $2, 'jury')
            ON CONFLICT (tournament_id, user_id, role) DO NOTHING",
        )
        .bind(tournament_id)
        .bind(user_id)
        .execute(db)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn list_jury(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<Vec<JuryListItem>, sqlx::Error> {
        sqlx::query_as::<_, JuryListItem>(
            "SELECT u.id, u.email, u.full_name
            FROM users u
            JOIN tournament_staff_roles tsr ON tsr.user_id = u.id
            WHERE tsr.tournament_id = $1 AND tsr.role = 'jury'
            ORDER BY u.full_name ASC",
        )
        .bind(tournament_id)
        .fetch_all(db)
        .await
    }

    pub async fn remove_jury_role(
        db: &PgPool,
        tournament_id: Uuid,
        user_id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM tournament_staff_roles
            WHERE tournament_id = $1 AND user_id = $2 AND role = 'jury'",
        )
        .bind(tournament_id)
        .bind(user_id)
        .execute(db)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn find_round(
        db: &PgPool,
        round_id: Uuid,
    ) -> Result<Option<AssignmentRound>, sqlx::Error> {
        sqlx::query_as::<_, AssignmentRound>(
            "SELECT id, tournament_id, status
            FROM rounds
            WHERE id = $1",
        )
        .bind(round_id)
        .fetch_optional(db)
        .await
    }

    pub async fn count_assignments_for_round(
        db: &PgPool,
        round_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)::bigint
            FROM jury_assignments
            WHERE round_id = $1",
        )
        .bind(round_id)
        .fetch_one(db)
        .await
    }

    pub async fn submissions_for_round(
        db: &PgPool,
        round_id: Uuid,
    ) -> Result<Vec<AssignmentSubmissionRow>, sqlx::Error> {
        sqlx::query_as::<_, AssignmentSubmissionRow>(
            "SELECT id, team_id
            FROM submissions
            WHERE round_id = $1
            ORDER BY submitted_at ASC, id ASC",
        )
        .bind(round_id)
        .fetch_all(db)
        .await
    }

    pub async fn jury_user_ids(db: &PgPool, tournament_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
        sqlx::query_scalar::<_, Uuid>(
            "SELECT user_id
            FROM tournament_staff_roles
            WHERE tournament_id = $1 AND role = 'jury'
            ORDER BY user_id ASC",
        )
        .bind(tournament_id)
        .fetch_all(db)
        .await
    }

    pub async fn accepted_team_member_ids(
        db: &PgPool,
        team_id: Uuid,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        sqlx::query_scalar::<_, Uuid>(
            "SELECT user_id
            FROM team_memberships
            WHERE team_id = $1 AND status = 'accepted'",
        )
        .bind(team_id)
        .fetch_all(db)
        .await
    }

    pub async fn insert_assignment(
        tx: &mut Transaction<'_, Postgres>,
        round_id: Uuid,
        submission_id: Uuid,
        jury_user_id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO jury_assignments (round_id, submission_id, jury_user_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (submission_id, jury_user_id) DO NOTHING",
        )
        .bind(round_id)
        .bind(submission_id)
        .bind(jury_user_id)
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn assignments_for_jury(
        db: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<AssignmentListItem>, sqlx::Error> {
        sqlx::query_as::<_, AssignmentListItem>(
            "SELECT
                ja.id,
                ja.status,
                tm.name AS team_name,
                t.title AS tournament_title,
                r.title AS round_title,
                s.submitted_at
            FROM jury_assignments ja
            JOIN submissions s ON s.id = ja.submission_id
            JOIN teams tm ON tm.id = s.team_id
            JOIN rounds r ON r.id = ja.round_id
            JOIN tournaments t ON t.id = r.tournament_id
            WHERE ja.jury_user_id = $1
            ORDER BY ja.assigned_at DESC",
        )
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn assignment_detail_for_jury(
        db: &PgPool,
        assignment_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<AssignmentDetailRow>, sqlx::Error> {
        sqlx::query_as::<_, AssignmentDetailRow>(
            "SELECT
                ja.id,
                ja.status,
                tm.id AS team_id,
                tm.name AS team_name,
                s.github_url,
                s.video_demo_url,
                s.live_demo_url,
                s.description,
                r.tournament_id
            FROM jury_assignments ja
            JOIN submissions s ON s.id = ja.submission_id
            JOIN teams tm ON tm.id = s.team_id
            JOIN rounds r ON r.id = ja.round_id
            WHERE ja.id = $1 AND ja.jury_user_id = $2",
        )
        .bind(assignment_id)
        .bind(user_id)
        .fetch_optional(db)
        .await
    }

    pub async fn criteria_for_assignment(
        db: &PgPool,
        assignment_id: Uuid,
        tournament_id: Uuid,
    ) -> Result<Vec<AssignmentCriterion>, sqlx::Error> {
        sqlx::query_as::<_, AssignmentCriterion>(
            "SELECT
                c.id,
                c.code,
                c.name,
                c.description,
                c.max_score,
                es.score
            FROM evaluation_criteria c
            LEFT JOIN evaluation_scores es
                ON es.criterion_id = c.id
                AND es.assignment_id = $1
            WHERE c.tournament_id = $2
            ORDER BY c.position ASC",
        )
        .bind(assignment_id)
        .bind(tournament_id)
        .fetch_all(db)
        .await
    }

    pub async fn assignment_comment(
        db: &PgPool,
        assignment_id: Uuid,
    ) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar::<_, String>(
            "SELECT comment
            FROM evaluation_comments
            WHERE assignment_id = $1",
        )
        .bind(assignment_id)
        .fetch_optional(db)
        .await
    }
}
