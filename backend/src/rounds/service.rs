use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult},
    tournaments::{
        dto::CreateFirstRoundRequest, model::TournamentStatus, repository::TournamentRepository,
        service::TournamentService,
    },
    users::auth::AuthenticatedUser,
};

use super::{
    dto::{
        ChangeRoundStatusRequest, CreateRoundRequest, RoundDetailResponse, RoundListResponse,
        UpdateRoundRequest,
    },
    model::{NewRound, Round, RoundStatus, UpdateRound},
    repository::RoundRepository,
};

pub struct RoundService;

pub struct NormalizedRoundInput {
    pub title: String,
    pub task_description: String,
    pub technology_requirements: Option<String>,
    pub must_have: Vec<String>,
    pub starts_at: DateTime<Utc>,
    pub deadline_at: DateTime<Utc>,
    pub position: i32,
}

impl RoundService {
    pub fn normalize_first_round(
        input: &CreateFirstRoundRequest,
        position: Option<i32>,
    ) -> ApiResult<NormalizedRoundInput> {
        normalize_round_input(
            &input.title,
            &input.task_description,
            input.technology_requirements.as_deref(),
            &input.must_have,
            input.starts_at,
            input.deadline_at,
            position.unwrap_or(1),
        )
    }

    pub async fn create_round(
        db: &PgPool,
        user: AuthenticatedUser,
        tournament_id: Uuid,
        payload: CreateRoundRequest,
    ) -> ApiResult<RoundDetailResponse> {
        TournamentService::require_tournament_organizer(db, tournament_id, user).await?;

        let position = match payload.position {
            Some(position) => position,
            None => RoundRepository::next_position(db, tournament_id).await?,
        };

        let normalized = normalize_round_input(
            &payload.title,
            &payload.task_description,
            payload.technology_requirements.as_deref(),
            &payload.must_have,
            payload.starts_at,
            payload.deadline_at,
            position,
        )?;

        let mut tx = db.begin().await?;
        let round = RoundRepository::insert(
            &mut tx,
            NewRound {
                tournament_id,
                title: &normalized.title,
                task_description: &normalized.task_description,
                technology_requirements: normalized.technology_requirements.as_deref(),
                starts_at: normalized.starts_at,
                deadline_at: normalized.deadline_at,
                position: normalized.position,
            },
        )
        .await?;
        RoundRepository::insert_requirements(&mut tx, round.id, &normalized.must_have).await?;
        tx.commit().await?;

        build_round_detail(db, round).await
    }

    pub async fn list_rounds(
        db: &PgPool,
        user: Option<AuthenticatedUser>,
        tournament_id: Uuid,
    ) -> ApiResult<RoundListResponse> {
        let include_drafts = match user {
            Some(user) => {
                TournamentRepository::is_organizer(db, tournament_id, user.user_id).await?
            }
            None => false,
        };

        let items = RoundRepository::list_by_tournament(db, tournament_id, include_drafts).await?;

        Ok(RoundListResponse { items })
    }

    pub async fn get_round(
        db: &PgPool,
        user: Option<AuthenticatedUser>,
        round_id: Uuid,
    ) -> ApiResult<RoundDetailResponse> {
        let round = find_round(db, round_id).await?;
        ensure_round_visible(db, user, &round).await?;
        build_round_detail(db, round).await
    }

