use super::model::{InvitationStatus, TeamInvitation};
use super::service::{normalize_member_emails, validate_team_fields};
use chrono::{Duration, Utc};
use uuid::Uuid;

#[test]
fn email_list_rejects_duplicates_case_insensitively() {
    let emails = vec![
        "member@example.com".to_string(),
        " MEMBER@example.com ".to_string(),
    ];

    assert!(normalize_member_emails(&emails).is_err());
}

#[test]
fn team_name_validation_rejects_blankish_value() {
    assert!(validate_team_fields(" ", &None, &None).is_err());
}

#[test]
fn invitation_expiration_test_fixture_is_expired() {
    let invitation = TeamInvitation {
        id: Uuid::new_v4(),
        team_id: Uuid::new_v4(),
        tournament_id: Uuid::new_v4(),
        email: "member@example.com".to_string(),
        invited_user_id: None,
        invited_by: Uuid::new_v4(),
        status: InvitationStatus::Pending.as_str().to_string(),
        expires_at: Utc::now() - Duration::minutes(1),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    assert!(invitation.expires_at < Utc::now());
}
