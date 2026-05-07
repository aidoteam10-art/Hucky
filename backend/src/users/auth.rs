use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;

use super::model::Claims;

pub struct AuthenticatedUser {
    pub user_id: uuid::Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];

                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret("супер_секретний_ключ".as_ref()),
                    &Validation::default(),
                )
                .map_err(|_| {
                    let err = Json(json!({"error": "Токен невалідний або прострочений"}));
                    (StatusCode::UNAUTHORIZED, err).into_response()
                })?;

                return Ok(AuthenticatedUser {
                    user_id: token_data.claims.sub,
                });
            }
        }

        let err = Json(json!({"error": "Немає токену авторизації"}));
        Err((StatusCode::UNAUTHORIZED, err).into_response())
    }
}
