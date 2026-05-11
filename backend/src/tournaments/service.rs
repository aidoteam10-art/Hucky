use std::str::FromStr;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult},
    rounds::{model::NewRound, repository::RoundRepository, service::RoundService},
    users::{auth::AuthenticatedUser, model::AccountRole, repository::UserRepository},
};

use super::{
    dto::{
        ChangeTournamentStatusRequest, CreateTournamentRequest, CreateTournamentResponse,
        MyTournamentsResponse, TournamentDetailResponse, TournamentListQuery,
        TournamentListResponse, UpdateTournamentRequest,
    },
    model::{NewTournament, Tournament, TournamentListFilter, TournamentStatus, UpdateTournament},
    repository::TournamentRepository,
};

pub struct TournamentService;

impl TournamentService {
    pub async fn create_tournament(
        db: &PgPool,
        user: AuthenticatedUser,
        payload: CreateTournamentRequest,
    ) -> ApiResult<CreateTournamentResponse> {
        Self::require_can_create_tournament(db, user).await?;
        validate_tournament_fields(
            &payload.title,
            &payload.description,
            &payload.rules,
            payload.registration_starts_at,
            payload.registration_ends_at,
            payload.starts_at,
            payload.max_teams,
        )?;

        let first_round = RoundService::normalize_first_round(&payload.first_round, Some(1))?;
        let mut tx = db.begin().await?;

        let tournament = TournamentRepository::insert(
            &mut tx,
            NewTournament {
                organizer_id: user.user_id,
                title: payload.title.trim(),
                description: payload.description.trim(),
                rules: payload.rules.trim(),
                registration_starts_at: payload.registration_starts_at,
                registration_ends_at: payload.registration_ends_at,
                starts_at: payload.starts_at,
                max_teams: payload.max_teams,
            },
        )
        .await?;

        TournamentRepository::insert_organizer_role(&mut tx, tournament.id, user.user_id).await?;

        let round = RoundRepository::insert(
            &mut tx,
            NewRound {
                tournament_id: tournament.id,
                title: &first_round.title,
                task_description: &first_round.task_description,
                technology_requirements: first_round.technology_requirements.as_deref(),
                starts_at: first_round.starts_at,
                deadline_at: first_round.deadline_at,
                position: first_round.position,
            },
        )
        .await?;

        RoundRepository::insert_requirements(&mut tx, round.id, &first_round.must_have).await?;
        tx.commit().await?;

        Ok(CreateTournamentResponse {
            id: tournament.id,
            status: TournamentStatus::Draft,
            title: tournament.title,
        })
    }

    pub async fn list_tournaments(
        db: &PgPool,
        query: TournamentListQuery,
    ) -> ApiResult<TournamentListResponse> {
        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(6).clamp(1, 50);

        let filter = TournamentListFilter {
            status: query.status,
            search: query.search,
            page,
            per_page,
        };

        let (items, total) = TournamentRepository::list(db, &filter).await?;
        let total_pages = if total == 0 {
            0
        } else {
            (total + per_page - 1) / per_page
        };

        Ok(TournamentListResponse {
            items,
            page,
            per_page,
            total,
            total_pages,
        })
    }

