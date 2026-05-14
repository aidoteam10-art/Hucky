use std::time::Duration;

use sqlx::PgPool;

#[derive(Debug, Default)]
pub struct StatusAutomationReport {
    pub tournaments_opened_registration: u64,
    pub tournaments_started: u64,
    pub tournaments_finished: u64,
    pub rounds_activated: u64,
    pub rounds_closed: u64,
    pub invitations_expired: u64,
}

impl StatusAutomationReport {
    fn has_changes(&self) -> bool {
        self.tournaments_opened_registration > 0
            || self.tournaments_started > 0
            || self.tournaments_finished > 0
            || self.rounds_activated > 0
            || self.rounds_closed > 0
            || self.invitations_expired > 0
    }
}

pub fn spawn_status_automation(db: PgPool) {
    tokio::spawn(async move {
        loop {
            match run_due_status_transitions(&db).await {
                Ok(report) if report.has_changes() => {
                    println!(
                        "Status automation updated: registration={}, running={}, finished={}, active_rounds={}, closed_rounds={}, expired_invitations={}",
                        report.tournaments_opened_registration,
                        report.tournaments_started,
                        report.tournaments_finished,
                        report.rounds_activated,
                        report.rounds_closed,
                        report.invitations_expired
                    );
                }
                Ok(_) => {}
                Err(error) => {
                    eprintln!("Status automation failed: {error}");
                }
            }

            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
}

pub async fn run_due_status_transitions(
    db: &PgPool,
) -> Result<StatusAutomationReport, sqlx::Error> {
    let tournaments_opened_registration = sqlx::query(
        "UPDATE tournaments
        SET status = 'registration', updated_at = NOW()
        WHERE status = 'draft'
            AND registration_starts_at <= NOW()",
    )
    .execute(db)
    .await?
    .rows_affected();

    let tournaments_started = sqlx::query(
        "UPDATE tournaments
        SET status = 'running', updated_at = NOW()
        WHERE status = 'registration'
            AND starts_at <= NOW()",
    )
    .execute(db)
    .await?
    .rows_affected();

    let rounds_activated = sqlx::query(
        "UPDATE rounds r
        SET status = 'active', updated_at = NOW()
        WHERE r.status = 'draft'
            AND r.starts_at <= NOW()
            AND EXISTS (
                SELECT 1
                FROM tournaments t
                WHERE t.id = r.tournament_id
                    AND t.status = 'running'
            )",
    )
    .execute(db)
    .await?
    .rows_affected();

    let rounds_closed = sqlx::query(
        "UPDATE rounds
        SET status = 'submission_closed', updated_at = NOW()
        WHERE status = 'active'
            AND deadline_at <= NOW()",
    )
    .execute(db)
    .await?
    .rows_affected();

    let tournaments_finished = sqlx::query(
        "UPDATE tournaments t
        SET status = 'finished',
            ends_at = COALESCE(t.ends_at, NOW()),
            updated_at = NOW()
        WHERE t.status = 'running'
            AND EXISTS (
                SELECT 1
                FROM rounds r
                WHERE r.tournament_id = t.id
            )
            AND NOT EXISTS (
                SELECT 1
                FROM rounds r
                WHERE r.tournament_id = t.id
                    AND r.status <> 'evaluated'
            )",
    )
    .execute(db)
    .await?
    .rows_affected();

    let invitations_expired = sqlx::query(
        "UPDATE team_invitations i
        SET status = 'expired', updated_at = NOW()
        WHERE i.status = 'pending'
            AND (
                i.expires_at <= NOW()
                OR EXISTS (
                    SELECT 1
                    FROM tournaments t
                    WHERE t.id = i.tournament_id
                        AND (
                            t.status <> 'registration'
                            OR t.registration_ends_at <= NOW()
                        )
                )
            )",
    )
    .execute(db)
    .await?
    .rows_affected();

    Ok(StatusAutomationReport {
        tournaments_opened_registration,
        tournaments_started,
        tournaments_finished,
        rounds_activated,
        rounds_closed,
        invitations_expired,
    })
}
