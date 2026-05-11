use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post},
};
use serde_json::json;
use uuid::Uuid;

use crate::{error::ApiResult, state::AppState, users::auth::AuthenticatedUser};

use super::{
    dto::{
        CreateInvitationRequest, CreateTeamRequest, MyInvitationsResponse, MyTeamsResponse,
        TeamDetailResponse, TeamInvitationResponse, TeamSummaryResponse, TournamentTeamsResponse,
        UpdateTeamRequest,
    },
    service::TeamService,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/tournaments/:id/teams",
            post(create_team_handler).get(list_tournament_teams_handler),
        )
        .route(
            "/api/teams/:id",
            get(get_team_handler).patch(update_team_handler),
        )
        .route(
            "/api/teams/:id/invitations",
            post(create_invitation_handler),
        )
        .route(
            "/api/teams/:id/members/:user_id",
            delete(remove_member_handler),
        )
        .route("/api/me/invitations", get(my_invitations_handler))
        .route("/api/me/teams", get(my_teams_handler))
        .route(
            "/api/invitations/:id/accept",
            post(accept_invitation_handler),
        )
        .route(
            "/api/invitations/:id/decline",
            post(decline_invitation_handler),
        )
        .route("/api/invitations/:id", delete(delete_invitation_handler))
}

async fn create_team_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(tournament_id): Path<Uuid>,
    Json(payload): Json<CreateTeamRequest>,
) -> ApiResult<Json<TeamSummaryResponse>> {
    let response = TeamService::create_team(&state.db, user, tournament_id, payload).await?;
    Ok(Json(response))
}

async fn list_tournament_teams_handler(
    State(state): State<AppState>,
    Path(tournament_id): Path<Uuid>,
) -> ApiResult<Json<TournamentTeamsResponse>> {
    let response = TeamService::list_tournament_teams(&state.db, tournament_id).await?;
    Ok(Json(response))
}

async fn get_team_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(team_id): Path<Uuid>,
) -> ApiResult<Json<TeamDetailResponse>> {
    let response = TeamService::get_team(&state.db, user, team_id).await?;
    Ok(Json(response))
}

async fn update_team_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(team_id): Path<Uuid>,
    Json(payload): Json<UpdateTeamRequest>,
) -> ApiResult<Json<TeamSummaryResponse>> {
    let response = TeamService::update_team(&state.db, user, team_id, payload).await?;
    Ok(Json(response))
}

async fn create_invitation_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(team_id): Path<Uuid>,
    Json(payload): Json<CreateInvitationRequest>,
) -> ApiResult<Json<TeamInvitationResponse>> {
    let response = TeamService::create_invitation(&state.db, user, team_id, payload).await?;
    Ok(Json(response))
}

async fn my_invitations_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> ApiResult<Json<MyInvitationsResponse>> {
    let response = TeamService::my_invitations(&state.db, user).await?;
    Ok(Json(response))
}

async fn accept_invitation_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(invitation_id): Path<Uuid>,
) -> ApiResult<Json<TeamSummaryResponse>> {
    let response = TeamService::accept_invitation(&state.db, user, invitation_id).await?;
    Ok(Json(response))
}

async fn decline_invitation_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(invitation_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    TeamService::decline_invitation(&state.db, user, invitation_id).await?;
    Ok(Json(json!({ "status": "declined" })))
}

async fn delete_invitation_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(invitation_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    TeamService::delete_invitation(&state.db, user, invitation_id).await?;
    Ok(Json(json!({ "status": "cancelled" })))
}

async fn remove_member_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((team_id, target_user_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<Json<serde_json::Value>> {
    TeamService::remove_member(&state.db, user, team_id, target_user_id).await?;
    Ok(Json(json!({ "status": "removed" })))
}

async fn my_teams_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> ApiResult<Json<MyTeamsResponse>> {
    let response = TeamService::my_teams(&state.db, user).await?;
    Ok(Json(response))
}
