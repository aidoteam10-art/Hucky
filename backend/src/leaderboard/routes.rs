use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use uuid::Uuid;

use crate::{error::ApiResult, state::AppState, users::auth::OptionalAuthenticatedUser};

use super::{dto::LeaderboardResponse, service::LeaderboardService};

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/api/tournaments/:id/leaderboard",
        get(get_leaderboard_handler),
    )
}

async fn get_leaderboard_handler(
    State(state): State<AppState>,
    OptionalAuthenticatedUser(user): OptionalAuthenticatedUser,
    Path(tournament_id): Path<Uuid>,
) -> ApiResult<Json<LeaderboardResponse>> {
    let response = LeaderboardService::get(&state.db, user, tournament_id).await?;
    Ok(Json(response))
}
