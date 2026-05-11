use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, patch, post},
};
use uuid::Uuid;

use crate::{
    error::ApiResult,
    state::AppState,
    users::auth::{AuthenticatedUser, OptionalAuthenticatedUser},
};

use super::{
    dto::{
        ChangeRoundStatusRequest, CreateRoundRequest, RoundDetailResponse, RoundListResponse,
        UpdateRoundRequest,
    },
    service::RoundService,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/tournaments/:id/rounds",
            post(create_round_handler).get(list_rounds_handler),
        )
        .route(
            "/api/rounds/:id",
            get(get_round_handler).patch(update_round_handler),
        )
        .route("/api/rounds/:id/status", patch(change_round_status_handler))
}

async fn create_round_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(tournament_id): Path<Uuid>,
    Json(payload): Json<CreateRoundRequest>,
) -> ApiResult<Json<RoundDetailResponse>> {
    let response = RoundService::create_round(&state.db, user, tournament_id, payload).await?;
    Ok(Json(response))
}

async fn list_rounds_handler(
    State(state): State<AppState>,
    OptionalAuthenticatedUser(user): OptionalAuthenticatedUser,
    Path(tournament_id): Path<Uuid>,
) -> ApiResult<Json<RoundListResponse>> {
    let response = RoundService::list_rounds(&state.db, user, tournament_id).await?;
    Ok(Json(response))
}

async fn get_round_handler(
    State(state): State<AppState>,
    OptionalAuthenticatedUser(user): OptionalAuthenticatedUser,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<RoundDetailResponse>> {
    let response = RoundService::get_round(&state.db, user, id).await?;
    Ok(Json(response))
}

async fn update_round_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRoundRequest>,
) -> ApiResult<Json<RoundDetailResponse>> {
    let response = RoundService::update_round(&state.db, user, id, payload).await?;
    Ok(Json(response))
}

async fn change_round_status_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<ChangeRoundStatusRequest>,
) -> ApiResult<Json<RoundDetailResponse>> {
    let response = RoundService::change_round_status(&state.db, user, id, payload).await?;
    Ok(Json(response))
}
