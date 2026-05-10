use axum::{
    Json, Router,
    http::{HeaderValue, Method, header},
    routing::get,
};
use serde::Serialize;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod config;
mod error;
mod rounds;
mod state;
mod teams;
mod tournaments;
mod users;

use crate::config::Config;
use crate::state::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::init().clone();

    println!("Connecting to db...");
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to db");
    println!("Successfully connected to db!!!");

    let app_state = AppState { db: pool };

    let frontend_origin = HeaderValue::from_str(&config.frontend_origin)
        .expect("FRONTEND_ORIGIN must be a valid origin");
    let cors = CorsLayer::new()
        .allow_origin(frontend_origin)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    // Збираємо додаток
    let app = Router::new()
        .route("/api/ping", get(ping_handler))
        .nest("/api/users", users::routes::users_routes())
        .merge(tournaments::routes::routes())
        .merge(rounds::routes::routes())
        .merge(teams::routes::routes())
        .layer(cors)
        .with_state(app_state);

    let bind_addr = config.bind_addr();
    let listener = TcpListener::bind(&bind_addr).await.unwrap();
    println!("Backend runs on http://{}", bind_addr);

    axum::serve(listener, app).await.unwrap();
}

async fn ping_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "success".to_string(),
        message: "Api works!!!".to_string(),
    })
}
