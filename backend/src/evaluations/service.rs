use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult},
    rounds::model::RoundStatus,
    tournaments::model::TournamentStatus,
    users::auth::AuthenticatedUser,
};

use super::{
    dto::{ScoreRequest, SubmitEvaluationRequest, SubmitEvaluationResponse},
    model::EvaluationCriterion,
    repository::EvaluationRepository,
};

pub struct EvaluationService;

impl EvaluationService {
    pub async fn submit(
        db: &PgPool,
        user: AuthenticatedUser,
        assignment_id: Uuid,
        payload: SubmitEvaluationRequest,
    ) -> ApiResult<SubmitEvaluationResponse> {
        let assignment = EvaluationRepository::find_assignment(db, assignment_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Assignment not found".to_string()))?;

        if assignment.jury_user_id != user.user_id {
            return Err(ApiError::Forbidden(
                "Only assigned jury can submit this evaluation".to_string(),
            ));
        }

        if assignment.status == "completed" {
            return Err(ApiError::Validation(
                "Assignment has already been evaluated".to_string(),
            ));
        }

        if assignment.status != "pending" {
            return Err(ApiError::Validation(
                "Only pending assignments can be evaluated".to_string(),
            ));
        }
        ensure_evaluation_window(&assignment.round_status, &assignment.tournament_status)?;

        let criteria =
            EvaluationRepository::criteria_for_tournament(db, assignment.tournament_id).await?;
        validate_scores(&payload.scores, &criteria)?;

        let comment = payload
            .comment
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        if matches!(&comment, Some(value) if value.chars().count() > 2000) {
            return Err(ApiError::Validation(
                "Evaluation comment cannot exceed 2000 characters".to_string(),
            ));
        }

        let mut tx = db.begin().await?;
        for score in payload.scores {
            EvaluationRepository::insert_score(
                &mut tx,
                assignment.id,
                score.criterion_id,
                score.score,
            )
            .await?;
        }

        if let Some(comment) = comment {
            EvaluationRepository::insert_comment(&mut tx, assignment.id, &comment).await?;
        }

        EvaluationRepository::mark_assignment_completed(&mut tx, assignment.id).await?;
        let pending =
            EvaluationRepository::pending_assignments_count(&mut tx, assignment.round_id).await?;
        if pending == 0 {
            EvaluationRepository::mark_round_evaluated(&mut tx, assignment.round_id).await?;
        }

        tx.commit().await?;

        Ok(SubmitEvaluationResponse {
            status: "completed".to_string(),
        })
    }
}

pub(crate) fn validate_scores(
    scores: &[ScoreRequest],
    criteria: &[EvaluationCriterion],
) -> ApiResult<()> {
    if criteria.is_empty() {
        return Err(ApiError::Validation(
            "Tournament has no evaluation criteria".to_string(),
        ));
    }

    if scores.len() != criteria.len() {
        return Err(ApiError::Validation(
            "Every criterion must have exactly one score".to_string(),
        ));
    }

    let criteria_by_id = criteria
        .iter()
        .map(|criterion| (criterion.id, criterion.max_score))
        .collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();

    for score in scores {
        if !seen.insert(score.criterion_id) {
            return Err(ApiError::Validation(
                "Duplicate criterion scores are not allowed".to_string(),
            ));
        }

        let Some(max_score) = criteria_by_id.get(&score.criterion_id) else {
            return Err(ApiError::Validation(
                "Score references criterion outside this tournament".to_string(),
            ));
        };

        if score.score < 0 || score.score > *max_score {
            return Err(ApiError::Validation(format!(
                "Score must be between 0 and {max_score}"
            )));
        }
    }

    Ok(())
}

fn ensure_evaluation_window(round_status: &str, tournament_status: &str) -> ApiResult<()> {
    let round_status = RoundStatus::from_str(round_status)
        .map_err(|_| ApiError::Validation("Round has invalid status".to_string()))?;
    let tournament_status = TournamentStatus::from_str(tournament_status)
        .map_err(|_| ApiError::Validation("Tournament has invalid status".to_string()))?;

    if round_status != RoundStatus::SubmissionClosed {
        return Err(ApiError::Validation(
            "Evaluations can be submitted only after submissions are closed".to_string(),
        ));
    }

    if tournament_status != TournamentStatus::Running {
        return Err(ApiError::Validation(
            "Evaluations can be submitted only while tournament is running".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_complete_scores() {
        let c1 = EvaluationCriterion {
            id: Uuid::new_v4(),
            max_score: 100,
        };
        let c2 = EvaluationCriterion {
            id: Uuid::new_v4(),
            max_score: 50,
        };

        let scores = vec![
            ScoreRequest {
                criterion_id: c1.id,
                score: 88,
            },
            ScoreRequest {
                criterion_id: c2.id,
                score: 50,
            },
        ];

        assert!(validate_scores(&scores, &[c1, c2]).is_ok());
    }

    #[test]
    fn rejects_duplicate_scores() {
        let criterion = EvaluationCriterion {
            id: Uuid::new_v4(),
            max_score: 100,
        };
        let scores = vec![
            ScoreRequest {
                criterion_id: criterion.id,
                score: 80,
            },
            ScoreRequest {
                criterion_id: criterion.id,
                score: 90,
            },
        ];

        assert!(validate_scores(&scores, &[criterion]).is_err());
    }

    #[test]
    fn rejects_score_outside_max() {
        let criterion = EvaluationCriterion {
            id: Uuid::new_v4(),
            max_score: 75,
        };
        let scores = vec![ScoreRequest {
            criterion_id: criterion.id,
            score: 76,
        }];

        assert!(validate_scores(&scores, &[criterion]).is_err());
    }

    #[test]
    fn evaluations_require_closed_round_and_running_tournament() {
        assert!(ensure_evaluation_window("submission_closed", "running").is_ok());
        assert!(ensure_evaluation_window("active", "running").is_err());
        assert!(ensure_evaluation_window("submission_closed", "finished").is_err());
    }
}
