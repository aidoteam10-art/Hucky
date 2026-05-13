use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::tournaments::model::Tournament;

use super::model::{
    MembershipStatus, MyInvitationRow, MyTeamRow, NewInvitation, NewTeam, Team, TeamInvitation,
    TeamMemberRow, TeamMembership, TeamRole, TournamentTeamRow, UserIdentity,
};

pub struct TeamRepository;

impl TeamRepository {
    pub async fn insert_team(
        tx: &mut Transaction<'_, Postgres>,
        team: NewTeam<'_>,
    ) -> Result<Team, sqlx::Error> {
        sqlx::query_as::<_, Team>(
            "INSERT INTO teams (tournament_id, name, organization, contact, created_by)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tournament_id, name, organization, contact, created_by, created_at, updated_at",
        )
        .bind(team.tournament_id)
        .bind(team.name)
        .bind(team.organization)
        .bind(team.contact)
        .bind(team.created_by)
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn insert_membership(
        tx: &mut Transaction<'_, Postgres>,
        team_id: Uuid,
        tournament_id: Uuid,
        user_id: Uuid,
        role: TeamRole,
        status: MembershipStatus,
    ) -> Result<TeamMembership, sqlx::Error> {
        sqlx::query_as::<_, TeamMembership>(
            "INSERT INTO team_memberships
                (team_id, tournament_id, user_id, role, status, joined_at)
            VALUES ($1, $2, $3, $4, $5, CASE WHEN $5 = 'accepted' THEN NOW() ELSE NULL END)
            RETURNING id, team_id, tournament_id, user_id, role, status, joined_at, created_at, updated_at",
        )
        .bind(team_id)
        .bind(tournament_id)
        .bind(user_id)
        .bind(role.as_str())
        .bind(status.as_str())
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn insert_invitation(
        tx: &mut Transaction<'_, Postgres>,
        invitation: NewInvitation<'_>,
    ) -> Result<TeamInvitation, sqlx::Error> {
        sqlx::query_as::<_, TeamInvitation>(
            "INSERT INTO team_invitations
                (team_id, tournament_id, email, invited_user_id, invited_by, status, token, expires_at)
            VALUES ($1, $2, $3, $4, $5, 'pending', $6, $7)
            RETURNING id, team_id, tournament_id, email, invited_user_id, invited_by, status,
                expires_at, created_at, updated_at",
        )
        .bind(invitation.team_id)
        .bind(invitation.tournament_id)
        .bind(invitation.email)
        .bind(invitation.invited_user_id)
        .bind(invitation.invited_by)
        .bind(invitation.token)
        .bind(invitation.expires_at)
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn find_tournament(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<Option<Tournament>, sqlx::Error> {
        sqlx::query_as::<_, Tournament>(
            "SELECT id, organizer_id, title, description, rules, status, registration_starts_at,
                registration_ends_at, starts_at, ends_at, max_teams
            FROM tournaments
            WHERE id = $1",
        )
        .bind(tournament_id)
        .fetch_optional(db)
        .await
    }

    pub async fn find_team(db: &PgPool, team_id: Uuid) -> Result<Option<Team>, sqlx::Error> {
        sqlx::query_as::<_, Team>(
            "SELECT id, tournament_id, name, organization, contact, created_by, created_at, updated_at
            FROM teams
            WHERE id = $1",
        )
        .bind(team_id)
        .fetch_optional(db)
        .await
    }

    pub async fn find_invitation(
        db: &PgPool,
        invitation_id: Uuid,
    ) -> Result<Option<TeamInvitation>, sqlx::Error> {
        sqlx::query_as::<_, TeamInvitation>(
            "SELECT id, team_id, tournament_id, email, invited_user_id, invited_by, status,
                expires_at, created_at, updated_at
            FROM team_invitations
            WHERE id = $1",
        )
        .bind(invitation_id)
        .fetch_optional(db)
        .await
    }

    pub async fn find_user_by_id(
        db: &PgPool,
        user_id: Uuid,
    ) -> Result<Option<UserIdentity>, sqlx::Error> {
        sqlx::query_as::<_, UserIdentity>(
            "SELECT id, email
            FROM users
            WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(db)
        .await
    }

    pub async fn find_user_by_email(
        db: &PgPool,
        email: &str,
    ) -> Result<Option<UserIdentity>, sqlx::Error> {
        sqlx::query_as::<_, UserIdentity>(
            "SELECT id, email
            FROM users
            WHERE lower(email) = lower($1)",
        )
        .bind(email)
        .fetch_optional(db)
        .await
    }

    pub async fn count_teams_in_tournament(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)::bigint
            FROM teams
            WHERE tournament_id = $1",
        )
        .bind(tournament_id)
        .fetch_one(db)
        .await
    }

    pub async fn count_accepted_members(db: &PgPool, team_id: Uuid) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)::bigint
            FROM team_memberships
            WHERE team_id = $1 AND status = 'accepted'",
        )
        .bind(team_id)
        .fetch_one(db)
        .await
    }

