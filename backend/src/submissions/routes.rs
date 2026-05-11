use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{post, put},
};
use serde_json::json;
use uuid::Uuid;

use crate::{error::ApiResult, state::AppState, users::auth::AuthenticatedUser};

use super::{dto::SubmissionRequest, service::SubmissionService};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/rounds/:id/submission",
            put(submit_work_handler).get(get_submission_handler),
        )
        .route(
            "/api/rounds/:id/submissions/lock",
            post(lock_submissions_handler),
        )
}

async fn submit_work_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(round_id): Path<Uuid>,
    Json(payload): Json<SubmissionRequest>,
) -> ApiResult<Json<super::dto::SubmissionResponse>> {
    let response = SubmissionService::submit(&state.db, user, round_id, payload).await?;
    Ok(Json(response))
}

async fn get_submission_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(round_id): Path<Uuid>,
) -> ApiResult<Json<super::dto::SubmissionResponse>> {
    let response = SubmissionService::get(&state.db, user, round_id).await?;
    Ok(Json(response))
}

async fn lock_submissions_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(round_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    SubmissionService::lock(&state.db, user, round_id).await?;
    Ok(Json(json!({ "status": "submission_closed" })))
}
