use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult},
    rounds::model::RoundStatus,
    tournaments::{
        model::TournamentStatus, repository::TournamentRepository, service::TournamentService,
    },
    users::{auth::AuthenticatedUser, model::AccountRole, repository::UserRepository},
};

use super::{
    dto::{
        AddJuryRequest, AssignmentDetailResponse, AssignmentListResponse, AssignmentSubmission,
        AssignmentTeam, GenerateAssignmentsRequest, GenerateAssignmentsResponse, JuryListResponse,
        OrganizerJuryManagementResponse,
    },
    model::{AssignmentCandidate, PlannedAssignment},
    repository::JuryRepository,
};

pub struct JuryService;

impl JuryService {
    pub async fn organizer_jury_management(
        db: &PgPool,
        user: AuthenticatedUser,
    ) -> ApiResult<OrganizerJuryManagementResponse> {
        match UserRepository::account_role(db, user.user_id).await? {
            Some(AccountRole::Organiser) => {}
            _ => {
                return Err(ApiError::Forbidden(
                    "Only organisers can manage tournament jury".to_string(),
                ));
            }
        }

        let (tournaments, juries) = tokio::try_join!(
            JuryRepository::list_organizer_manageable_tournaments(db, user.user_id),
            JuryRepository::list_jury_candidates(db)
        )?;

        Ok(OrganizerJuryManagementResponse {
            tournaments,
            juries,
        })
    }

    pub async fn add_jury(
        db: &PgPool,
        user: AuthenticatedUser,
        tournament_id: Uuid,
        payload: AddJuryRequest,
    ) -> ApiResult<JuryListResponse> {
        TournamentService::require_tournament_organizer(db, tournament_id, user).await?;
        ensure_jury_management_allowed(db, tournament_id).await?;

        let added_by_email = payload.email.is_some();
        let target_user_id = match (payload.user_id, payload.email) {
            (Some(user_id), None) => {
                match UserRepository::account_role(db, user_id).await? {
                    Some(AccountRole::Jury) => {}
                    Some(_) => {
                        return Err(ApiError::Validation(
                            "Selected user must have jury role".to_string(),
                        ));
                    }
                    None => return Err(ApiError::NotFound("User not found".to_string())),
                }
                user_id
            }
            (None, Some(email)) => {
                let email = email.trim();
                if email.is_empty() {
                    return Err(ApiError::Validation("Jury email is required".to_string()));
                }

                JuryRepository::find_user_by_email(db, email)
                    .await?
                    .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?
            }
            _ => {
                return Err(ApiError::Validation(
                    "Provide either user_id or email".to_string(),
                ));
            }
        };

        let rows = JuryRepository::add_jury_role(db, tournament_id, target_user_id).await?;
        if rows == 0 {
            return Err(ApiError::Conflict(
                "This user is already jury for the tournament".to_string(),
            ));
        }
        if added_by_email {
            UserRepository::promote_to_jury_if_participant(db, target_user_id).await?;
        }

        Self::list_jury(db, user, tournament_id).await
    }

    pub async fn list_jury(
        db: &PgPool,
        user: AuthenticatedUser,
        tournament_id: Uuid,
    ) -> ApiResult<JuryListResponse> {
        TournamentService::require_tournament_organizer(db, tournament_id, user).await?;
        let items = JuryRepository::list_jury(db, tournament_id).await?;
        Ok(JuryListResponse { items })
    }

    pub async fn remove_jury(
        db: &PgPool,
        user: AuthenticatedUser,
        tournament_id: Uuid,
        target_user_id: Uuid,
    ) -> ApiResult<()> {
        TournamentService::require_tournament_organizer(db, tournament_id, user).await?;
        ensure_jury_management_allowed(db, tournament_id).await?;
        let rows = JuryRepository::remove_jury_role(db, tournament_id, target_user_id).await?;
        if rows == 0 {
            return Err(ApiError::NotFound(
                "Jury role not found for this tournament".to_string(),
            ));
        }

        Ok(())
    }

