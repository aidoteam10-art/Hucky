use axum::{Json, Router, routing::get};
use serde::Serialize;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod error;
mod rounds;
mod state;
mod tournaments;
mod users;

use crate::state::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    println!("Connecting to db...");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to db");
    println!("Successfully connected to db!!!");

    let app_state = AppState { db: pool };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Збираємо додаток
    let app = Router::new()
        .route("/api/ping", get(ping_handler))
        .nest("/api/users", users::routes::users_routes())
        .merge(tournaments::routes::routes())
        .merge(rounds::routes::routes())
        .layer(cors)
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Backend runs on http://127.0.0.1:8080");

    axum::serve(listener, app).await.unwrap();
}

async fn ping_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "success".to_string(),
        message: "Api works!!!".to_string(),
    })
}
