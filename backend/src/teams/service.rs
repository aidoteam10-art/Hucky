use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult},
    tournaments::model::{Tournament, TournamentStatus},
    users::{auth::AuthenticatedUser, model::AccountRole, repository::UserRepository},
};

use super::{
    dto::{
        CreateInvitationRequest, CreateTeamRequest, InvitedByResponse, MyInvitationItem,
        MyInvitationsResponse, MyTeamItem, MyTeamTournament, MyTeamsResponse, TeamCaptainPreview,
        TeamDetailResponse, TeamInvitationResponse, TeamMemberResponse, TeamSummaryResponse,
        TournamentTeamItem, TournamentTeamsResponse, UpdateTeamRequest,
    },
    model::{InvitationStatus, MembershipStatus, NewInvitation, NewTeam, Team, TeamRole},
    repository::TeamRepository,
};

const MAX_TEAM_FIELD_LENGTH: usize = 120;
const MAX_INVITED_MEMBERS: usize = 5;
const MAX_TOTAL_TEAM_MEMBERS: i64 = 6;
const INVITATION_TTL_DAYS: i64 = 7;

pub struct TeamService;

impl TeamService {
    pub async fn create_team(
        db: &PgPool,
        user: AuthenticatedUser,
        tournament_id: Uuid,
        payload: CreateTeamRequest,
    ) -> ApiResult<TeamSummaryResponse> {
        ensure_user_can_participate(db, user.user_id).await?;
        let current_user = TeamRepository::find_user_by_id(db, user.user_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Current user not found".to_string()))?;
        let tournament = find_tournament(db, tournament_id).await?;

        ensure_team_registration_is_open(&tournament)?;
        validate_team_fields(&payload.name, &payload.organization, &payload.contact)?;
        let member_emails = normalize_member_emails(&payload.member_emails)?;

        if member_emails.is_empty() {
            return Err(ApiError::Validation(
                "At least one member email is required".to_string(),
            ));
        }
        if member_emails.len() > MAX_INVITED_MEMBERS {
            return Err(ApiError::Validation(
                "Team can invite at most 5 members".to_string(),
            ));
        }

        if member_emails
            .iter()
            .any(|email| email.eq_ignore_ascii_case(&current_user.email))
        {
            return Err(ApiError::Validation(
                "Captain email cannot be included in member invitations".to_string(),
            ));
        }

        if TeamRepository::accepted_membership_in_tournament(db, user.user_id, tournament_id)
            .await?
            .is_some()
        {
            return Err(ApiError::Conflict(
                "User already belongs to an accepted team in this tournament".to_string(),
            ));
        }

        if let Some(max_teams) = tournament.max_teams {
            let existing_teams =
                TeamRepository::count_teams_in_tournament(db, tournament_id).await?;
            if existing_teams >= i64::from(max_teams) {
                return Err(ApiError::Conflict(
                    "Tournament has reached the maximum number of teams".to_string(),
                ));
            }
        }

        let mut invited_users = Vec::with_capacity(member_emails.len());
        for email in &member_emails {
            invited_users.push(TeamRepository::find_user_by_email(db, email).await?);
        }

        for invited_user in invited_users.iter().flatten() {
            ensure_user_can_participate(db, invited_user.id).await?;

            if TeamRepository::accepted_membership_in_tournament(db, invited_user.id, tournament_id)
                .await?
                .is_some()
            {
                return Err(ApiError::Conflict(format!(
                    "{} already belongs to another accepted team in this tournament",
                    invited_user.email
                )));
            }
        }

        let mut tx = db.begin().await?;
        let team = TeamRepository::insert_team(
            &mut tx,
            NewTeam {
                tournament_id,
                created_by: user.user_id,
                name: payload.name.trim(),
                organization: trimmed_optional(payload.organization.as_deref()),
                contact: trimmed_optional(payload.contact.as_deref()),
            },
        )
        .await?;

        TeamRepository::insert_membership(
            &mut tx,
            team.id,
            tournament_id,
            user.user_id,
            TeamRole::Captain,
            MembershipStatus::Accepted,
        )
        .await?;

        for (email, invited_user) in member_emails.iter().zip(invited_users.iter()) {
            let token = Uuid::new_v4().to_string();
            let expires_at = Utc::now() + Duration::days(INVITATION_TTL_DAYS);
            TeamRepository::insert_invitation(
                &mut tx,
                NewInvitation {
                    team_id: team.id,
                    tournament_id,
                    email,
                    invited_user_id: invited_user.as_ref().map(|user| user.id),
                    invited_by: user.user_id,
                    token: &token,
                    expires_at,
                },
            )
            .await?;
        }

        tx.commit().await?;

        Ok(team_summary(
            team,
            1,
            Some(TeamRole::Captain.as_str().to_string()),
        ))
    }

    pub async fn list_tournament_teams(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> ApiResult<TournamentTeamsResponse> {
        find_tournament(db, tournament_id).await?;
        let items = TeamRepository::tournament_teams(db, tournament_id)
            .await?
            .into_iter()
            .map(|row| TournamentTeamItem {
                id: row.id,
                name: row.name,
                organization: row.organization,
                members_count: row.members_count,
                captain: row
                    .captain_id
                    .zip(row.captain_full_name)
                    .map(|(id, full_name)| TeamCaptainPreview { id, full_name }),
            })
            .collect();

        Ok(TournamentTeamsResponse { items })
    }

    pub async fn get_team(
        db: &PgPool,
        user: AuthenticatedUser,
        team_id: Uuid,
    ) -> ApiResult<TeamDetailResponse> {
        let team = find_team(db, team_id).await?;
        ensure_can_view_team(db, user.user_id, &team).await?;
        build_team_detail(db, team).await
    }

    pub async fn update_team(
        db: &PgPool,
        user: AuthenticatedUser,
        team_id: Uuid,
        payload: UpdateTeamRequest,
    ) -> ApiResult<TeamSummaryResponse> {
        let team = find_team(db, team_id).await?;
        let tournament = find_tournament(db, team.tournament_id).await?;
        let is_captain = TeamRepository::is_team_captain(db, user.user_id, team_id).await?;
        let is_organizer =
            TeamRepository::is_tournament_organizer(db, team.tournament_id, user.user_id).await?;

        if !is_captain && !is_organizer {
            return Err(ApiError::Forbidden(
                "Only team captain or tournament organizer can update team".to_string(),
            ));
        }

        if is_captain {
            ensure_team_registration_is_open(&tournament)?;
        } else {
            ensure_tournament_not_started(&tournament)?;
        }

        validate_team_fields(&payload.name, &payload.organization, &payload.contact)?;

        let updated = TeamRepository::update_team(
            db,
            team_id,
            payload.name.trim(),
            trimmed_optional(payload.organization.as_deref()),
            trimmed_optional(payload.contact.as_deref()),
        )
        .await?;
        let members_count = TeamRepository::count_accepted_members(db, team_id).await?;

        Ok(team_summary(
            updated,
            members_count,
            if is_captain {
                Some(TeamRole::Captain.as_str().to_string())
            } else {
                None
            },
        ))
    }

    pub async fn create_invitation(
        db: &PgPool,
        user: AuthenticatedUser,
        team_id: Uuid,
        payload: CreateInvitationRequest,
    ) -> ApiResult<TeamInvitationResponse> {
        let team = find_team(db, team_id).await?;
        let tournament = find_tournament(db, team.tournament_id).await?;
        ensure_team_registration_is_open(&tournament)?;
        ensure_can_manage_team(db, user.user_id, &team).await?;

        let emails = normalize_member_emails(&[payload.email])?;
        let email = emails
            .first()
            .ok_or_else(|| ApiError::Validation("Email is required".to_string()))?;

        let invited_user = TeamRepository::find_user_by_email(db, email).await?;
        let accepted_members_count = TeamRepository::count_accepted_members(db, team_id).await?;
        let pending_invitations_count = TeamRepository::pending_invitations(db, team_id)
            .await?
            .len() as i64;

        if let Some(invited_user) = &invited_user {
            ensure_user_can_participate(db, invited_user.id).await?;
        }

        if accepted_members_count + pending_invitations_count >= MAX_TOTAL_TEAM_MEMBERS {
            return Err(ApiError::Validation(
                "Team already has the maximum number of members and pending invitations"
                    .to_string(),
            ));
        }

        if let Some(invited_user) = &invited_user {
            if TeamRepository::is_team_member(db, invited_user.id, team_id).await? {
                return Err(ApiError::Conflict(
                    "User is already an accepted member of this team".to_string(),
                ));
            }

            if TeamRepository::accepted_membership_in_tournament(
                db,
                invited_user.id,
                team.tournament_id,
            )
            .await?
            .is_some()
            {
                return Err(ApiError::Conflict(
                    "User already belongs to another accepted team in this tournament".to_string(),
                ));
            }
        }

        let mut tx = db.begin().await?;
        let token = Uuid::new_v4().to_string();
        let invitation = TeamRepository::insert_invitation(
            &mut tx,
            NewInvitation {
                team_id,
                tournament_id: team.tournament_id,
                email,
                invited_user_id: invited_user.map(|user| user.id),
                invited_by: user.user_id,
                token: &token,
                expires_at: Utc::now() + Duration::days(INVITATION_TTL_DAYS),
            },
        )
        .await?;
        tx.commit().await?;

        Ok(invitation_response(invitation))
    }

    pub async fn my_invitations(
        db: &PgPool,
        user: AuthenticatedUser,
    ) -> ApiResult<MyInvitationsResponse> {
        let current_user = TeamRepository::find_user_by_id(db, user.user_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Current user not found".to_string()))?;

        let items = TeamRepository::my_invitations(db, user.user_id, &current_user.email)
            .await?
            .into_iter()
            .map(|row| MyInvitationItem {
                id: row.id,
                team_id: row.team_id,
                team_name: row.team_name,
                tournament_id: row.tournament_id,
                tournament_title: row.tournament_title,
                invited_by: InvitedByResponse {
                    id: row.invited_by_id,
                    full_name: row.invited_by_full_name,
                },
                expires_at: row.expires_at,
            })
            .collect();

        Ok(MyInvitationsResponse { items })
    }

    pub async fn accept_invitation(
        db: &PgPool,
        user: AuthenticatedUser,
        invitation_id: Uuid,
    ) -> ApiResult<TeamSummaryResponse> {
        ensure_user_can_participate(db, user.user_id).await?;
        let current_user = TeamRepository::find_user_by_id(db, user.user_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Current user not found".to_string()))?;
        let invitation = find_invitation(db, invitation_id).await?;

        ensure_invitation_belongs_to_user(&invitation, user.user_id, &current_user.email)?;
        let tournament = find_tournament(db, invitation.tournament_id).await?;
        ensure_invitation_can_be_accepted(&invitation, &tournament)?;

        if TeamRepository::accepted_membership_in_tournament(
            db,
            user.user_id,
            invitation.tournament_id,
        )
        .await?
        .is_some()
        {
            return Err(ApiError::Conflict(
                "User already belongs to an accepted team in this tournament".to_string(),
            ));
        }

        let mut tx = db.begin().await?;
        TeamRepository::bind_invitation_to_user(&mut tx, invitation.id, user.user_id).await?;
        TeamRepository::insert_membership(
            &mut tx,
            invitation.team_id,
            invitation.tournament_id,
            user.user_id,
            TeamRole::Member,
            MembershipStatus::Accepted,
        )
        .await?;
        TeamRepository::update_invitation_status_tx(
            &mut tx,
            invitation.id,
            InvitationStatus::Accepted.as_str(),
        )
        .await?;
        tx.commit().await?;

        let team = find_team(db, invitation.team_id).await?;
        let members_count = TeamRepository::count_accepted_members(db, team.id).await?;
        Ok(team_summary(
            team,
            members_count,
            Some(TeamRole::Member.as_str().to_string()),
        ))
    }

    pub async fn decline_invitation(
        db: &PgPool,
        user: AuthenticatedUser,
        invitation_id: Uuid,
    ) -> ApiResult<()> {
        let current_user = TeamRepository::find_user_by_id(db, user.user_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Current user not found".to_string()))?;
        let invitation = find_invitation(db, invitation_id).await?;

        ensure_invitation_belongs_to_user(&invitation, user.user_id, &current_user.email)?;
        ensure_pending_and_unexpired(&invitation)?;

        TeamRepository::update_invitation_status(
            db,
            invitation.id,
            InvitationStatus::Declined.as_str(),
        )
        .await?;
        Ok(())
    }

    pub async fn delete_invitation(
        db: &PgPool,
        user: AuthenticatedUser,
        invitation_id: Uuid,
    ) -> ApiResult<()> {
        let invitation = find_invitation(db, invitation_id).await?;
        let team = find_team(db, invitation.team_id).await?;
        ensure_can_manage_team(db, user.user_id, &team).await?;

        if invitation.status != InvitationStatus::Pending.as_str() {
            return Err(ApiError::Validation(
                "Only pending invitations can be cancelled".to_string(),
            ));
        }

        TeamRepository::update_invitation_status(
            db,
            invitation.id,
            InvitationStatus::Cancelled.as_str(),
        )
        .await?;
        Ok(())
    }

    pub async fn remove_member(
        db: &PgPool,
        user: AuthenticatedUser,
        team_id: Uuid,
        target_user_id: Uuid,
    ) -> ApiResult<()> {
        let team = find_team(db, team_id).await?;
        let tournament = find_tournament(db, team.tournament_id).await?;
        ensure_tournament_not_started(&tournament)?;
        ensure_can_manage_team(db, user.user_id, &team).await?;

        if user.user_id == target_user_id {
            return Err(ApiError::Validation(
                "Captain cannot remove themselves without captain transfer".to_string(),
            ));
        }

        let rows = TeamRepository::mark_member_removed(db, team_id, target_user_id).await?;
        if rows == 0 {
            return Err(ApiError::NotFound("Team member not found".to_string()));
        }

        Ok(())
    }

    pub async fn my_teams(db: &PgPool, user: AuthenticatedUser) -> ApiResult<MyTeamsResponse> {
        let items = TeamRepository::my_teams(db, user.user_id)
            .await?
            .into_iter()
            .map(|row| MyTeamItem {
                team_id: row.team_id,
                team_name: row.team_name,
                role: row.role,
                status: row.status,
                tournament: MyTeamTournament {
                    id: row.tournament_id,
                    title: row.tournament_title,
                    status: row.tournament_status,
                },
                members_count: row.members_count,
            })
            .collect();

        Ok(MyTeamsResponse { items })
    }

    #[allow(dead_code)]
    pub async fn ensure_team_is_ready_for_tournament(db: &PgPool, team_id: Uuid) -> ApiResult<()> {
        let count = TeamRepository::count_accepted_members(db, team_id).await?;
        if count < 2 {
            return Err(ApiError::Validation(
                "Team must have at least 2 accepted members".to_string(),
            ));
        }

        Ok(())
    }
}

async fn find_tournament(db: &PgPool, tournament_id: Uuid) -> ApiResult<Tournament> {
    TeamRepository::find_tournament(db, tournament_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Tournament not found".to_string()))
}

async fn find_team(db: &PgPool, team_id: Uuid) -> ApiResult<Team> {
    TeamRepository::find_team(db, team_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Team not found".to_string()))
}

async fn find_invitation(
    db: &PgPool,
    invitation_id: Uuid,
) -> ApiResult<super::model::TeamInvitation> {
    TeamRepository::find_invitation(db, invitation_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Invitation not found".to_string()))
}

async fn ensure_can_view_team(db: &PgPool, user_id: Uuid, team: &Team) -> ApiResult<()> {
    let is_member = TeamRepository::is_team_member(db, user_id, team.id).await?;
    let is_organizer =
        TeamRepository::is_tournament_organizer(db, team.tournament_id, user_id).await?;

    if is_member || is_organizer {
        Ok(())
    } else {
        Err(ApiError::Forbidden(
            "User cannot view this team's private details".to_string(),
        ))
    }
}

async fn ensure_can_manage_team(db: &PgPool, user_id: Uuid, team: &Team) -> ApiResult<()> {
    let is_captain = TeamRepository::is_team_captain(db, user_id, team.id).await?;
    let is_organizer =
        TeamRepository::is_tournament_organizer(db, team.tournament_id, user_id).await?;

    if is_captain || is_organizer {
        Ok(())
    } else {
        Err(ApiError::Forbidden(
            "Only team captain or tournament organizer can perform this action".to_string(),
        ))
    }
}

async fn ensure_user_can_participate(db: &PgPool, user_id: Uuid) -> ApiResult<()> {
    let role = UserRepository::account_role(db, user_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    if role == AccountRole::Organiser {
        return Err(ApiError::Forbidden(
            "Organisers cannot register or join tournament teams".to_string(),
        ));
    }

    Ok(())
}

fn ensure_team_registration_is_open(tournament: &Tournament) -> ApiResult<()> {
    let status = tournament
        .status
        .parse::<TournamentStatus>()
        .map_err(|_| ApiError::Validation("Tournament has invalid status".to_string()))?;
    if status != TournamentStatus::Registration {
        return Err(ApiError::Validation(
            "Team registration is available only while tournament registration is open".to_string(),
        ));
    }

    if Utc::now() >= tournament.registration_ends_at {
        return Err(ApiError::Validation(
            "Team registration is closed for this tournament".to_string(),
        ));
    }

    Ok(())
}

fn ensure_tournament_not_started(tournament: &Tournament) -> ApiResult<()> {
    let status = tournament
        .status
        .parse::<TournamentStatus>()
        .map_err(|_| ApiError::Validation("Tournament has invalid status".to_string()))?;

    if matches!(
        status,
        TournamentStatus::Running | TournamentStatus::Finished
    ) || Utc::now() >= tournament.starts_at
    {
        return Err(ApiError::Validation(
            "Team changes are locked after tournament start".to_string(),
        ));
    }

    Ok(())
}

fn ensure_invitation_belongs_to_user(
    invitation: &super::model::TeamInvitation,
    user_id: Uuid,
    user_email: &str,
) -> ApiResult<()> {
    let matches_user_id = invitation.invited_user_id == Some(user_id);
    let matches_email = invitation.email.eq_ignore_ascii_case(user_email);

    if matches_user_id || matches_email {
        Ok(())
    } else {
        Err(ApiError::Forbidden(
            "Invitation does not belong to current user".to_string(),
        ))
    }
}

fn ensure_pending_and_unexpired(invitation: &super::model::TeamInvitation) -> ApiResult<()> {
    if invitation.status != InvitationStatus::Pending.as_str() {
        return Err(ApiError::Validation(
            "Invitation is not pending".to_string(),
        ));
    }

    if invitation.expires_at < Utc::now() {
        return Err(ApiError::Validation("Invitation has expired".to_string()));
    }

    Ok(())
}

fn ensure_invitation_can_be_accepted(
    invitation: &super::model::TeamInvitation,
    tournament: &Tournament,
) -> ApiResult<()> {
    ensure_pending_and_unexpired(invitation)?;
    ensure_team_registration_is_open(tournament).map_err(|_| {
        ApiError::Validation(
            "Invitation can be accepted only while tournament registration is open".to_string(),
        )
    })
}

fn build_team_detail(
    db: &PgPool,
    team: Team,
) -> impl std::future::Future<Output = ApiResult<TeamDetailResponse>> + '_ {
    async move {
        let members = TeamRepository::team_members(db, team.id)
            .await?
            .into_iter()
            .map(|member| TeamMemberResponse {
                user_id: member.user_id,
                full_name: member.full_name,
                email: member.email,
                role: member.role,
                status: member.status,
            })
            .collect();

        let pending_invitations = TeamRepository::pending_invitations(db, team.id)
            .await?
            .into_iter()
            .map(invitation_response)
            .collect();

        Ok(TeamDetailResponse {
            id: team.id,
            tournament_id: team.tournament_id,
            name: team.name,
            organization: team.organization,
            contact: team.contact,
            members,
            pending_invitations,
        })
    }
}

fn team_summary(
    team: Team,
    members_count: i64,
    current_user_role: Option<String>,
) -> TeamSummaryResponse {
    TeamSummaryResponse {
        id: team.id,
        tournament_id: team.tournament_id,
        name: team.name,
        organization: team.organization,
        contact: team.contact,
        members_count,
        current_user_role,
    }
}

fn invitation_response(invitation: super::model::TeamInvitation) -> TeamInvitationResponse {
    TeamInvitationResponse {
        id: invitation.id,
        team_id: invitation.team_id,
        tournament_id: invitation.tournament_id,
        email: invitation.email,
        status: invitation.status,
        expires_at: invitation.expires_at,
    }
}

pub(crate) fn validate_team_fields(
    name: &str,
    organization: &Option<String>,
    contact: &Option<String>,
) -> ApiResult<()> {
    if name.trim().chars().count() < 2 {
        return Err(ApiError::Validation(
            "Team name must contain at least 2 characters".to_string(),
        ));
    }

    if let Some(value) = organization {
        if value.trim().chars().count() > MAX_TEAM_FIELD_LENGTH {
            return Err(ApiError::Validation(
                "Organization must be at most 120 characters".to_string(),
            ));
        }
    }

    if let Some(value) = contact {
        if value.trim().chars().count() > MAX_TEAM_FIELD_LENGTH {
            return Err(ApiError::Validation(
                "Contact must be at most 120 characters".to_string(),
            ));
        }
    }

    Ok(())
}

pub(crate) fn normalize_member_emails(emails: &[String]) -> ApiResult<Vec<String>> {
    let mut normalized = Vec::new();

    for email in emails {
        let email = email.trim().to_lowercase();
        if email.is_empty() {
            continue;
        }

        if !is_valid_email(&email) {
            return Err(ApiError::Validation(format!("Invalid email: {}", email)));
        }

        if normalized.iter().any(|existing| existing == &email) {
            return Err(ApiError::Validation(
                "Member invitation emails must not contain duplicates".to_string(),
            ));
        }

        normalized.push(email);
    }

    Ok(normalized)
}

fn trimmed_optional(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

fn is_valid_email(value: &str) -> bool {
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };

    !local.is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && !value.chars().any(char::is_whitespace)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_email_case_and_spaces() {
        let emails = vec![
            " User@Example.COM ".to_string(),
            "other@test.dev".to_string(),
        ];
        let normalized = normalize_member_emails(&emails).unwrap();

        assert_eq!(normalized, vec!["user@example.com", "other@test.dev"]);
    }

    #[test]
    fn rejects_duplicate_emails_after_normalization() {
        let emails = vec![
            "user@example.com".to_string(),
            " USER@example.com ".to_string(),
        ];
        let result = normalize_member_emails(&emails);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_short_team_name() {
        let result = validate_team_fields(&"A".to_string(), &None, &None);

        assert!(result.is_err());
    }

    #[test]
    fn allows_team_registration_when_status_is_registration_before_date_window() {
        let tournament = tournament_with_status(
            TournamentStatus::Registration,
            Utc::now() + Duration::days(1),
            Utc::now() + Duration::days(2),
        );

        assert!(ensure_team_registration_is_open(&tournament).is_ok());
    }

    #[test]
    fn rejects_team_registration_after_registration_end() {
        let tournament = tournament_with_status(
            TournamentStatus::Registration,
            Utc::now() - Duration::days(2),
            Utc::now() - Duration::days(1),
        );

        assert!(ensure_team_registration_is_open(&tournament).is_err());
    }

    #[test]
    fn rejects_team_registration_when_status_is_not_registration() {
        let tournament = tournament_with_status(
            TournamentStatus::Draft,
            Utc::now() - Duration::days(1),
            Utc::now() + Duration::days(1),
        );

        assert!(ensure_team_registration_is_open(&tournament).is_err());
    }

    #[test]
    fn rejects_expired_invitation() {
        let invitation = super::super::model::TeamInvitation {
            id: Uuid::new_v4(),
            team_id: Uuid::new_v4(),
            tournament_id: Uuid::new_v4(),
            email: "user@example.com".to_string(),
            invited_user_id: None,
            invited_by: Uuid::new_v4(),
            status: InvitationStatus::Pending.as_str().to_string(),
            expires_at: Utc::now() - Duration::minutes(1),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(ensure_pending_and_unexpired(&invitation).is_err());
    }

    #[test]
    fn rejects_invitation_when_registration_ended() {
        let invitation = pending_invitation();
        let tournament = tournament_with_status(
            TournamentStatus::Registration,
            Utc::now() - Duration::days(2),
            Utc::now() - Duration::minutes(1),
        );

        assert!(ensure_invitation_can_be_accepted(&invitation, &tournament).is_err());
    }

    #[test]
    fn rejects_invitation_when_tournament_is_running() {
        let invitation = pending_invitation();
        let tournament = tournament_with_status(
            TournamentStatus::Running,
            Utc::now() - Duration::days(2),
            Utc::now() + Duration::days(1),
        );

        assert!(ensure_invitation_can_be_accepted(&invitation, &tournament).is_err());
    }

    #[test]
    fn accepts_invitation_while_registration_is_open() {
        let invitation = pending_invitation();
        let tournament = tournament_with_status(
            TournamentStatus::Registration,
            Utc::now() - Duration::days(1),
            Utc::now() + Duration::days(1),
        );

        assert!(ensure_invitation_can_be_accepted(&invitation, &tournament).is_ok());
    }

    fn pending_invitation() -> super::super::model::TeamInvitation {
        super::super::model::TeamInvitation {
            id: Uuid::new_v4(),
            team_id: Uuid::new_v4(),
            tournament_id: Uuid::new_v4(),
            email: "user@example.com".to_string(),
            invited_user_id: None,
            invited_by: Uuid::new_v4(),
            status: InvitationStatus::Pending.as_str().to_string(),
            expires_at: Utc::now() + Duration::days(1),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn tournament_with_status(
        status: TournamentStatus,
        registration_starts_at: chrono::DateTime<Utc>,
        registration_ends_at: chrono::DateTime<Utc>,
    ) -> Tournament {
        Tournament {
            id: Uuid::new_v4(),
            organizer_id: Uuid::new_v4(),
            title: "Test tournament".to_string(),
            description: "Description".to_string(),
            rules: "Rules".to_string(),
            status: status.as_str().to_string(),
            registration_starts_at,
            registration_ends_at,
            starts_at: Utc::now() + Duration::days(3),
            ends_at: None,
            max_teams: None,
        }
    }
}
