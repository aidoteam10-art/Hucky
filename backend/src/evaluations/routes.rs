use axum::{
    Json, Router,
    extract::{Path, State},
    routing::post,
};
use uuid::Uuid;

use crate::{error::ApiResult, state::AppState, users::auth::AuthenticatedUser};

use super::{
    dto::{SubmitEvaluationRequest, SubmitEvaluationResponse},
    service::EvaluationService,
};

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/api/jury/assignments/:id/evaluation",
        post(submit_evaluation_handler),
    )
}

async fn submit_evaluation_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(assignment_id): Path<Uuid>,
    Json(payload): Json<SubmitEvaluationRequest>,
) -> ApiResult<Json<SubmitEvaluationResponse>> {
    let response = EvaluationService::submit(&state.db, user, assignment_id, payload).await?;
    Ok(Json(response))
}
