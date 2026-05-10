use axum::http::StatusCode;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::Serialize;
use uuid::Uuid;

use super::auth::AuthenticatedUser;
use super::model::{LoginRequest, RegisterUserRequest};
use super::repository::UserRepository;
use super::service::UserService;
use crate::state::AppState;

#[derive(Serialize)]
pub struct CountResponse {
    pub count: Option<i64>,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub message: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<serde_json::Value>)> {
    match UserService::login_user(&state.db, payload).await {
        Ok(token) => Ok(Json(LoginResponse { token })),
        Err(err_msg) => {
            let error_response = serde_json::json!({
                "error" : err_msg
            });
            Err((StatusCode::UNAUTHORIZED, Json(error_response)))
        }
    }
}

pub async fn users_count_handler(State(state): State<AppState>) -> Json<CountResponse> {
    let count = UserRepository::count(&state.db).await.unwrap_or(0);

    Json(CountResponse { count: Some(count) })
}

pub async fn register_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<serde_json::Value>)> {
    match UserService::register_user(&state.db, payload).await {
        Ok(new_user_id) => Ok(Json(RegisterResponse {
            id: new_user_id,
            message: "Користувач успішно зареєстрований!".to_string(),
        })),
        Err(err_msg) => {
            let error_response = serde_json::json!({
                "error": err_msg
            });
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
}

#[derive(Serialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
}

pub async fn get_me_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<UserProfileResponse>, (StatusCode, Json<serde_json::Value>)> {
    match UserRepository::find_by_id(&state.db, user.user_id).await {
        Ok(Some(db_user)) => Ok(Json(UserProfileResponse {
            id: db_user.id,
            email: db_user.email,
            full_name: db_user.full_name,
        })),
        Ok(None) => {
            let error_response = serde_json::json!({"error": "Користувач не знайдений в БД"});
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(_) => {
            let error_response = serde_json::json!({"error": "Помилка бази даних"});
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

pub fn users_routes() -> Router<AppState> {
    Router::new()
        .route("/count", get(users_count_handler))
        .route("/register", post(register_user_handler))
        .route("/login", post(login_user_handler))
        .route("/me", get(get_me_handler))
}
