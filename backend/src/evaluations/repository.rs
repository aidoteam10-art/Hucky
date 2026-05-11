use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::model::{AssignmentForEvaluation, EvaluationCriterion};

pub struct EvaluationRepository;

impl EvaluationRepository {
    pub async fn find_assignment(
        db: &PgPool,
        assignment_id: Uuid,
    ) -> Result<Option<AssignmentForEvaluation>, sqlx::Error> {
        sqlx::query_as::<_, AssignmentForEvaluation>(
            "SELECT
                ja.id,
                ja.status,
                ja.jury_user_id,
                ja.round_id,
                r.tournament_id
            FROM jury_assignments ja
            JOIN rounds r ON r.id = ja.round_id
            WHERE ja.id = $1",
        )
        .bind(assignment_id)
        .fetch_optional(db)
        .await
    }

    pub async fn criteria_for_tournament(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<Vec<EvaluationCriterion>, sqlx::Error> {
        sqlx::query_as::<_, EvaluationCriterion>(
            "SELECT id, max_score
            FROM evaluation_criteria
            WHERE tournament_id = $1
            ORDER BY position ASC",
        )
        .bind(tournament_id)
        .fetch_all(db)
        .await
    }

    pub async fn insert_score(
        tx: &mut Transaction<'_, Postgres>,
        assignment_id: Uuid,
        criterion_id: Uuid,
        score: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO evaluation_scores (assignment_id, criterion_id, score)
            VALUES ($1, $2, $3)",
        )
        .bind(assignment_id)
        .bind(criterion_id)
        .bind(score)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn insert_comment(
        tx: &mut Transaction<'_, Postgres>,
        assignment_id: Uuid,
        comment: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO evaluation_comments (assignment_id, comment)
            VALUES ($1, $2)",
        )
        .bind(assignment_id)
        .bind(comment)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn mark_assignment_completed(
        tx: &mut Transaction<'_, Postgres>,
        assignment_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE jury_assignments
            SET status = 'completed', completed_at = NOW()
            WHERE id = $1",
        )
        .bind(assignment_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn pending_assignments_count(
        tx: &mut Transaction<'_, Postgres>,
        round_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)::bigint
            FROM jury_assignments
            WHERE round_id = $1 AND status = 'pending'",
        )
        .bind(round_id)
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn mark_round_evaluated(
        tx: &mut Transaction<'_, Postgres>,
        round_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE rounds SET status = 'evaluated', updated_at = NOW() WHERE id = $1")
            .bind(round_id)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
