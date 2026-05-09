use axum::{extract::{Path, State}, routing::{get, post, delete}, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;
use crate::state::AppState;
use crate::users::model::Claims;

#[derive(Deserialize)]
pub struct AddJuryReq { pub email: String }

#[derive(Serialize)]
pub struct JuryListItem {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
}

#[derive(Deserialize)]
pub struct GenerateAssignmentsReq {
    pub reviews_per_submission: i64,
    pub max_assignments_per_jury: i64,
}

#[derive(Deserialize)]
pub struct SubmitEvaluationReq {
    pub scores: Vec<ScoreReq>,
    pub comment: Option<String>,
}

#[derive(Deserialize)]
pub struct ScoreReq {
    pub criterion_id: Uuid,
    pub score: i32,
}

#[derive(Serialize)]
pub struct AssignmentListItem {
    pub id: Uuid,
    pub status: String,
    pub team_name: String,
    pub tournament_title: String,
    pub round_title: String,
}

#[derive(Serialize)]
pub struct TeamInfo {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize)]
pub struct SubmissionInfo {
    pub github_url: String,
    pub video_demo_url: String,
    pub live_demo_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct CriterionRes {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: String,
    pub max_score: i32,
    pub score: Option<i32>,
}

#[derive(Serialize)]
pub struct AssignmentDetailRes {
    pub id: Uuid,
    pub status: String,
    pub team: TeamInfo,
    pub submission: SubmissionInfo,
    pub criteria: Vec<CriterionRes>,
    pub comment: Option<String>,
}

async fn add_jury(
    State(state): State<AppState>, 
    claims: Claims, 
    Path(tournament_id): Path<Uuid>, 
    Json(payload): Json<AddJuryReq>,
) -> Result<axum::http::StatusCode, (axum::http::StatusCode, String)> {
    let current_user_id = claims.sub;

    let is_organizer = sqlx::query_scalar!(
        "SELECT 1 FROM tournament_staff_roles WHERE tournament_id = $1 AND user_id = $2 AND role = 'organizer'",
        tournament_id, current_user_id
    ).fetch_optional(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;

    if is_organizer.is_none() {
        return Err((axum::http::StatusCode::FORBIDDEN, "Тільки організатор може додавати журі".into()));
    }

    let target_user_id = sqlx::query_scalar!("SELECT id FROM users WHERE email = LOWER($1)", payload.email)
        .fetch_optional(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?
        .ok_or((axum::http::StatusCode::NOT_FOUND, "Користувача не знайдено".into()))?;

    let res = sqlx::query!(
        "INSERT INTO tournament_staff_roles (tournament_id, user_id, role) VALUES ($1, $2, 'jury') ON CONFLICT DO NOTHING",
        tournament_id, target_user_id
    ).execute(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;
    
    if res.rows_affected() == 0 {
        return Err((axum::http::StatusCode::CONFLICT, "Цей користувач вже є журі в цьому турнірі".into()));
    }

    Ok(axum::http::StatusCode::OK)
}

async fn get_jury_list(
    State(state): State<AppState>, Path(tournament_id): Path<Uuid>,
) -> Result<Json<Vec<JuryListItem>>, (axum::http::StatusCode, String)> {
    let jury_list = sqlx::query_as!(
        JuryListItem,
        "SELECT u.id, u.email, u.full_name 
         FROM users u JOIN tournament_staff_roles tsr ON u.id = tsr.user_id
         WHERE tsr.tournament_id = $1 AND tsr.role = 'jury'", tournament_id
    ).fetch_all(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;

    Ok(Json(jury_list))
}

async fn remove_jury(
    State(state): State<AppState>, 
    claims: Claims, 
    Path((tournament_id, user_id_to_remove)): Path<(Uuid, Uuid)>,
) -> Result<axum::http::StatusCode, (axum::http::StatusCode, String)> {
    let current_user_id = claims.sub;

    let is_organizer = sqlx::query_scalar!(
        "SELECT 1 FROM tournament_staff_roles WHERE tournament_id = $1 AND user_id = $2 AND role = 'organizer'",
        tournament_id, current_user_id
    ).fetch_optional(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;

    if is_organizer.is_none() {
        return Err((axum::http::StatusCode::FORBIDDEN, "Тільки організатор може видаляти журі".into()));
    }

    sqlx::query!("DELETE FROM tournament_staff_roles WHERE tournament_id = $1 AND user_id = $2 AND role = 'jury'", tournament_id, user_id_to_remove)
        .execute(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn get_jury_assignments(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<AssignmentListItem>>, (axum::http::StatusCode, String)> {
    let user_id = claims.sub;
    
    let records = sqlx::query_as!(
        AssignmentListItem,
        r#"SELECT ja.id, ja.status, t.name as team_name, tr.title as tournament_title, r.title as round_title
        FROM jury_assignments ja
        JOIN submissions s ON ja.submission_id = s.id JOIN teams t ON s.team_id = t.id
        JOIN rounds r ON s.round_id = r.id JOIN tournaments tr ON r.tournament_id = tr.id
        WHERE ja.jury_user_id = $1 ORDER BY ja.assigned_at DESC"#, user_id
    ).fetch_all(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;

    Ok(Json(records))
}

async fn get_jury_assignment_detail(
    State(state): State<AppState>, 
    claims: Claims, 
    Path(assignment_id): Path<Uuid>,
) -> Result<Json<AssignmentDetailRes>, (axum::http::StatusCode, String)> {
    let user_id = claims.sub;

    let detail = sqlx::query!(
        r#"SELECT ja.id, ja.status, t.id as team_id, t.name as team_name, s.github_url, s.video_demo_url, 
               s.live_demo_url, s.description, r.tournament_id
        FROM jury_assignments ja JOIN submissions s ON ja.submission_id = s.id
        JOIN teams t ON s.team_id = t.id JOIN rounds r ON s.round_id = r.id
        WHERE ja.id = $1 AND ja.jury_user_id = $2"#, assignment_id, user_id
    ).fetch_optional(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?
    .ok_or((axum::http::StatusCode::NOT_FOUND, "Призначення не знайдено або доступ заборонено".into()))?;

    let criteria_records = sqlx::query!(
        r#"SELECT c.id, c.code, c.name, c.description, c.max_score, es.score as "score?"
           FROM evaluation_criteria c
           LEFT JOIN evaluation_scores es ON c.id = es.criterion_id AND es.assignment_id = $1
           WHERE c.tournament_id = $2 ORDER BY c.position ASC"#,
        assignment_id, detail.tournament_id
    ).fetch_all(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;

    let criteria = criteria_records.into_iter().map(|c| CriterionRes {
        id: c.id,
        code: c.code,
        name: c.name,
        description: c.description,
        max_score: c.max_score,
        score: c.score,
    }).collect();

    let comment = sqlx::query_scalar!("SELECT comment FROM evaluation_comments WHERE assignment_id = $1", assignment_id)
        .fetch_optional(&state.db).await.unwrap_or(None).flatten();

    Ok(Json(AssignmentDetailRes {
        id: detail.id, 
        status: detail.status, 
        team: TeamInfo { id: detail.team_id, name: detail.team_name },
        submission: SubmissionInfo {
            github_url: detail.github_url,
            video_demo_url: detail.video_demo_url,
            live_demo_url: detail.live_demo_url,
            description: detail.description,
        },
        criteria,
        comment,
    }))
}

async fn generate_assignments(
    State(state): State<AppState>, 
    claims: Claims, 
    Path(round_id): Path<Uuid>, 
    Json(payload): Json<GenerateAssignmentsReq>,
) -> Result<axum::http::StatusCode, (axum::http::StatusCode, String)> {
    if payload.reviews_per_submission <= 0 || payload.max_assignments_per_jury <= 0 {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Параметри генерації мають бути більшими за 0".into()));
    }

    let user_id = claims.sub;

    let round = sqlx::query!("SELECT tournament_id, status FROM rounds WHERE id = $1", round_id)
        .fetch_optional(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?
        .ok_or((axum::http::StatusCode::NOT_FOUND, "Round not found".into()))?;

    let is_organizer = sqlx::query_scalar!(
        "SELECT 1 FROM tournament_staff_roles WHERE tournament_id = $1 AND user_id = $2 AND role = 'organizer'",
        round.tournament_id, user_id
    ).fetch_optional(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;

    if is_organizer.is_none() {
        return Err((axum::http::StatusCode::FORBIDDEN, "Тільки організатор може генерувати призначення".into()));
    }
    if round.status != "submission_closed" {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Генерація можлива лише після закриття прийому робіт (submission_closed)".into()));
    }

    let submissions = sqlx::query!("SELECT id, team_id FROM submissions WHERE round_id = $1", round_id)
        .fetch_all(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;

    let mut jury_list = sqlx::query!(
        r#"SELECT user_id, 
           COALESCE((SELECT count(*) FROM jury_assignments WHERE jury_user_id = tsr.user_id), 0) as "load!" 
           FROM tournament_staff_roles tsr 
           WHERE tsr.tournament_id = $1 AND tsr.role = 'jury'"#, 
        round.tournament_id
    ).fetch_all(&state.db).await.unwrap_or_default();

    if jury_list.is_empty() { 
        return Err((axum::http::StatusCode::BAD_REQUEST, "В турнірі немає журі".into())); 
    }

    let mut tx = state.db.begin().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Tx Error".into()))?;

    for sub in submissions {
        jury_list.sort_by_key(|j| j.load);
        let mut assigned = 0;
        
        let team_members: Vec<Uuid> = sqlx::query_scalar!("SELECT user_id FROM team_memberships WHERE team_id = $1 AND status = 'accepted'", sub.team_id)
            .fetch_all(&mut *tx).await.unwrap_or_default();

        for jury in jury_list.iter_mut() {
            if assigned >= payload.reviews_per_submission { break; }
            if jury.load >= payload.max_assignments_per_jury { continue; }
            if team_members.contains(&jury.user_id) { continue; }

            let res = sqlx::query!(
                "INSERT INTO jury_assignments (round_id, submission_id, jury_user_id) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
                round_id, sub.id, jury.user_id
            ).execute(&mut *tx).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;
            
            if res.rows_affected() > 0 { jury.load += 1; assigned += 1; }
        }

        if assigned < payload.reviews_per_submission {
            let _ = tx.rollback().await;
            return Err((axum::http::StatusCode::BAD_REQUEST, format!("Не вистачає незалежних журі для чесного оцінювання команди.")));
        }
    }
    
    tx.commit().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Tx Error".into()))?;
    Ok(axum::http::StatusCode::OK)
}

async fn submit_evaluation(
    State(state): State<AppState>, 
    claims: Claims, 
    Path(assignment_id): Path<Uuid>, 
    Json(payload): Json<SubmitEvaluationReq>,
) -> Result<axum::http::StatusCode, (axum::http::StatusCode, String)> {
    let user_id = claims.sub;
    
    let assignment = sqlx::query!(
        "SELECT ja.status, ja.jury_user_id, r.id as round_id, r.tournament_id FROM jury_assignments ja JOIN rounds r ON ja.round_id = r.id WHERE ja.id = $1", 
        assignment_id
    ).fetch_optional(&state.db).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?
    .ok_or((axum::http::StatusCode::NOT_FOUND, "Призначення не знайдено".into()))?;
        
    if assignment.jury_user_id != user_id { return Err((axum::http::StatusCode::FORBIDDEN, "Це не ваше призначення".into())); }
    if assignment.status == "completed" { return Err((axum::http::StatusCode::BAD_REQUEST, "Вже оцінено".into())); }

    let criteria_count = sqlx::query_scalar!("SELECT count(*) FROM evaluation_criteria WHERE tournament_id = $1", assignment.tournament_id)
        .fetch_one(&state.db).await.unwrap_or(Some(0)).unwrap_or(0);

    if payload.scores.len() as i64 != criteria_count {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Потрібно оцінити всі критерії турніру".into()));
    }

    let mut unique_criteria = HashSet::new();
    for s in &payload.scores {
        if !unique_criteria.insert(s.criterion_id) { return Err((axum::http::StatusCode::BAD_REQUEST, "Дублювання оцінок заборонено".into())); }
        if s.score < 0 || s.score > 100 { return Err((axum::http::StatusCode::BAD_REQUEST, "Оцінка має бути від 0 до 100".into())); }
    }

    let mut tx = state.db.begin().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Tx Error".into()))?;
    
    for s in payload.scores {
        sqlx::query!("INSERT INTO evaluation_scores (assignment_id, criterion_id, score) VALUES ($1, $2, $3)", assignment_id, s.criterion_id, s.score)
            .execute(&mut *tx).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;
    }
    
    if let Some(c) = payload.comment {
        sqlx::query!("INSERT INTO evaluation_comments (assignment_id, comment) VALUES ($1, $2)", assignment_id, c)
            .execute(&mut *tx).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;
    }

    sqlx::query!("UPDATE jury_assignments SET status = 'completed', completed_at = NOW() WHERE id = $1", assignment_id)
        .execute(&mut *tx).await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB Error".into()))?;
        
    tx.commit().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Tx Error".into()))?;
    
    let pending_count = sqlx::query_scalar!("SELECT count(*) FROM jury_assignments WHERE round_id = $1 AND status = 'pending'", assignment.round_id)
        .fetch_one(&state.db).await.unwrap_or(Some(1));

    if pending_count == Some(0) {
        let _ = sqlx::query!("UPDATE rounds SET status = 'evaluated' WHERE id = $1", assignment.round_id).execute(&state.db).await;
    }

    Ok(axum::http::StatusCode::OK)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tournaments/:id/jury", post(add_jury).get(get_jury_list))
        .route("/tournaments/:id/jury/:user_id", delete(remove_jury))
        .route("/rounds/:id/assignments/generate", post(generate_assignments))
        .route("/jury/assignments", get(get_jury_assignments))
        .route("/jury/assignments/:id", get(get_jury_assignment_detail))
        .route("/jury/assignments/:id/evaluation", post(submit_evaluation))
}