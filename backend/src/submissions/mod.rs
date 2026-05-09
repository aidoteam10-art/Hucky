use axum::{extract::{Path, State}, routing::{post, put}, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::state::AppState;


#[derive(Deserialize)]
pub struct SubmissionReq {
    pub github_url: String,
    pub video_demo_url: String,
    pub live_demo_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct SubmissionRes {
    pub id: Uuid,
    pub github_url: String,
    pub video_demo_url: String,
    pub live_demo_url: Option<String>,
    pub description: Option<String>,
    pub status: String,
}


async fn get_mock_user_id() -> Uuid {
    Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap_or_default()
}


async fn submit_work(
    State(state): State<AppState>,
    Path(round_id): Path<Uuid>,
    Json(payload): Json<SubmissionReq>,
) -> Result<Json<SubmissionRes>, (axum::http::StatusCode, String)> {
    let user_id = get_mock_user_id().await;

    if !payload.github_url.starts_with("http") || !payload.video_demo_url.starts_with("http") {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Посилання мають бути валідними URL (починатися з http/https)".into()));
    }
    if let Some(ref live_url) = payload.live_demo_url {
        if !live_url.is_empty() && !live_url.starts_with("http") {
            return Err((axum::http::StatusCode::BAD_REQUEST, "Live Demo має бути валідним URL".into()));
        }
    }
    if let Some(ref desc) = payload.description {
        if desc.chars().count() > 2000 {
            return Err((axum::http::StatusCode::BAD_REQUEST, "Опис не може перевищувати 2000 символів".into()));
        }
    }

    let team_id = sqlx::query_scalar!(
        "SELECT t.id FROM teams t
        JOIN team_memberships tm ON t.id = tm.team_id
        JOIN rounds r ON r.tournament_id = t.tournament_id
        WHERE r.id = $1 AND tm.user_id = $2 AND tm.status = 'accepted' AND tm.role = 'captain'",
        round_id, user_id
    )
    .fetch_optional(&state.db).await
    .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?
    .ok_or((axum::http::StatusCode::FORBIDDEN, "Тільки капітан команди може здавати роботу".into()))?;

    let accepted_members = sqlx::query_scalar!(
        "SELECT count(*) FROM team_memberships WHERE team_id = $1 AND status = 'accepted'",
        team_id
    )
    .fetch_one(&state.db).await.unwrap_or(Some(0)).unwrap_or(0);

    if accepted_members < 2 {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Команда повинна мати мінімум 2 підтверджених учасників для здачі роботи".into()));
    }

    let round = sqlx::query!("SELECT status, deadline_at FROM rounds WHERE id = $1", round_id)
        .fetch_one(&state.db).await
        .map_err(|_| (axum::http::StatusCode::NOT_FOUND, "Раунд не знайдено".into()))?;
        
    if round.status != "active" || round.deadline_at < chrono::Utc::now() {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Дедлайн минув або раунд неактивний".into()));
    }

    let existing_sub = sqlx::query!("SELECT locked_at FROM submissions WHERE round_id = $1 AND team_id = $2", round_id, team_id)
        .fetch_optional(&state.db).await.unwrap_or(None);
        
    if let Some(sub) = existing_sub {
        if sub.locked_at.is_some() {
            return Err((axum::http::StatusCode::FORBIDDEN, "Робота вже заблокована для перевірки".into()));
        }
    }

    let submission = sqlx::query_as!(
        SubmissionRes,
        r#"
        INSERT INTO submissions (round_id, team_id, github_url, video_demo_url, live_demo_url, description)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (round_id, team_id)
        DO UPDATE SET github_url = $3, video_demo_url = $4, live_demo_url = $5, description = $6, updated_at = NOW()
        RETURNING id, github_url, video_demo_url, live_demo_url, description, status
        "#,
        round_id, team_id, payload.github_url, payload.video_demo_url, payload.live_demo_url, payload.description
    )
    .fetch_one(&state.db).await
    .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Помилка збереження".into()))?;

    Ok(Json(submission))
}

async fn get_submission(
    State(state): State<AppState>,
    Path(round_id): Path<Uuid>,
) -> Result<Json<SubmissionRes>, (axum::http::StatusCode, String)> {
    let user_id = get_mock_user_id().await;
    
    let sub = sqlx::query_as!(
        SubmissionRes,
        "SELECT s.id, s.github_url, s.video_demo_url, s.live_demo_url, s.description, s.status
        FROM submissions s
        JOIN team_memberships tm ON s.team_id = tm.team_id
        WHERE s.round_id = $1 AND tm.user_id = $2 AND tm.status = 'accepted'",
        round_id, user_id
    )
    .fetch_optional(&state.db).await
    .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?
    .ok_or((axum::http::StatusCode::NOT_FOUND, "Робота не знайдена".into()))?;

    Ok(Json(sub))
}

async fn lock_submissions(
    State(state): State<AppState>,
    Path(round_id): Path<Uuid>,
) -> Result<axum::http::StatusCode, (axum::http::StatusCode, String)> {
    let user_id = get_mock_user_id().await;

    let is_organizer = sqlx::query_scalar!(
        "SELECT 1 FROM tournament_staff_roles tsr
         JOIN rounds r ON r.tournament_id = tsr.tournament_id
         WHERE r.id = $1 AND tsr.user_id = $2 AND tsr.role = 'organizer'",
        round_id, user_id
    )
    .fetch_optional(&state.db).await
    .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;

    if is_organizer.is_none() {
        return Err((axum::http::StatusCode::FORBIDDEN, "Тільки організатор може блокувати роботи".into()));
    }

    let mut tx = state.db.begin().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Tx Error".into()))?;
    
    sqlx::query!("UPDATE submissions SET status = 'locked', locked_at = NOW() WHERE round_id = $1", round_id)
        .execute(&mut *tx).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;
        
    sqlx::query!("UPDATE rounds SET status = 'submission_closed' WHERE id = $1", round_id)
        .execute(&mut *tx).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;
        
    tx.commit().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Tx Error".into()))?;

    Ok(axum::http::StatusCode::OK)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/rounds/:id/submission", put(submit_work).get(get_submission))
        .route("/rounds/:id/submissions/lock", post(lock_submissions))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_submit_work_after_deadline_fails(pool: PgPool) {
        let state = axum::extract::State(AppState { db: pool.clone() });
        let round_id_mock = Uuid::new_v4(); 
        
        let payload = axum::Json(SubmissionReq {
            github_url: "https://github.com/test/repo".into(),
            video_demo_url: "https://youtube.com/watch?v=123".into(),
            live_demo_url: None,
            description: Some("Test submission".into()),
        });

        let response = submit_work(state, axum::extract::Path(round_id_mock), payload).await;
        assert!(response.is_err());
    }
}