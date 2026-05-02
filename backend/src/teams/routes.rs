use axum::{extract::{Path, State}, Json, routing::{post, get, delete, patch}, Router};
use sqlx::PgPool;
use uuid::Uuid;
use serde::Deserialize;
use crate::teams::{service, model::Team};

#[derive(Deserialize)]
pub struct InviteRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub organization: Option<String>,
    pub contact: Option<String>,
    pub member_emails: Vec<String>,
}

pub fn team_routes() -> Router<PgPool> {
    Router::new()
        .route("/tournaments/:id/teams", post(create_team_handler).get(get_tournament_teams_handler)) // Додано GET
        .route("/teams/:team_id", get(get_team_handler).patch(update_team_handler))
        .route("/my-teams", get(get_my_teams_handler))
        .route("/teams/:id/invitations", post(send_invitation_handler)) // Додано POST для інвайтів
        .route("/my-invitations", get(get_my_invitations_handler)) // Додано GET для моїх інвайтів
        .route("/teams/:id/members/:user_id", delete(kick_member_handler))
        .route("/invitations/:token/accept", post(accept_invitation_handler))
        .route("/invitations/:token/decline", post(decline_invitation_handler))
}

async fn create_team_handler(
    State(pool): State<PgPool>,
    Path(tournament_id): Path<Uuid>,
    Json(payload): Json<CreateTeamRequest>,
) -> Result<Json<Team>, String> {
    let temporary_user_id = Uuid::new_v4(); // Тимчасово

    let team = service::register_new_team(
        &pool,
        tournament_id,
        temporary_user_id,
        payload, // Передаємо весь об'єкт
    ).await?;

    Ok(Json(team))
}

async fn get_tournament_teams_handler(
    State(pool): State<PgPool>,
    Path(tournament_id): Path<Uuid>,
) -> Result<Json<Vec<crate::teams::model::Team>>, String> {
    let teams = service::get_tournament_teams(&pool, tournament_id).await?;
    Ok(Json(teams))
}

async fn get_team_handler(
    State(pool): State<PgPool>,
    Path(team_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, String> {
    let (team, members) = service::get_team_details(&pool, team_id).await?;
    Ok(Json(serde_json::json!({
        "team": team,
        "members": members
    })))
}

async fn send_invitation_handler(
    State(pool): State<PgPool>,
    Path(team_id): Path<Uuid>,
    Json(payload): Json<InviteRequest>,
) -> Result<Json<crate::teams::model::TeamInvitation>, String> {
    let temporary_captain_id = Uuid::new_v4();
    let invitation = service::invite_member(&pool, team_id, temporary_captain_id, payload.email).await?;
    Ok(Json(invitation))
}

async fn accept_invitation_handler(
    State(pool): State<PgPool>,
    Path(token): Path<String>,
) -> Result<String, String> {
    service::accept_invitation(&pool, token).await?;
    Ok("Ви успішно приєдналися до команди!".to_string())
}

async fn get_my_invitations_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<crate::teams::model::TeamInvitation>>, String> {
    let user_email = "test@example.com".to_string();
    let invitations = service::get_my_invitations(&pool, user_email).await?;
    Ok(Json(invitations))
}

async fn kick_member_handler(
    State(pool): State<PgPool>,
    Path((team_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<String, String> {
    let captain_id = Uuid::new_v4();
    service::kick_member(&pool, team_id, captain_id, user_id).await?;
    Ok("Учасника видалено".to_string())
}
async fn decline_invitation_handler(
    State(pool): State<PgPool>,
    Path(token): Path<String>,
) -> Result<String, String> {
    service::decline_invitation(&pool, token).await?;
    Ok("Запрошення відхилено".to_string())
}
async fn update_team_handler(
    State(pool): State<PgPool>,
    Path(team_id): Path<Uuid>,
    Json(payload): Json<CreateTeamRequest>,
) -> Result<Json<Team>, String> {
    let mock_user_id = Uuid::nil(); // Замінити на реальний ID з сесії
    let team = service::update_team_details(&pool, team_id, mock_user_id, payload.name, payload.organization, payload.contact).await?;
    Ok(Json(team))
}

async fn get_my_teams_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Team>>, String> {
    let temporary_user_id = Uuid::new_v4(); // Тимчасово
    let teams = service::get_my_teams(&pool, temporary_user_id).await?;
    Ok(Json(teams))
}