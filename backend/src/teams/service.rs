use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;
use crate::teams::model::{Team, TeamMembership, TeamInvitation};
use crate::teams::repository;
use crate::teams::routes::CreateTeamRequest;

pub async fn register_new_team(
    pool: &PgPool,
    tournament_id: Uuid,
    creator_id: Uuid,
    payload: CreateTeamRequest,
) -> Result<Team, String> {
    if payload.name.trim().len() < 2 { return Err("Назва занадто коротка".to_string()); }

    let mut unique_emails = payload.member_emails.clone();
    unique_emails.sort();
    unique_emails.dedup();

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let team = repository::create_team_with_captain(pool, tournament_id, creator_id, payload.name, payload.organization, payload.contact)
        .await.map_err(|e| e.to_string())?;

    for email in unique_emails {
        repository::create_invitation(&mut tx, team.id, tournament_id, email, creator_id)
            .await.map_err(|e| format!("Помилка інвайту: {}", e))?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(team)
}

pub async fn get_tournament_teams(pool: &PgPool, tournament_id: Uuid) -> Result<Vec<Team>, String> {
    repository::get_teams_by_tournament(pool, tournament_id)
        .await
        .map_err(|e| format!("Не вдалося отримати список команд: {}", e))
}

pub async fn get_team_details(pool: &PgPool, team_id: Uuid) -> Result<(Team, Vec<TeamMembership>), String> {
    let team = repository::get_team_by_id(pool, team_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Команду не знайдено".to_string())?;

    let members = repository::get_team_members(pool, team_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok((team, members))
}

pub async fn invite_member(
    pool: &PgPool,
    team_id: Uuid,
    inviter_id: Uuid,
    email: String,
) -> Result<TeamInvitation, String> {
    let team = repository::get_team_by_id(pool, team_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Команду не знайдено")?;

    let is_captain = repository::is_team_captain(pool, inviter_id, team_id)
        .await
        .map_err(|e| e.to_string())?;

    if !is_captain {
        return Err("Тільки капітан може надсилати запрошення".to_string());
    }

    // Потрібна транзакція, бо репозиторій тепер очікує Transaction
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let invitation = repository::create_invitation(&mut tx, team_id, team.tournament_id, email, inviter_id)
        .await
        .map_err(|e| format!("Не вдалося створити запрошення: {}", e))?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(invitation)
}

pub async fn accept_invitation(pool: &PgPool, token: String) -> Result<(), String> {
    let invitation = sqlx::query_as::<_, TeamInvitation>(
        "SELECT * FROM team_invitations WHERE token = $1 AND status = 'pending'"
    )
    .bind(&token)
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?
    .ok_or("Запрошення не знайдено або вже використано")?;

    if invitation.expires_at < Utc::now() {
        return Err("Термін дії запрошення вичерпано".to_string());
    }

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    sqlx::query("UPDATE team_invitations SET status = 'accepted' WHERE id = $1")
        .bind(invitation.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(user_id) = invitation.invited_user_id {
        sqlx::query(
            "INSERT INTO team_memberships (team_id, tournament_id, user_id, role, status, joined_at)
             VALUES ($1, $2, $3, 'member', 'accepted', NOW())"
        )
        .bind(invitation.team_id)
        .bind(invitation.tournament_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn get_my_invitations(pool: &PgPool, email: String) -> Result<Vec<TeamInvitation>, String> {
    repository::get_user_invitations(pool, email)
        .await
        .map_err(|e| format!("Не вдалося отримати запрошення: {}", e))
}

pub async fn kick_member(
    pool: &PgPool,
    team_id: Uuid,
    captain_id: Uuid,
    target_user_id: Uuid
) -> Result<(), String> {
    let members = repository::get_team_members(pool, team_id).await.map_err(|e| e.to_string())?;
    let is_captain = members.iter().any(|m| m.user_id == captain_id && m.role == "captain");

    if !is_captain {
        return Err("Тільки капітан може видаляти учасників".to_string());
    }

    repository::remove_member(pool, team_id, target_user_id)
        .await
        .map_err(|e| format!("Помилка при видаленні: {}", e))
}
pub async fn decline_invitation(pool: &PgPool, token: String) -> Result<(), String> {
    // 14.1 Перевірка існування інвайту
    let invitation = sqlx::query_as::<_, TeamInvitation>(
        "SELECT * FROM team_invitations WHERE token = $1 AND status = 'pending'"
    )
    .bind(&token)
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?
    .ok_or("Запрошення не знайдено")?;

    // 14.2 Оновлення статусу на 'declined' без створення membership
    sqlx::query("UPDATE team_invitations SET status = 'declined' WHERE id = $1")
        .bind(invitation.id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
pub async fn update_team_details(
    pool: &PgPool,
    team_id: Uuid,
    user_id: Uuid,
    name: String,
    organization: Option<String>,
    contact: Option<String>,
) -> Result<Team, String> {
    if !repository::is_team_captain(pool, user_id, team_id).await.unwrap_or(false) {
        return Err("Тільки капітан може редагувати дані".to_string());
    }
    if name.trim().len() < 2 { return Err("Назва занадто коротка".to_string()); }

    repository::update_team(pool, team_id, name, organization, contact)
        .await.map_err(|e| e.to_string())
}
pub async fn get_my_teams(pool: &PgPool, user_id: Uuid) -> Result<Vec<Team>, String> {
    let memberships = repository::get_user_memberships(pool, user_id)
        .await
        .map_err(|e| e.to_string())?;

    let mut teams = Vec::new();
    for ms in memberships {
        if let Ok(Some(team)) = repository::get_team_by_id(pool, ms.team_id).await {
            teams.push(team);
        }
    }
    Ok(teams)
}
pub async fn ensure_team_is_ready_for_tournament(pool: &PgPool, team_id: Uuid) -> Result<bool, String> {
    let count = repository::count_accepted_members(pool, team_id).await.map_err(|e| e.to_string())?;
    if count < 2 { return Err("Мінімум 2 учасники".to_string()); }
    Ok(true)
}