    pub async fn update_round(
        db: &PgPool,
        user: AuthenticatedUser,
        round_id: Uuid,
        payload: UpdateRoundRequest,
    ) -> ApiResult<RoundDetailResponse> {
        let existing = find_round(db, round_id).await?;
        TournamentService::require_tournament_organizer(db, existing.tournament_id, user).await?;

        let title = payload.title.unwrap_or(existing.title);
        let task_description = payload
            .task_description
            .unwrap_or(existing.task_description);
        let technology_requirements = payload
            .technology_requirements
            .or(existing.technology_requirements);
        let starts_at = payload.starts_at.unwrap_or(existing.starts_at);
        let deadline_at = payload.deadline_at.unwrap_or(existing.deadline_at);
        let position = payload.position.unwrap_or(existing.position);
        let requirements = payload
            .must_have
            .map(|items| normalize_requirements(&items));

        validate_round_fields(&title, &task_description, starts_at, deadline_at, position)?;

        let mut tx = db.begin().await?;
        let updated = RoundRepository::update(
            &mut tx,
            round_id,
            UpdateRound {
                title: title.trim(),
                task_description: task_description.trim(),
                technology_requirements: technology_requirements.as_deref(),
                starts_at,
                deadline_at,
                position,
            },
        )
        .await?;

        if let Some(requirements) = requirements {
            RoundRepository::replace_requirements(&mut tx, round_id, &requirements).await?;
        }

        tx.commit().await?;
        build_round_detail(db, updated).await
    }

    pub async fn change_round_status(
        db: &PgPool,
        user: AuthenticatedUser,
        round_id: Uuid,
        payload: ChangeRoundStatusRequest,
    ) -> ApiResult<RoundDetailResponse> {
        let round = find_round(db, round_id).await?;
        TournamentService::require_tournament_organizer(db, round.tournament_id, user).await?;

        let current = parse_round_status(&round.status)?;
        validate_round_status_transition(current, payload.status)?;

        if current == RoundStatus::Draft && payload.status == RoundStatus::Active {
            validate_round_can_be_activated(db, &round).await?;
        }

        if current == RoundStatus::SubmissionClosed && payload.status == RoundStatus::Evaluated {
            return Self::mark_round_evaluated_after_reviews(db, round_id).await;
        }

        let updated = RoundRepository::update_status(db, round_id, payload.status).await?;
        build_round_detail(db, updated).await
    }

    pub async fn mark_round_evaluated_after_reviews(
        db: &PgPool,
        round_id: Uuid,
    ) -> ApiResult<RoundDetailResponse> {
        let round = find_round(db, round_id).await?;
        let current = parse_round_status(&round.status)?;

        if current != RoundStatus::SubmissionClosed {
            return Err(ApiError::Validation(
                "Only closed submissions can be marked as evaluated".to_string(),
            ));
        }

        let updated = RoundRepository::update_status(db, round_id, RoundStatus::Evaluated).await?;
        build_round_detail(db, updated).await
    }
}