    pub async fn accepted_membership_in_tournament(
        db: &PgPool,
        user_id: Uuid,
        tournament_id: Uuid,
    ) -> Result<Option<TeamMembership>, sqlx::Error> {
        sqlx::query_as::<_, TeamMembership>(
            "SELECT id, team_id, tournament_id, user_id, role, status, joined_at, created_at, updated_at
            FROM team_memberships
            WHERE user_id = $1 AND tournament_id = $2 AND status = 'accepted'",
        )
        .bind(user_id)
        .bind(tournament_id)
        .fetch_optional(db)
        .await
    }

    pub async fn is_team_member(
        db: &PgPool,
        user_id: Uuid,
        team_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(
                SELECT 1
                FROM team_memberships
                WHERE user_id = $1 AND team_id = $2 AND status = 'accepted'
            )",
        )
        .bind(user_id)
        .bind(team_id)
        .fetch_one(db)
        .await
    }

    pub async fn is_team_captain(
        db: &PgPool,
        user_id: Uuid,
        team_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(
                SELECT 1
                FROM team_memberships
                WHERE user_id = $1
                    AND team_id = $2
                    AND role = 'captain'
                    AND status = 'accepted'
            )",
        )
        .bind(user_id)
        .bind(team_id)
        .fetch_one(db)
        .await
    }

    pub async fn is_tournament_organizer(
        db: &PgPool,
        tournament_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (
                SELECT 1
                FROM tournaments t
                WHERE t.id = $1
                    AND EXISTS (
                        SELECT 1
                        FROM users u
                        WHERE u.id = $2
                            AND u.account_role = 'organiser'
                    )
                    AND (
                        t.organizer_id = $2
                        OR EXISTS (
                            SELECT 1
                            FROM tournament_staff_roles tsr
                            WHERE tsr.tournament_id = t.id
                                AND tsr.user_id = $2
                                AND tsr.role = 'organizer'
                        )
                    )
            )",
        )
        .bind(tournament_id)
        .bind(user_id)
        .fetch_one(db)
        .await
    }

    pub async fn tournament_teams(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentTeamRow>, sqlx::Error> {
        sqlx::query_as::<_, TournamentTeamRow>(
            "SELECT
                tm.id,
                tm.name,
                tm.organization,
                COUNT(ms.user_id) FILTER (WHERE ms.status = 'accepted')::bigint AS members_count,
                captain.user_id AS captain_id,
                captain_user.full_name AS captain_full_name
            FROM teams tm
            LEFT JOIN team_memberships ms ON ms.team_id = tm.id
            LEFT JOIN team_memberships captain
                ON captain.team_id = tm.id
                AND captain.role = 'captain'
                AND captain.status = 'accepted'
            LEFT JOIN users captain_user ON captain_user.id = captain.user_id
            WHERE tm.tournament_id = $1
            GROUP BY tm.id, captain.user_id, captain_user.full_name
            ORDER BY tm.created_at DESC",
        )
        .bind(tournament_id)
        .fetch_all(db)
        .await
    }

    pub async fn team_members(
        db: &PgPool,
        team_id: Uuid,
    ) -> Result<Vec<TeamMemberRow>, sqlx::Error> {
        sqlx::query_as::<_, TeamMemberRow>(
            "SELECT u.id AS user_id, u.full_name, u.email, ms.role, ms.status
            FROM team_memberships ms
            JOIN users u ON u.id = ms.user_id
            WHERE ms.team_id = $1
            ORDER BY
                CASE WHEN ms.role = 'captain' THEN 0 ELSE 1 END,
                u.full_name ASC",
        )
        .bind(team_id)
        .fetch_all(db)
        .await
    }

    pub async fn pending_invitations(
        db: &PgPool,
        team_id: Uuid,
    ) -> Result<Vec<TeamInvitation>, sqlx::Error> {
        sqlx::query_as::<_, TeamInvitation>(
            "SELECT i.id, i.team_id, i.tournament_id, i.email, i.invited_user_id, i.invited_by,
                i.status, i.expires_at, i.created_at, i.updated_at
            FROM team_invitations i
            JOIN tournaments t ON t.id = i.tournament_id
            WHERE i.team_id = $1
                AND i.status = 'pending'
                AND i.expires_at > NOW()
                AND t.status = 'registration'
                AND t.registration_ends_at > NOW()
            ORDER BY i.created_at DESC",
        )
        .bind(team_id)
        .fetch_all(db)
        .await
    }

    pub async fn my_invitations(
        db: &PgPool,
        user_id: Uuid,
        email: &str,
    ) -> Result<Vec<MyInvitationRow>, sqlx::Error> {
        sqlx::query_as::<_, MyInvitationRow>(
            "SELECT
                i.id,
                i.team_id,
                tm.name AS team_name,
                i.tournament_id,
                t.title AS tournament_title,
                invited_by_user.id AS invited_by_id,
                invited_by_user.full_name AS invited_by_full_name,
                i.expires_at
            FROM team_invitations i
            JOIN teams tm ON tm.id = i.team_id
            JOIN tournaments t ON t.id = i.tournament_id
            JOIN users invited_by_user ON invited_by_user.id = i.invited_by
            WHERE i.status = 'pending'
                AND i.expires_at > NOW()
                AND t.status = 'registration'
                AND t.registration_ends_at > NOW()
                AND (i.invited_user_id = $1 OR lower(i.email) = lower($2))
            ORDER BY i.created_at DESC",
        )
        .bind(user_id)
        .bind(email)
        .fetch_all(db)
        .await
    }

    pub async fn my_teams(db: &PgPool, user_id: Uuid) -> Result<Vec<MyTeamRow>, sqlx::Error> {
        sqlx::query_as::<_, MyTeamRow>(
            "SELECT
                tm.id AS team_id,
                tm.name AS team_name,
                ms.role,
                ms.status,
                t.id AS tournament_id,
                t.title AS tournament_title,
                t.status AS tournament_status,
                (
                    SELECT COUNT(*)::bigint
                    FROM team_memberships accepted
                    WHERE accepted.team_id = tm.id AND accepted.status = 'accepted'
                ) AS members_count
            FROM team_memberships ms
            JOIN teams tm ON tm.id = ms.team_id
            JOIN tournaments t ON t.id = tm.tournament_id
            WHERE ms.user_id = $1
            ORDER BY t.starts_at DESC, tm.created_at DESC",
        )
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn update_team(
        db: &PgPool,
        team_id: Uuid,
        name: &str,
        organization: Option<&str>,
        contact: Option<&str>,
    ) -> Result<Team, sqlx::Error> {
        sqlx::query_as::<_, Team>(
            "UPDATE teams
            SET name = $2, organization = $3, contact = $4, updated_at = NOW()
            WHERE id = $1
            RETURNING id, tournament_id, name, organization, contact, created_by, created_at, updated_at",
        )
        .bind(team_id)
        .bind(name)
        .bind(organization)
        .bind(contact)
        .fetch_one(db)
        .await
    }

    pub async fn bind_invitation_to_user(
        tx: &mut Transaction<'_, Postgres>,
        invitation_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE team_invitations
            SET invited_user_id = COALESCE(invited_user_id, $2), updated_at = NOW()
            WHERE id = $1",
        )
        .bind(invitation_id)
        .bind(user_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn update_invitation_status(
        db: &PgPool,
        invitation_id: Uuid,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE team_invitations
            SET status = $2, updated_at = NOW()
            WHERE id = $1",
        )
        .bind(invitation_id)
        .bind(status)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn update_invitation_status_tx(
        tx: &mut Transaction<'_, Postgres>,
        invitation_id: Uuid,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE team_invitations
            SET status = $2, updated_at = NOW()
            WHERE id = $1",
        )
        .bind(invitation_id)
        .bind(status)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn mark_member_removed(
        db: &PgPool,
        team_id: Uuid,
        user_id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE team_memberships
            SET status = 'removed', updated_at = NOW()
            WHERE team_id = $1 AND user_id = $2 AND role <> 'captain' AND status = 'accepted'",
        )
        .bind(team_id)
        .bind(user_id)
        .execute(db)
        .await?;

        Ok(result.rows_affected())
    }
}
