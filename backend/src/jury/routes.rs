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
        AddJuryRequest, AssignmentDetailResponse, AssignmentListResponse,
        GenerateAssignmentsRequest, GenerateAssignmentsResponse, JuryListResponse,
    },
    service::JuryService,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/tournaments/:id/jury",
            post(add_jury_handler).get(list_jury_handler),
        )
        .route(
            "/api/tournaments/:id/jury/:user_id",
            delete(remove_jury_handler),
        )
        .route(
            "/api/rounds/:id/assignments/generate",
            post(generate_assignments_handler),
        )
        .route("/api/jury/assignments", get(my_assignments_handler))
        .route("/api/jury/assignments/:id", get(assignment_detail_handler))
}

async fn add_jury_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(tournament_id): Path<Uuid>,
    Json(payload): Json<AddJuryRequest>,
) -> ApiResult<Json<JuryListResponse>> {
    let response = JuryService::add_jury(&state.db, user, tournament_id, payload).await?;
    Ok(Json(response))
}

async fn list_jury_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(tournament_id): Path<Uuid>,
) -> ApiResult<Json<JuryListResponse>> {
    let response = JuryService::list_jury(&state.db, user, tournament_id).await?;
    Ok(Json(response))
}

async fn remove_jury_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((tournament_id, target_user_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<Json<serde_json::Value>> {
    JuryService::remove_jury(&state.db, user, tournament_id, target_user_id).await?;
    Ok(Json(json!({ "status": "removed" })))
}

async fn generate_assignments_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(round_id): Path<Uuid>,
    Json(payload): Json<GenerateAssignmentsRequest>,
) -> ApiResult<Json<GenerateAssignmentsResponse>> {
    let response = JuryService::generate_assignments(&state.db, user, round_id, payload).await?;
    Ok(Json(response))
}

async fn my_assignments_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> ApiResult<Json<AssignmentListResponse>> {
    let response = JuryService::my_assignments(&state.db, user).await?;
    Ok(Json(response))
}

async fn assignment_detail_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(assignment_id): Path<Uuid>,
) -> ApiResult<Json<AssignmentDetailResponse>> {
    let response = JuryService::assignment_detail(&state.db, user, assignment_id).await?;
    Ok(Json(response))
}