    pub async fn my_tournaments(
        db: &PgPool,
        user: AuthenticatedUser,
    ) -> ApiResult<MyTournamentsResponse> {
        let role = UserRepository::account_role(db, user.user_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Current user not found".to_string()))?;

        if role != AccountRole::Organiser {
            return Err(ApiError::Forbidden(
                "Only organiser can view managed tournaments".to_string(),
            ));
        }

        let items = TournamentRepository::list_for_organizer(db, user.user_id).await?;

        Ok(MyTournamentsResponse { items })
    }

    pub async fn get_tournament(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> ApiResult<TournamentDetailResponse> {
        let tournament = find_tournament(db, tournament_id).await?;
        build_detail_response(db, tournament).await
    }

    pub async fn update_tournament(
        db: &PgPool,
        user: AuthenticatedUser,
        tournament_id: Uuid,
        payload: UpdateTournamentRequest,
    ) -> ApiResult<TournamentDetailResponse> {
        Self::require_tournament_organizer(db, tournament_id, user).await?;
        let existing = find_tournament(db, tournament_id).await?;

        let title = payload.title.unwrap_or(existing.title);
        let description = payload.description.unwrap_or(existing.description);
        let rules = payload.rules.unwrap_or(existing.rules);
        let registration_starts_at = payload
            .registration_starts_at
            .unwrap_or(existing.registration_starts_at);
        let registration_ends_at = payload
            .registration_ends_at
            .unwrap_or(existing.registration_ends_at);
        let starts_at = payload.starts_at.unwrap_or(existing.starts_at);
        let ends_at = payload.ends_at.or(existing.ends_at);
        let max_teams = payload.max_teams.or(existing.max_teams);

        validate_tournament_fields(
            &title,
            &description,
            &rules,
            registration_starts_at,
            registration_ends_at,
            starts_at,
            max_teams,
        )?;

        let updated = TournamentRepository::update(
            db,
            tournament_id,
            UpdateTournament {
                title: title.trim(),
                description: description.trim(),
                rules: rules.trim(),
                registration_starts_at,
                registration_ends_at,
                starts_at,
                ends_at,
                max_teams,
            },
        )
        .await?;

        build_detail_response(db, updated).await
    }

    pub async fn change_tournament_status(
        db: &PgPool,
        user: AuthenticatedUser,
        tournament_id: Uuid,
        payload: ChangeTournamentStatusRequest,
    ) -> ApiResult<CreateTournamentResponse> {
        Self::require_tournament_organizer(db, tournament_id, user).await?;
        let tournament = find_tournament(db, tournament_id).await?;
        let current = parse_tournament_status(&tournament.status)?;

        validate_tournament_status_transition(current, payload.status)?;

        if current == TournamentStatus::Registration && payload.status == TournamentStatus::Running
        {
            let rounds_count = TournamentRepository::count_rounds(db, tournament_id).await?;
            if rounds_count == 0 {
                return Err(ApiError::Validation(
                    "Tournament needs at least one round before it can start".to_string(),
                ));
            }
        }

        let updated =
            TournamentRepository::update_status(db, tournament_id, payload.status).await?;

        Ok(CreateTournamentResponse {
            id: updated.id,
            status: payload.status,
            title: updated.title,
        })
    }

    pub async fn require_tournament_organizer(
        db: &PgPool,
        tournament_id: Uuid,
        user: AuthenticatedUser,
    ) -> ApiResult<()> {
        let exists = TournamentRepository::is_organizer(db, tournament_id, user.user_id).await?;
        if exists {
            Ok(())
        } else {
            Err(ApiError::Forbidden(
                "Only tournament organizer can perform this action".to_string(),
            ))
        }
    }

    async fn require_can_create_tournament(db: &PgPool, user: AuthenticatedUser) -> ApiResult<()> {
        let role = UserRepository::account_role(db, user.user_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Current user not found".to_string()))?;

        if role.can_create_tournaments() {
            Ok(())
        } else {
            Err(ApiError::Forbidden(
                "Only organiser can create tournaments".to_string(),
            ))
        }
    }
}

async fn find_tournament(db: &PgPool, tournament_id: Uuid) -> ApiResult<Tournament> {
    TournamentRepository::find_by_id(db, tournament_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Tournament not found".to_string()))
}

async fn build_detail_response(
    db: &PgPool,
    tournament: Tournament,
) -> ApiResult<TournamentDetailResponse> {
    let active_round = TournamentRepository::active_round(db, tournament.id).await?;
    let registered_teams = TournamentRepository::registered_teams(db, tournament.id).await?;
    let registered_teams_count = registered_teams.len() as i64;

    Ok(TournamentDetailResponse {
        id: tournament.id,
        title: tournament.title,
        description: tournament.description,
        rules: tournament.rules,
        status: parse_tournament_status(&tournament.status)?,
        registration_starts_at: tournament.registration_starts_at,
        registration_ends_at: tournament.registration_ends_at,
        starts_at: tournament.starts_at,
        ends_at: tournament.ends_at,
        max_teams: tournament.max_teams,
        registered_teams_count,
        registered_teams,
        active_round,
    })
}

fn parse_tournament_status(value: &str) -> ApiResult<TournamentStatus> {
    TournamentStatus::from_str(value)
        .map_err(|_| ApiError::Validation("Tournament has invalid status".to_string()))
}

pub(crate) fn validate_tournament_fields(
    title: &str,
    description: &str,
    rules: &str,
    registration_starts_at: chrono::DateTime<chrono::Utc>,
    registration_ends_at: chrono::DateTime<chrono::Utc>,
    starts_at: chrono::DateTime<chrono::Utc>,
    max_teams: Option<i32>,
) -> ApiResult<()> {
    if title.trim().chars().count() < 3 {
        return Err(ApiError::Validation(
            "Tournament title must contain at least 3 characters".to_string(),
        ));
    }

    if description.trim().chars().count() < 10 {
        return Err(ApiError::Validation(
            "Tournament description must contain at least 10 characters".to_string(),
        ));
    }

    if rules.trim().chars().count() < 10 {
        return Err(ApiError::Validation(
            "Tournament rules must contain at least 10 characters".to_string(),
        ));
    }

    if registration_starts_at >= registration_ends_at {
        return Err(ApiError::Validation(
            "Registration start must be before registration end".to_string(),
        ));
    }

    if registration_ends_at > starts_at {
        return Err(ApiError::Validation(
            "Registration end must be before or equal to tournament start".to_string(),
        ));
    }

    if matches!(max_teams, Some(value) if value <= 0) {
        return Err(ApiError::Validation(
            "Max teams must be greater than zero".to_string(),
        ));
    }

    Ok(())
}

pub(crate) fn validate_tournament_status_transition(
    current: TournamentStatus,
    next: TournamentStatus,
) -> ApiResult<()> {
    let allowed = matches!(
        (current, next),
        (TournamentStatus::Draft, TournamentStatus::Registration)
            | (TournamentStatus::Registration, TournamentStatus::Running)
            | (TournamentStatus::Registration, TournamentStatus::Draft)
            | (TournamentStatus::Running, TournamentStatus::Finished)
    );

    if allowed {
        Ok(())
    } else {
        Err(ApiError::Validation(format!(
            "Tournament status cannot change from {} to {}",
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
    fn accepts_valid_tournament_dates() {
        let result = validate_tournament_fields(
            "Spring Hackathon",
            "Detailed tournament description",
            "Tournament rules are clear",
            date(1),
            date(5),
            date(6),
            Some(20),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_invalid_registration_window() {
        let result = validate_tournament_fields(
            "Spring Hackathon",
            "Detailed tournament description",
            "Tournament rules are clear",
            date(5),
            date(1),
            date(6),
            Some(20),
        );

        assert!(result.is_err());
    }

    #[test]
    fn accepts_valid_status_transition() {
        let result = validate_tournament_status_transition(
            TournamentStatus::Draft,
            TournamentStatus::Registration,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_invalid_status_transition() {
        let result = validate_tournament_status_transition(
            TournamentStatus::Draft,
            TournamentStatus::Finished,
        );

        assert!(result.is_err());
    }
}
