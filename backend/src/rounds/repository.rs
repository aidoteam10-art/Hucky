use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::model::{NewRound, Round, RoundListItem, RoundRequirement, RoundStatus, UpdateRound};

pub struct RoundRepository;

impl RoundRepository {
    pub async fn insert(
        tx: &mut Transaction<'_, Postgres>,
        round: NewRound<'_>,
    ) -> Result<Round, sqlx::Error> {
        sqlx::query_as::<_, Round>(
            "INSERT INTO rounds (
                tournament_id,
                title,
                task_description,
                technology_requirements,
                status,
                starts_at,
                deadline_at,
                position
            )
            VALUES ($1, $2, $3, $4, 'draft', $5, $6, $7)
            RETURNING id, tournament_id, title, task_description, technology_requirements,
                status, starts_at, deadline_at, position",
        )
        .bind(round.tournament_id)
        .bind(round.title)
        .bind(round.task_description)
        .bind(round.technology_requirements)
        .bind(round.starts_at)
        .bind(round.deadline_at)
        .bind(round.position)
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn insert_requirements(
        tx: &mut Transaction<'_, Postgres>,
        round_id: Uuid,
        requirements: &[String],
    ) -> Result<(), sqlx::Error> {
        for (index, requirement) in requirements.iter().enumerate() {
            sqlx::query(
                "INSERT INTO round_requirements (round_id, text, position)
                VALUES ($1, $2, $3)",
            )
            .bind(round_id)
            .bind(requirement)
            .bind((index + 1) as i32)
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    pub async fn replace_requirements(
        tx: &mut Transaction<'_, Postgres>,
        round_id: Uuid,
        requirements: &[String],
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM round_requirements WHERE round_id = $1")
            .bind(round_id)
            .execute(&mut **tx)
            .await?;

        Self::insert_requirements(tx, round_id, requirements).await
    }

    pub async fn find_by_id(db: &PgPool, round_id: Uuid) -> Result<Option<Round>, sqlx::Error> {
        sqlx::query_as::<_, Round>(
            "SELECT id, tournament_id, title, task_description, technology_requirements,
                status, starts_at, deadline_at, position
            FROM rounds
            WHERE id = $1",
        )
        .bind(round_id)
        .fetch_optional(db)
        .await
    }

    pub async fn list_by_tournament(
        db: &PgPool,
        tournament_id: Uuid,
        include_drafts: bool,
    ) -> Result<Vec<RoundListItem>, sqlx::Error> {
        sqlx::query_as::<_, RoundListItem>(
            "SELECT id, tournament_id, title, task_description, technology_requirements,
                status, starts_at, deadline_at, position
            FROM rounds
            WHERE tournament_id = $1 AND ($2 OR status <> 'draft')
            ORDER BY position ASC",
        )
        .bind(tournament_id)
        .bind(include_drafts)
        .fetch_all(db)
        .await
    }

    pub async fn requirements(
        db: &PgPool,
        round_id: Uuid,
    ) -> Result<Vec<RoundRequirement>, sqlx::Error> {
        sqlx::query_as::<_, RoundRequirement>(
            "SELECT id, text, position
            FROM round_requirements
            WHERE round_id = $1
            ORDER BY position ASC",
        )
        .bind(round_id)
        .fetch_all(db)
        .await
    }

    pub async fn update(
        tx: &mut Transaction<'_, Postgres>,
        round_id: Uuid,
        round: UpdateRound<'_>,
    ) -> Result<Round, sqlx::Error> {
        sqlx::query_as::<_, Round>(
            "UPDATE rounds
            SET title = $2,
                task_description = $3,
                technology_requirements = $4,
                starts_at = $5,
                deadline_at = $6,
                position = $7,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, tournament_id, title, task_description, technology_requirements,
                status, starts_at, deadline_at, position",
        )
        .bind(round_id)
        .bind(round.title)
        .bind(round.task_description)
        .bind(round.technology_requirements)
        .bind(round.starts_at)
        .bind(round.deadline_at)
        .bind(round.position)
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn update_status(
        db: &PgPool,
        round_id: Uuid,
        status: RoundStatus,
    ) -> Result<Round, sqlx::Error> {
        sqlx::query_as::<_, Round>(
            "UPDATE rounds
            SET status = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, tournament_id, title, task_description, technology_requirements,
                status, starts_at, deadline_at, position",
        )
        .bind(round_id)
        .bind(status.as_str())
        .fetch_one(db)
        .await
    }

    pub async fn pending_assignments_count(
        db: &PgPool,
        round_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)::bigint
            FROM jury_assignments
            WHERE round_id = $1 AND status = 'pending'",
        )
        .bind(round_id)
        .fetch_one(db)
        .await
    }

    pub async fn next_position(db: &PgPool, tournament_id: Uuid) -> Result<i32, sqlx::Error> {
        let position = sqlx::query_scalar::<_, i32>(
            "SELECT COALESCE(MAX(position), 0) + 1 FROM rounds WHERE tournament_id = $1",
        )
        .bind(tournament_id)
        .fetch_one(db)
        .await?;

        Ok(position)
    }
}
