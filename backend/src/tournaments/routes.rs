use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, patch, post},
};
use uuid::Uuid;

use crate::{error::ApiResult, state::AppState, users::auth::AuthenticatedUser};

use super::{
    dto::{
        ChangeTournamentStatusRequest, CreateTournamentRequest, CreateTournamentResponse,
        MyTournamentsResponse, TournamentDetailResponse, TournamentListQuery,
        TournamentListResponse, UpdateTournamentRequest,
    },
    service::TournamentService,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/tournaments",
            post(create_tournament_handler).get(list_tournaments_handler),
        )
        .route(
            "/api/tournaments/:id",
            get(get_tournament_handler).patch(update_tournament_handler),
        )
        .route(
            "/api/tournaments/:id/status",
            patch(change_tournament_status_handler),
        )
        .route("/api/me/tournaments", get(my_tournaments_handler))
}

async fn create_tournament_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateTournamentRequest>,
) -> ApiResult<Json<CreateTournamentResponse>> {
    let response = TournamentService::create_tournament(&state.db, user, payload).await?;
    Ok(Json(response))
}

async fn list_tournaments_handler(
    State(state): State<AppState>,
    Query(query): Query<TournamentListQuery>,
) -> ApiResult<Json<TournamentListResponse>> {
    let response = TournamentService::list_tournaments(&state.db, query).await?;
    Ok(Json(response))
}

async fn get_tournament_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<TournamentDetailResponse>> {
    let response = TournamentService::get_tournament(&state.db, id).await?;
    Ok(Json(response))
}

async fn update_tournament_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTournamentRequest>,
) -> ApiResult<Json<TournamentDetailResponse>> {
    let response = TournamentService::update_tournament(&state.db, user, id, payload).await?;
    Ok(Json(response))
}

async fn change_tournament_status_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<ChangeTournamentStatusRequest>,
) -> ApiResult<Json<CreateTournamentResponse>> {
    let response =
        TournamentService::change_tournament_status(&state.db, user, id, payload).await?;
    Ok(Json(response))
}

async fn my_tournaments_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> ApiResult<Json<MyTournamentsResponse>> {
    let response = TournamentService::my_tournaments(&state.db, user).await?;
    Ok(Json(response))
}