async fn find_round(db: &PgPool, round_id: Uuid) -> ApiResult<Round> {
    RoundRepository::find_by_id(db, round_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Round not found".to_string()))
}

async fn build_round_detail(db: &PgPool, round: Round) -> ApiResult<RoundDetailResponse> {
    let must_have = RoundRepository::requirements(db, round.id).await?;
    let time_left_seconds = (round.deadline_at - Utc::now()).num_seconds().max(0);

    Ok(RoundDetailResponse {
        id: round.id,
        tournament_id: round.tournament_id,
        title: round.title,
        task_description: round.task_description,
        technology_requirements: round.technology_requirements,
        status: parse_round_status(&round.status)?,
        starts_at: round.starts_at,
        deadline_at: round.deadline_at,
        time_left_seconds,
        position: round.position,
        must_have,
    })
}

async fn ensure_round_visible(
    db: &PgPool,
    user: Option<AuthenticatedUser>,
    round: &Round,
) -> ApiResult<()> {
    let status = parse_round_status(&round.status)?;
    if status != RoundStatus::Draft {
        return Ok(());
    }

    if let Some(user) = user {
        let is_organizer =
            TournamentRepository::is_organizer(db, round.tournament_id, user.user_id).await?;
        if is_organizer {
            return Ok(());
        }
    }

    Err(ApiError::NotFound("Round not found".to_string()))
}

async fn validate_round_can_be_activated(db: &PgPool, round: &Round) -> ApiResult<()> {
    let tournament = TournamentRepository::find_by_id(db, round.tournament_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Tournament not found".to_string()))?;
    let tournament_status = TournamentStatus::from_str(&tournament.status)
        .map_err(|_| ApiError::Validation("Tournament has invalid status".to_string()))?;

    if tournament_status != TournamentStatus::Running {
        return Err(ApiError::Validation(
            "Round can be activated only when tournament is running".to_string(),
        ));
    }

    validate_round_fields(
        &round.title,
        &round.task_description,
        round.starts_at,
        round.deadline_at,
        round.position,
    )
}

fn parse_round_status(value: &str) -> ApiResult<RoundStatus> {
    RoundStatus::from_str(value)
        .map_err(|_| ApiError::Validation("Round has invalid status".to_string()))
}

fn normalize_round_input(
    title: &str,
    task_description: &str,
    technology_requirements: Option<&str>,
    must_have: &[String],
    starts_at: DateTime<Utc>,
    deadline_at: DateTime<Utc>,
    position: i32,
) -> ApiResult<NormalizedRoundInput> {
    validate_round_fields(title, task_description, starts_at, deadline_at, position)?;

    Ok(NormalizedRoundInput {
        title: title.trim().to_string(),
        task_description: task_description.trim().to_string(),
        technology_requirements: technology_requirements
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned),
        must_have: normalize_requirements(must_have),
        starts_at,
        deadline_at,
        position,
    })
}

fn normalize_requirements(requirements: &[String]) -> Vec<String> {
    requirements
        .iter()
        .map(|requirement| requirement.trim())
        .filter(|requirement| !requirement.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

pub(crate) fn validate_round_fields(
    title: &str,
    task_description: &str,
    starts_at: DateTime<Utc>,
    deadline_at: DateTime<Utc>,
    position: i32,
) -> ApiResult<()> {
    if title.trim().chars().count() < 3 {
        return Err(ApiError::Validation(
            "Round title must contain at least 3 characters".to_string(),
        ));
    }

    if task_description.trim().chars().count() < 20 {
        return Err(ApiError::Validation(
            "Task description must contain at least 20 characters".to_string(),
        ));
    }

    if starts_at >= deadline_at {
        return Err(ApiError::Validation(
            "Round start must be before deadline".to_string(),
        ));
    }

    if position <= 0 {
        return Err(ApiError::Validation(
            "Round position must be greater than zero".to_string(),
        ));
    }

    Ok(())
}

pub(crate) fn validate_round_status_transition(
    current: RoundStatus,
    next: RoundStatus,
) -> ApiResult<()> {
    let allowed = matches!(
        (current, next),
        (RoundStatus::Draft, RoundStatus::Active)
            | (RoundStatus::Active, RoundStatus::SubmissionClosed)
            | (RoundStatus::SubmissionClosed, RoundStatus::Evaluated)
    );

    if allowed {
        Ok(())
    } else {
        Err(ApiError::Validation(format!(
            "Round status cannot change from {} to {}",
            current, next
        )))
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;

    fn date(day: u32) -> chrono::DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 5, day, 10, 0, 0).unwrap()
    }

    #[test]
    fn accepts_valid_round_dates() {
        let result = validate_round_fields(
            "MVP",
            "Build a working MVP with documented decisions",
            date(10),
            date(12),
            1,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_invalid_round_dates() {
        let result = validate_round_fields(
            "MVP",
            "Build a working MVP with documented decisions",
            date(12),
            date(10),
            1,
        );

        assert!(result.is_err());
    }

    #[test]
    fn accepts_valid_round_status_transition() {
        let result = validate_round_status_transition(RoundStatus::Draft, RoundStatus::Active);

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_invalid_round_status_transition() {
        let result = validate_round_status_transition(RoundStatus::Active, RoundStatus::Evaluated);

        assert!(result.is_err());
    }
}
