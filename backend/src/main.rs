use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use tokio::net::TcpListener;
use sqlx::PgPool;

// Підключаємо наші нові модулі
mod state;
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
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

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

    // Збираємо наш головний роутер з різних частин
    let app = Router::new()
        .route("/api/ping", get(ping_handler))
        // Підключаємо роути користувачів (вони будуть за адресою /api/users/...)
        .nest("/api/users", users::routes::users_routes())
        .layer(cors)
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Backend runs on http://127.0.0.1:8080");

    axum::serve(listener, app).await.unwrap();
}

async fn ping_handler() -> Json<HealthResponse> {
    let response = HealthResponse {
        status: "success".to_string(),
        message: "Api works!!!".to_string(),
    };
    Json(response)
}