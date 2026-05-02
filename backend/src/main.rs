use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use tokio::net::TcpListener;
use sqlx::postgres::PgPoolOptions; // Для підключення до БД
use std::env;
use dotenvy::dotenv;

// Підключаємо твої модулі
mod teams;
mod users;
mod tournaments;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Завантажуємо .env файл

    // Отримуємо URL бази даних з оточення
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Створюємо пул підключень до бази
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Збираємо додаток
    let app = Router::new()
        .route("/api/ping", get(ping_handler))
        // Підключаємо твої роути команд і передаємо їм доступ до бази[cite: 3]
        .nest("/api", teams::routes::team_routes())
        .layer(cors)
        .with_state(pool); // "Впорскуємо" базу в додаток

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("🚀 Backend runs on http://127.0.0.1:8080");

    axum::serve(listener, app).await.unwrap();
}

async fn ping_handler() -> Json<HealthResponse> {
    let response = HealthResponse {
        status: "success".to_string(),
        message: "Api works!!!".to_string(),
    };
    Json(response)
}