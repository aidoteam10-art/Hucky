use axum::{
    Json, async_trait,
    extract::FromRequestParts,
    http::{HeaderMap, StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde_json::json;

use crate::config::Config;

use super::model::Claims;

#[derive(Clone, Copy)]
pub struct AuthenticatedUser {
    pub user_id: uuid::Uuid,
}

pub struct OptionalAuthenticatedUser(pub Option<AuthenticatedUser>);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match decode_user(&parts.headers) {
            Ok(Some(user)) => Ok(user),
            Ok(None) => {
                let err = Json(json!({"error": "No authorization token"}));
                Err((StatusCode::UNAUTHORIZED, err).into_response())
            }
            Err(response) => Err(response),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match decode_user(&parts.headers) {
            Ok(user) => Ok(OptionalAuthenticatedUser(user)),
            Err(response) => Err(response),
        }
    }
}

fn decode_user(headers: &HeaderMap) -> Result<Option<AuthenticatedUser>, Response> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    let Some(auth_header) = auth_header else {
        return Ok(None);
    };

    if !auth_header.starts_with("Bearer ") {
        return Ok(None);
    }

    let token = &auth_header[7..];
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(Config::get().jwt_secret_bytes()),
        &Validation::default(),
    )
    .map_err(|_| {
        let err = Json(json!({"error": "Token is invalid or expired"}));
        (StatusCode::UNAUTHORIZED, err).into_response()
    })?;

    Ok(Some(AuthenticatedUser {
        user_id: token_data.claims.sub,
    }))
}