    pub async fn generate_assignments(
        db: &PgPool,
        user: AuthenticatedUser,
        round_id: Uuid,
        payload: GenerateAssignmentsRequest,
    ) -> ApiResult<GenerateAssignmentsResponse> {
        if payload.reviews_per_submission < 1 {
            return Err(ApiError::Validation(
                "reviews_per_submission must be at least 1".to_string(),
            ));
        }
        if payload.max_assignments_per_jury < 1 {
            return Err(ApiError::Validation(
                "max_assignments_per_jury must be at least 1".to_string(),
            ));
        }

        let round = JuryRepository::find_round(db, round_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Round not found".to_string()))?;
        TournamentService::require_tournament_organizer(db, round.tournament_id, user).await?;
        let tournament = TournamentRepository::find_by_id(db, round.tournament_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Tournament not found".to_string()))?;

        ensure_assignment_generation_allowed(&tournament.status, &round.status)?;

        let existing_assignments =
            JuryRepository::count_assignments_for_round(db, round.id).await?;
        if existing_assignments > 0 {
            return Err(ApiError::Conflict(
                "Assignments have already been generated for this round".to_string(),
            ));
        }

        let submissions = JuryRepository::submissions_for_round(db, round.id).await?;
        if submissions.is_empty() {
            return Err(ApiError::Validation(
                "Cannot generate assignments without submissions".to_string(),
            ));
        }

        let jury_ids = JuryRepository::jury_user_ids(db, round.tournament_id).await?;
        if jury_ids.is_empty() {
            return Err(ApiError::Validation(
                "Tournament has no jury members".to_string(),
            ));
        }

        let mut candidates = Vec::with_capacity(submissions.len());
        for submission in submissions {
            let excluded_user_ids =
                JuryRepository::accepted_team_member_ids(db, submission.team_id)
                    .await?
                    .into_iter()
                    .collect::<HashSet<_>>();
            candidates.push(AssignmentCandidate {
                submission_id: submission.id,
                excluded_user_ids,
            });
        }

        let assignments = plan_assignments(
            &candidates,
            &jury_ids,
            payload.reviews_per_submission as usize,
            payload.max_assignments_per_jury as usize,
        )
        .map_err(ApiError::Validation)?;

        let mut tx = db.begin().await?;
        let mut created_assignments = 0;
        for assignment in &assignments {
            let rows = JuryRepository::insert_assignment(
                &mut tx,
                round.id,
                assignment.submission_id,
                assignment.jury_user_id,
            )
            .await?;
            created_assignments += rows as usize;
        }
        tx.commit().await?;

        Ok(GenerateAssignmentsResponse {
            created_assignments,
        })
    }

    pub async fn my_assignments(
        db: &PgPool,
        user: AuthenticatedUser,
    ) -> ApiResult<AssignmentListResponse> {
        let items = JuryRepository::assignments_for_jury(db, user.user_id).await?;
        Ok(AssignmentListResponse { items })
    }

    pub async fn assignment_detail(
        db: &PgPool,
        user: AuthenticatedUser,
        assignment_id: Uuid,
    ) -> ApiResult<AssignmentDetailResponse> {
        let detail = JuryRepository::assignment_detail_for_jury(db, assignment_id, user.user_id)
            .await?
            .ok_or_else(|| {
                ApiError::NotFound("Assignment not found or access denied".to_string())
            })?;

        let criteria =
            JuryRepository::criteria_for_assignment(db, detail.id, detail.tournament_id).await?;
        let comment = JuryRepository::assignment_comment(db, detail.id).await?;

        Ok(AssignmentDetailResponse {
            id: detail.id,
            status: detail.status,
            team: AssignmentTeam {
                id: detail.team_id,
                name: detail.team_name,
            },
            submission: AssignmentSubmission {
                github_url: detail.github_url,
                video_demo_url: detail.video_demo_url,
                live_demo_url: detail.live_demo_url,
                description: detail.description,
            },
            criteria,
            comment,
        })
    }
}

pub(crate) fn plan_assignments(
    submissions: &[AssignmentCandidate],
    jury_ids: &[Uuid],
    reviews_per_submission: usize,
    max_assignments_per_jury: usize,
) -> Result<Vec<PlannedAssignment>, String> {
    if reviews_per_submission == 0 || max_assignments_per_jury == 0 {
        return Err("Assignment limits must be greater than zero".to_string());
    }

    let mut load = jury_ids
        .iter()
        .copied()
        .map(|jury_id| (jury_id, 0usize))
        .collect::<HashMap<_, _>>();
    let mut planned = Vec::with_capacity(submissions.len() * reviews_per_submission);

    for submission in submissions {
        let mut candidates = jury_ids
            .iter()
            .copied()
            .filter(|jury_id| !submission.excluded_user_ids.contains(jury_id))
            .filter(|jury_id| load.get(jury_id).copied().unwrap_or(0) < max_assignments_per_jury)
            .collect::<Vec<_>>();

        candidates.sort_by_key(|jury_id| (load.get(jury_id).copied().unwrap_or(0), *jury_id));

        if candidates.len() < reviews_per_submission {
            return Err(
                "Not enough eligible jury members to generate fair assignments".to_string(),
            );
        }

        for jury_user_id in candidates.into_iter().take(reviews_per_submission) {
            planned.push(PlannedAssignment {
                submission_id: submission.submission_id,
                jury_user_id,
            });
            *load.entry(jury_user_id).or_insert(0) += 1;
        }
    }

    Ok(planned)
}

async fn ensure_jury_management_allowed(db: &PgPool, tournament_id: Uuid) -> ApiResult<()> {
    let tournament = TournamentRepository::find_by_id(db, tournament_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Tournament not found".to_string()))?;

    validate_jury_management_status(&tournament.status)
}

fn validate_jury_management_status(status: &str) -> ApiResult<()> {
    let status = TournamentStatus::from_str(status)
        .map_err(|_| ApiError::Validation("Tournament has invalid status".to_string()))?;

    if status == TournamentStatus::Finished {
        return Err(ApiError::Validation(
            "Jury members cannot be changed after tournament is finished".to_string(),
        ));
    }

    Ok(())
}

fn ensure_assignment_generation_allowed(
    tournament_status: &str,
    round_status: &str,
) -> ApiResult<()> {
    let tournament_status = TournamentStatus::from_str(tournament_status)
        .map_err(|_| ApiError::Validation("Tournament has invalid status".to_string()))?;
    let round_status = RoundStatus::from_str(round_status)
        .map_err(|_| ApiError::Validation("Round has invalid status".to_string()))?;

    if tournament_status != TournamentStatus::Running {
        return Err(ApiError::Validation(
            "Assignments can be generated only while tournament is running".to_string(),
        ));
    }

    if round_status != RoundStatus::SubmissionClosed {
        return Err(ApiError::Validation(
            "Assignments can be generated only after submissions are closed".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(submission_id: Uuid, excluded_user_ids: &[Uuid]) -> AssignmentCandidate {
        AssignmentCandidate {
            submission_id,
            excluded_user_ids: excluded_user_ids.iter().copied().collect(),
        }
    }

    #[test]
    fn assignment_algorithm_distributes_evenly() {
        let jury = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
        let submissions = vec![
            candidate(Uuid::new_v4(), &[]),
            candidate(Uuid::new_v4(), &[]),
            candidate(Uuid::new_v4(), &[]),
        ];

        let planned = plan_assignments(&submissions, &jury, 2, 3).unwrap();
        let mut loads = HashMap::<Uuid, usize>::new();
        for assignment in planned {
            *loads.entry(assignment.jury_user_id).or_default() += 1;
        }

        let min = loads.values().min().copied().unwrap();
        let max = loads.values().max().copied().unwrap();
        assert!(max - min <= 1);
    }

    #[test]
    fn assignment_algorithm_does_not_duplicate_jury_per_submission() {
        let jury = vec![Uuid::new_v4(), Uuid::new_v4()];
        let submission_id = Uuid::new_v4();
        let planned = plan_assignments(&[candidate(submission_id, &[])], &jury, 2, 2).unwrap();
        let unique = planned
            .iter()
            .map(|assignment| assignment.jury_user_id)
            .collect::<HashSet<_>>();

        assert_eq!(unique.len(), 2);
    }

    #[test]
    fn assignment_algorithm_excludes_own_team_members() {
        let own_team_jury = Uuid::new_v4();
        let eligible_jury = Uuid::new_v4();
        let jury = vec![own_team_jury, eligible_jury];

        let planned =
            plan_assignments(&[candidate(Uuid::new_v4(), &[own_team_jury])], &jury, 1, 2).unwrap();

        assert_eq!(planned[0].jury_user_id, eligible_jury);
    }

    #[test]
    fn assignment_algorithm_rejects_insufficient_jury() {
        let jury = vec![Uuid::new_v4()];
        let result = plan_assignments(&[candidate(Uuid::new_v4(), &jury)], &jury, 1, 2);

        assert!(result.is_err());
    }

    #[test]
    fn jury_management_is_allowed_until_tournament_is_finished() {
        assert!(validate_jury_management_status("draft").is_ok());
        assert!(validate_jury_management_status("registration").is_ok());
        assert!(validate_jury_management_status("running").is_ok());
        assert!(validate_jury_management_status("finished").is_err());
    }

    #[test]
    fn assignments_require_running_tournament_and_closed_round() {
        assert!(ensure_assignment_generation_allowed("running", "submission_closed").is_ok());
        assert!(ensure_assignment_generation_allowed("registration", "submission_closed").is_err());
        assert!(ensure_assignment_generation_allowed("running", "active").is_err());
    }
}
