use axum::http::StatusCode;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, patch, post},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use super::auth::AuthenticatedUser;
use super::model::{AccountRole, LoginRequest, RegisterUserRequest};
use super::repository::UserRepository;
use super::service::UserService;
use crate::error::{ApiError, ApiResult};
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
    pub role: String,
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
            role: db_user.account_role,
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

#[derive(Debug, Deserialize)]
pub struct AdminUserListQuery {
    pub role: Option<String>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleRequest {
    pub role: AccountRole,
}

#[derive(Debug, Serialize)]
pub struct AdminUserResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub role: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct AdminUserListResponse {
    pub items: Vec<AdminUserResponse>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/api/admin/users", get(list_admin_users_handler))
        .route(
            "/api/admin/users/:id/role",
            patch(update_admin_user_role_handler),
        )
}

async fn list_admin_users_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(query): Query<AdminUserListQuery>,
) -> ApiResult<Json<AdminUserListResponse>> {
    require_admin(&state, user).await?;

    let role = query
        .role
        .as_deref()
        .map(AccountRole::from_str)
        .transpose()
        .map_err(|_| ApiError::Validation("Invalid role filter".to_string()))?;
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let (items, total) =
        UserRepository::list_admin_users(&state.db, role, query.search.as_deref(), page, per_page)
            .await?;
    let total_pages = if total == 0 {
        0
    } else {
        (total + per_page - 1) / per_page
    };

    Ok(Json(AdminUserListResponse {
        items: items.into_iter().map(admin_user_response).collect(),
        page,
        per_page,
        total,
        total_pages,
    }))
}

async fn update_admin_user_role_handler(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(target_user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRoleRequest>,
) -> ApiResult<Json<AdminUserResponse>> {
    require_admin(&state, user).await?;
    if target_user_id == user.user_id && payload.role != AccountRole::Admin {
        return Err(ApiError::Validation(
            "Admin cannot remove their own admin role".to_string(),
        ));
    }

    let updated = UserRepository::set_role(&state.db, target_user_id, payload.role)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    Ok(Json(admin_user_response(updated)))
}

async fn require_admin(state: &AppState, user: AuthenticatedUser) -> ApiResult<()> {
    match UserRepository::account_role(&state.db, user.user_id).await? {
        Some(AccountRole::Admin) => Ok(()),
        _ => Err(ApiError::Forbidden(
            "Only admin can perform this action".to_string(),
        )),
    }
}

fn admin_user_response(row: super::repository::AdminUserRow) -> AdminUserResponse {
    AdminUserResponse {
        id: row.id,
        email: row.email,
        full_name: row.full_name,
        role: row.account_role,
        created_at: row.created_at,
    }
}
