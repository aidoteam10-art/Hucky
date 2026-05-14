use axum::{
    Json, Router,
    http::{HeaderValue, Method, header},
    routing::get,
};
use serde::Serialize;
use tower_http::cors::CorsLayer;

pub mod automation;
pub mod certificates;
pub mod config;
pub mod error;
pub mod evaluations;
pub mod jury;
pub mod leaderboard;
pub mod rounds;
pub mod state;
pub mod submissions;
pub mod teams;
pub mod tournaments;
pub mod users;

use crate::{config::Config, state::AppState};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

pub fn build_app(app_state: AppState, config: &Config) -> Router {
    let frontend_origin =
        HeaderValue::from_str(&config.frontend_origin).expect("FRONTEND_ORIGIN must be valid");
    let cors = CorsLayer::new()
        .allow_origin(frontend_origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    Router::new()
        .route("/api/ping", get(ping_handler))
        .nest("/api/users", users::routes::users_routes())
        .merge(users::routes::admin_routes())
        .merge(tournaments::routes::routes())
        .merge(rounds::routes::routes())
        .merge(teams::routes::routes())
        .merge(submissions::routes())
        .merge(jury::routes())
        .merge(evaluations::routes())
        .merge(leaderboard::routes())
        .merge(certificates::routes::routes())
        .layer(cors)
        .with_state(app_state)
}

async fn ping_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "success".to_string(),
        message: "Api works!!!".to_string(),
    })
}
