use chrono::Utc;
use sqlx::PgPool;
use url::Url;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult},
    users::auth::AuthenticatedUser,
};

use super::{
    dto::{SubmissionRequest, SubmissionResponse},
    model::{Submission, SubmissionInput},
    repository::SubmissionRepository,
};

pub struct SubmissionService;

impl SubmissionService {
    pub async fn submit(
        db: &PgPool,
        user: AuthenticatedUser,
        round_id: Uuid,
        payload: SubmissionRequest,
    ) -> ApiResult<SubmissionResponse> {
        let normalized = NormalizedSubmission::from_request(payload)?;
        let round = SubmissionRepository::find_round(db, round_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Round not found".to_string()))?;

        if round.status != "active" {
            return Err(ApiError::Validation(
                "Submission is allowed only while round is active".to_string(),
            ));
        }

        if Utc::now() >= round.deadline_at {
            return Err(ApiError::Validation(
                "Round deadline has passed".to_string(),
            ));
        }

        let team_id = SubmissionRepository::captain_team_for_round(db, round.id, user.user_id)
            .await?
            .ok_or_else(|| {
                ApiError::Forbidden(
                    "Only accepted team captain can submit work for this round".to_string(),
                )
            })?;

        let accepted_members = SubmissionRepository::count_accepted_members(db, team_id).await?;
        if accepted_members < 2 {
            return Err(ApiError::Validation(
                "Team must have at least 2 accepted members before submission".to_string(),
            ));
        }

        if let Some(existing) =
            SubmissionRepository::find_for_team_round(db, round.id, team_id).await?
        {
            if existing.locked_at.is_some() || existing.status == "locked" {
                return Err(ApiError::Forbidden(
                    "Submission is locked and cannot be edited".to_string(),
                ));
            }
        }

        let submission = SubmissionRepository::upsert_submission(
            db,
            round.id,
            team_id,
            SubmissionInput {
                github_url: &normalized.github_url,
                video_demo_url: &normalized.video_demo_url,
                live_demo_url: normalized.live_demo_url.as_deref(),
                description: normalized.description.as_deref(),
            },
        )
        .await?;

        Ok(submission.into())
    }

    pub async fn get(
        db: &PgPool,
        user: AuthenticatedUser,
        round_id: Uuid,
    ) -> ApiResult<SubmissionResponse> {
        let submission = SubmissionRepository::find_for_user_round(db, round_id, user.user_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Submission not found".to_string()))?;

        Ok(submission.into())
    }

    pub async fn lock(db: &PgPool, user: AuthenticatedUser, round_id: Uuid) -> ApiResult<()> {
        let round = SubmissionRepository::find_round(db, round_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Round not found".to_string()))?;

        let is_organizer =
            SubmissionRepository::is_tournament_organizer(db, round.tournament_id, user.user_id)
                .await?;
        if !is_organizer {
            return Err(ApiError::Forbidden(
                "Only tournament organizer can lock submissions".to_string(),
            ));
        }

        if Utc::now() < round.deadline_at {
            return Err(ApiError::Validation(
                "Submissions can be locked only after the round deadline".to_string(),
            ));
        }

        if round.status == "evaluated" {
            return Err(ApiError::Validation(
                "Evaluated round cannot be locked again".to_string(),
            ));
        }

        let mut tx = db.begin().await?;
        SubmissionRepository::lock_round_submissions(&mut tx, round.id).await?;
        SubmissionRepository::update_round_status(&mut tx, round.id, "submission_closed").await?;
        tx.commit().await?;

        Ok(())
    }
}

struct NormalizedSubmission {
    github_url: String,
    video_demo_url: String,
    live_demo_url: Option<String>,
    description: Option<String>,
}

impl NormalizedSubmission {
    fn from_request(payload: SubmissionRequest) -> ApiResult<Self> {
        let github_url = required_url("github_url", payload.github_url)?;
        let video_demo_url = required_url("video_demo_url", payload.video_demo_url)?;
        let live_demo_url = optional_url("live_demo_url", payload.live_demo_url)?;
        let description = payload
            .description
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        if matches!(&description, Some(value) if value.chars().count() > 2000) {
            return Err(ApiError::Validation(
                "Description cannot exceed 2000 characters".to_string(),
            ));
        }

        Ok(Self {
            github_url,
            video_demo_url,
            live_demo_url,
            description,
        })
    }
}

fn required_url(field: &str, value: String) -> ApiResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ApiError::Validation(format!("{field} is required")));
    }

    validate_http_url(field, trimmed)?;
    Ok(trimmed.to_string())
}

fn optional_url(field: &str, value: Option<String>) -> ApiResult<Option<String>> {
    let Some(value) = value else {
        return Ok(None);
    };

    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    validate_http_url(field, trimmed)?;
    Ok(Some(trimmed.to_string()))
}

pub(crate) fn validate_http_url(field: &str, value: &str) -> ApiResult<()> {
    let parsed = Url::parse(value)
        .map_err(|_| ApiError::Validation(format!("{field} must be a valid URL")))?;

    match parsed.scheme() {
        "http" | "https" => Ok(()),
        _ => Err(ApiError::Validation(format!(
            "{field} must use http or https"
        ))),
    }
}

impl From<Submission> for SubmissionResponse {
    fn from(submission: Submission) -> Self {
        Self {
            id: submission.id,
            round_id: submission.round_id,
            team_id: submission.team_id,
            github_url: submission.github_url,
            video_demo_url: submission.video_demo_url,
            live_demo_url: submission.live_demo_url,
            description: submission.description,
            status: submission.status,
            submitted_at: submission.submitted_at,
            updated_at: submission.updated_at,
            locked_at: submission.locked_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_http_urls() {
        assert!(validate_http_url("github_url", "https://github.com/org/repo").is_ok());
    }

    #[test]
    fn rejects_non_url_values() {
        assert!(validate_http_url("github_url", "not-a-url").is_err());
    }

    #[test]
    fn rejects_non_http_urls() {
        assert!(validate_http_url("github_url", "ftp://example.com/repo").is_err());
    }

    #[test]
    fn normalizes_empty_optional_fields() {
        let normalized = NormalizedSubmission::from_request(SubmissionRequest {
            github_url: " https://github.com/org/repo ".to_string(),
            video_demo_url: "https://youtube.com/watch?v=1".to_string(),
            live_demo_url: Some("   ".to_string()),
            description: Some("  demo  ".to_string()),
        })
        .unwrap();

        assert_eq!(normalized.github_url, "https://github.com/org/repo");
        assert_eq!(normalized.live_demo_url, None);
        assert_eq!(normalized.description, Some("demo".to_string()));
    }
}
