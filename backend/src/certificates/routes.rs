use axum::{Json, Router, extract::State, routing::get};

use crate::{error::ApiResult, state::AppState, users::auth::AuthenticatedUser};

use super::{dto::CertificatesResponse, service::CertificateService};

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/me/certificates", get(my_certificates_handler))
}

async fn my_certificates_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> ApiResult<Json<CertificatesResponse>> {
    let response = CertificateService::list_for_user(&state.db, user).await?;
    Ok(Json(response))
}
