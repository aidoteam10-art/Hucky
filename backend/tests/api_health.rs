use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use backend::{build_app, config::Config, state::AppState};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

fn test_config() -> Config {
    Config {
        database_url: "postgres://test:test@localhost:5432/test".to_string(),
        jwt_secret: "test-secret-that-is-long-enough-for-jwt".to_string(),
        server_host: "127.0.0.1".to_string(),
        server_port: 0,
        frontend_origin: "http://localhost:5173".to_string(),
        superadmin_email: None,
    }
}

fn test_app() -> axum::Router {
    let db = PgPoolOptions::new()
        .connect_lazy("postgres://test:test@localhost:5432/test")
        .expect("test database URL should be syntactically valid");

    build_app(AppState { db }, &test_config())
}

#[tokio::test]
async fn ping_returns_health_payload() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .uri("/api/ping")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(payload["status"], "success");
    assert_eq!(payload["message"], "Api works!!!");
}

#[tokio::test]
async fn protected_routes_reject_missing_token_before_database_work() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .uri("/api/users/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
