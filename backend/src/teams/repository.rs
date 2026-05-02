use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use chrono::Utc;
use crate::teams::model::{Team, TeamMembership, TeamInvitation};

pub async fn create_team_with_captain(
    pool: &PgPool,
    tournament_id: Uuid,
    creator_id: Uuid,
    name: String,
    organization: Option<String>,
    contact: Option<String>,
) -> Result<Team, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let team = sqlx::query_as::<_, Team>(
        r#"INSERT INTO teams (tournament_id, name, organization, contact, created_by)
           VALUES ($1, $2, $3, $4, $5) RETURNING *"#
    )
    .bind(tournament_id).bind(name).bind(organization).bind(contact).bind(creator_id)
    .fetch_one(&mut *tx).await?;

    sqlx::query(
        r#"INSERT INTO team_memberships (team_id, tournament_id, user_id, role, status, joined_at)
           VALUES ($1, $2, $3, 'captain', 'accepted', NOW())"#
    )
    .bind(team.id).bind(tournament_id).bind(creator_id)
    .execute(&mut *tx).await?;

    tx.commit().await?;
    Ok(team)
}

pub async fn get_teams_by_tournament(pool: &PgPool, tournament_id: Uuid) -> Result<Vec<Team>, sqlx::Error> {
    sqlx::query_as::<_, Team>(
        "SELECT * FROM teams WHERE tournament_id = $1 ORDER BY created_at DESC"
    )
    .bind(tournament_id)
    .fetch_all(pool)
    .await
}

pub async fn get_team_by_id(pool: &PgPool, team_id: Uuid) -> Result<Option<Team>, sqlx::Error> {
    sqlx::query_as::<_, Team>("SELECT * FROM teams WHERE id = $1")
        .bind(team_id)
        .fetch_optional(pool)
        .await
}

pub async fn get_team_members(pool: &PgPool, team_id: Uuid) -> Result<Vec<TeamMembership>, sqlx::Error> {
    sqlx::query_as::<_, TeamMembership>(
        "SELECT * FROM team_memberships WHERE team_id = $1"
    )
    .bind(team_id)
    .fetch_all(pool)
    .await
}

pub async fn create_invitation(
    executor: &mut Transaction<'_, Postgres>, // Використовуємо транзакцію для надійності
    team_id: Uuid,
    tournament_id: Uuid,
    email: String,
    invited_by: Uuid,
) -> Result<TeamInvitation, sqlx::Error> {
    let token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + chrono::Duration::days(7);

    sqlx::query_as::<_, TeamInvitation>(
        r#"INSERT INTO team_invitations (team_id, tournament_id, email, invited_by, status, token, expires_at)
           VALUES ($1, $2, $3, $4, 'pending', $5, $6) RETURNING *"#
    )
    .bind(team_id).bind(tournament_id).bind(email.to_lowercase()).bind(invited_by).bind(token).bind(expires_at)
    .fetch_one(&mut **executor).await
}

pub async fn remove_member(pool: &PgPool, team_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM team_memberships WHERE team_id = $1 AND user_id = $2 AND role != 'captain'")
        .bind(team_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_user_invitations(pool: &PgPool, email: String) -> Result<Vec<TeamInvitation>, sqlx::Error> {
    sqlx::query_as::<_, TeamInvitation>(
        "SELECT * FROM team_invitations WHERE email = $1 AND status = 'pending' ORDER BY created_at DESC"
    )
    .bind(email.to_lowercase())
    .fetch_all(pool)
    .await
}
// Перевірка, чи є користувач учасником команди (Пункт 15.2)
pub async fn is_team_member(pool: &PgPool, user_id: Uuid, team_id: Uuid) -> Result<bool, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM team_memberships WHERE user_id = $1 AND team_id = $2 AND status = 'accepted')",
        user_id, team_id
    )
    .fetch_one(pool)
    .await?;
    Ok(row.exists.unwrap_or(false))
}

// Перевірка, чи є користувач капітаном (Пункт 15.3)
pub async fn is_team_captain(pool: &PgPool, user_id: Uuid, team_id: Uuid) -> Result<bool, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM team_memberships WHERE user_id = $1 AND team_id = $2 AND role = 'captain' AND status = 'accepted')",
        user_id, team_id
    )
    .fetch_one(pool).await?;
    Ok(row.exists.unwrap_or(false))
}

// Підрахунок прийнятих учасників (Пункт 15.4)
pub async fn count_accepted_members(pool: &PgPool, team_id: Uuid) -> Result<i64, sqlx::Error> {
    let row = sqlx::query!("SELECT COUNT(*) FROM team_memberships WHERE team_id = $1 AND status = 'accepted'", team_id)
        .fetch_one(pool).await?;
    Ok(row.count.unwrap_or(0))
}
pub async fn update_team(pool: &PgPool, team_id: Uuid, name: String, organization: Option<String>, contact: Option<String>) -> Result<Team, sqlx::Error> {
    sqlx::query_as::<_, Team>(
        "UPDATE teams SET name = $1, organization = $2, contact = $3, updated_at = NOW() WHERE id = $4 RETURNING *"
    )
    .bind(name).bind(organization).bind(contact).bind(team_id)
    .fetch_one(pool).await
}
pub async fn get_user_memberships(pool: &PgPool, user_id: Uuid) -> Result<Vec<TeamMembership>, sqlx::Error> {
    sqlx::query_as::<_, TeamMembership>(
        "SELECT * FROM team_memberships WHERE user_id = $1 AND status = 'accepted'"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}