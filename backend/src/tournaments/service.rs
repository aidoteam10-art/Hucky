use std::str::FromStr;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    certificates::service::CertificateService,
    error::{ApiError, ApiResult},
    rounds::{model::NewRound, repository::RoundRepository, service::RoundService},
    users::{
        auth::{AuthenticatedUser, OptionalAuthenticatedUser},
        model::AccountRole,
        repository::UserRepository,
    },
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
        user: OptionalAuthenticatedUser,
        query: TournamentListQuery,
    ) -> ApiResult<TournamentListResponse> {
        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(6).clamp(1, 50);

        let filter = TournamentListFilter {
            status: query.status,
            search: query.search,
            viewer_id: user.0.map(|user| user.user_id),
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
        user: OptionalAuthenticatedUser,
        tournament_id: Uuid,
    ) -> ApiResult<TournamentDetailResponse> {
        let tournament = find_tournament(db, tournament_id).await?;
        ensure_can_view_tournament(&tournament, user.0)?;
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
        ensure_tournament_editable(&existing)?;

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
        let registered_teams =
            TournamentRepository::count_registered_teams(db, tournament_id).await?;
        validate_max_teams_capacity(max_teams, registered_teams)?;

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

        if current == TournamentStatus::Running && payload.status == TournamentStatus::Finished {
            ensure_tournament_can_finish(db, tournament_id).await?;
        }

        let updated =
            TournamentRepository::update_status(db, tournament_id, payload.status).await?;

        if current == TournamentStatus::Running && payload.status == TournamentStatus::Finished {
            CertificateService::generate_for_tournament(db, tournament_id).await?;
        }

        Ok(CreateTournamentResponse {
            id: updated.id,
            status: payload.status,
            title: updated.title,
        })
    }

    pub async fn delete_tournament(
        db: &PgPool,
        user: AuthenticatedUser,
        tournament_id: Uuid,
    ) -> ApiResult<()> {
        Self::require_tournament_organizer(db, tournament_id, user).await?;
        let tournament = find_tournament(db, tournament_id).await?;
        ensure_tournament_deletable(&tournament)?;

        let rows = TournamentRepository::delete(db, tournament_id).await?;
        if rows == 0 {
            return Err(ApiError::NotFound("Tournament not found".to_string()));
        }

        Ok(())
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

fn ensure_can_view_tournament(
    tournament: &Tournament,
    user: Option<AuthenticatedUser>,
) -> ApiResult<()> {
    if tournament.status != TournamentStatus::Draft.as_str() {
        return Ok(());
    }

    if user.is_some_and(|user| user.user_id == tournament.organizer_id) {
        return Ok(());
    }

    Err(ApiError::NotFound("Tournament not found".to_string()))
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

fn ensure_tournament_editable(tournament: &Tournament) -> ApiResult<()> {
    let status = parse_tournament_status(&tournament.status)?;
    if matches!(
        status,
        TournamentStatus::Draft | TournamentStatus::Registration
    ) {
        return Ok(());
    }

    Err(ApiError::Validation(
        "Tournament can be edited only before it starts".to_string(),
    ))
}

fn ensure_tournament_deletable(tournament: &Tournament) -> ApiResult<()> {
    let status = parse_tournament_status(&tournament.status)?;
    if status == TournamentStatus::Draft {
        return Ok(());
    }

    Err(ApiError::Validation(
        "Only draft tournaments can be deleted".to_string(),
    ))
}

async fn ensure_tournament_can_finish(db: &PgPool, tournament_id: Uuid) -> ApiResult<()> {
    let rounds_count = TournamentRepository::count_rounds(db, tournament_id).await?;
    let not_evaluated_count =
        TournamentRepository::count_rounds_not_evaluated(db, tournament_id).await?;
    validate_tournament_finish_preconditions(rounds_count, not_evaluated_count)
}

fn validate_tournament_finish_preconditions(
    rounds_count: i64,
    not_evaluated_count: i64,
) -> ApiResult<()> {
    if rounds_count == 0 {
        return Err(ApiError::Validation(
            "Tournament needs at least one evaluated round before it can finish".to_string(),
        ));
    }

    if not_evaluated_count > 0 {
        return Err(ApiError::Validation(
            "Tournament can be finished only after all rounds are evaluated".to_string(),
        ));
    }

    Ok(())
}

fn validate_max_teams_capacity(max_teams: Option<i32>, registered_teams: i64) -> ApiResult<()> {
    if let Some(max_teams) = max_teams {
        if registered_teams > i64::from(max_teams) {
            return Err(ApiError::Validation(
                "Max teams cannot be lower than currently registered teams".to_string(),
            ));
        }
    }

    Ok(())
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

    #[test]
    fn rejects_tournament_edit_after_it_starts() {
        let running = tournament_with_owner(Uuid::new_v4(), TournamentStatus::Running);
        let finished = tournament_with_owner(Uuid::new_v4(), TournamentStatus::Finished);

        assert!(ensure_tournament_editable(&running).is_err());
        assert!(ensure_tournament_editable(&finished).is_err());
    }

    #[test]
    fn allows_tournament_edit_before_it_starts() {
        let draft = tournament_with_owner(Uuid::new_v4(), TournamentStatus::Draft);
        let registration = tournament_with_owner(Uuid::new_v4(), TournamentStatus::Registration);

        assert!(ensure_tournament_editable(&draft).is_ok());
        assert!(ensure_tournament_editable(&registration).is_ok());
    }

    #[test]
    fn allows_only_draft_tournament_deletion() {
        let draft = tournament_with_owner(Uuid::new_v4(), TournamentStatus::Draft);
        let registration = tournament_with_owner(Uuid::new_v4(), TournamentStatus::Registration);
        let running = tournament_with_owner(Uuid::new_v4(), TournamentStatus::Running);
        let finished = tournament_with_owner(Uuid::new_v4(), TournamentStatus::Finished);

        assert!(ensure_tournament_deletable(&draft).is_ok());
        assert!(ensure_tournament_deletable(&registration).is_err());
        assert!(ensure_tournament_deletable(&running).is_err());
        assert!(ensure_tournament_deletable(&finished).is_err());
    }

    #[test]
    fn rejects_finish_until_all_rounds_are_evaluated() {
        assert!(validate_tournament_finish_preconditions(0, 0).is_err());
        assert!(validate_tournament_finish_preconditions(3, 1).is_err());
        assert!(validate_tournament_finish_preconditions(3, 0).is_ok());
    }

    #[test]
    fn rejects_max_teams_below_registered_count() {
        assert!(validate_max_teams_capacity(Some(2), 3).is_err());
        assert!(validate_max_teams_capacity(Some(3), 3).is_ok());
        assert!(validate_max_teams_capacity(None, 30).is_ok());
    }

    #[test]
    fn hides_draft_tournament_from_non_owner() {
        let tournament = tournament_with_owner(Uuid::new_v4(), TournamentStatus::Draft);
        let user = AuthenticatedUser {
            user_id: Uuid::new_v4(),
        };

        assert!(ensure_can_view_tournament(&tournament, Some(user)).is_err());
        assert!(ensure_can_view_tournament(&tournament, None).is_err());
    }

    #[test]
    fn allows_draft_tournament_owner() {
        let owner_id = Uuid::new_v4();
        let tournament = tournament_with_owner(owner_id, TournamentStatus::Draft);
        let user = AuthenticatedUser { user_id: owner_id };

        assert!(ensure_can_view_tournament(&tournament, Some(user)).is_ok());
    }

    #[test]
    fn allows_published_tournament_without_user() {
        let tournament = tournament_with_owner(Uuid::new_v4(), TournamentStatus::Registration);

        assert!(ensure_can_view_tournament(&tournament, None).is_ok());
    }

    fn tournament_with_owner(owner_id: Uuid, status: TournamentStatus) -> Tournament {
        Tournament {
            id: Uuid::new_v4(),
            organizer_id: owner_id,
            title: "Spring Hackathon".to_string(),
            description: "Detailed tournament description".to_string(),
            rules: "Tournament rules are clear".to_string(),
            status: status.as_str().to_string(),
            registration_starts_at: date(1),
            registration_ends_at: date(5),
            starts_at: date(6),
            ends_at: None,
            max_teams: Some(20),
        }
    }
}
