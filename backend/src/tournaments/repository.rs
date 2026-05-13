use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

use super::model::{
    ActiveRoundPreview, NewTournament, Tournament, TournamentListFilter, TournamentListItem,
    TournamentStatus, UpdateTournament,
};

pub struct TournamentRepository;

impl TournamentRepository {
    pub async fn insert(
        tx: &mut Transaction<'_, Postgres>,
        tournament: NewTournament<'_>,
    ) -> Result<Tournament, sqlx::Error> {
        sqlx::query_as::<_, Tournament>(
            "INSERT INTO tournaments (
                organizer_id,
                title,
                description,
                rules,
                status,
                registration_starts_at,
                registration_ends_at,
                starts_at,
                max_teams
            )
            VALUES ($1, $2, $3, $4, 'draft', $5, $6, $7, $8)
            RETURNING id, organizer_id, title, description, rules, status, registration_starts_at,
                registration_ends_at, starts_at, ends_at, max_teams",
        )
        .bind(tournament.organizer_id)
        .bind(tournament.title)
        .bind(tournament.description)
        .bind(tournament.rules)
        .bind(tournament.registration_starts_at)
        .bind(tournament.registration_ends_at)
        .bind(tournament.starts_at)
        .bind(tournament.max_teams)
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn insert_organizer_role(
        tx: &mut Transaction<'_, Postgres>,
        tournament_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO tournament_staff_roles (tournament_id, user_id, role)
            VALUES ($1, $2, 'organizer')
            ON CONFLICT (tournament_id, user_id, role) DO NOTHING",
        )
        .bind(tournament_id)
        .bind(user_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(
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

    pub async fn list(
        db: &PgPool,
        filter: &TournamentListFilter,
    ) -> Result<(Vec<TournamentListItem>, i64), sqlx::Error> {
        let offset = (filter.page - 1) * filter.per_page;

        let mut list_builder = QueryBuilder::<Postgres>::new(
            "SELECT
                t.id,
                t.title,
                t.description,
                t.status,
                t.starts_at,
                COALESCE((SELECT COUNT(*) FROM rounds r WHERE r.tournament_id = t.id), 0)::bigint AS rounds_count,
                t.max_teams,
                COALESCE((SELECT COUNT(*) FROM teams tm WHERE tm.tournament_id = t.id), 0)::bigint AS registered_teams
            FROM tournaments t",
        );

        push_filters(
            &mut list_builder,
            filter.status,
            filter.search.as_deref(),
            filter.viewer_id,
        );
        list_builder
            .push(" ORDER BY t.created_at DESC LIMIT ")
            .push_bind(filter.per_page)
            .push(" OFFSET ")
            .push_bind(offset);

        let items = list_builder
            .build_query_as::<TournamentListItem>()
            .fetch_all(db)
            .await?;

        let mut count_builder =
            QueryBuilder::<Postgres>::new("SELECT COUNT(*)::bigint FROM tournaments t");
        push_filters(
            &mut count_builder,
            filter.status,
            filter.search.as_deref(),
            filter.viewer_id,
        );

        let total = count_builder
            .build_query_scalar::<i64>()
            .fetch_one(db)
            .await?;

        Ok((items, total))
    }

    pub async fn list_for_organizer(
        db: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<TournamentListItem>, sqlx::Error> {
        sqlx::query_as::<_, TournamentListItem>(
            "SELECT
                t.id,
                t.title,
                t.description,
                t.status,
                t.starts_at,
                COALESCE((SELECT COUNT(*) FROM rounds r WHERE r.tournament_id = t.id), 0)::bigint AS rounds_count,
                t.max_teams,
                COALESCE((SELECT COUNT(*) FROM teams tm WHERE tm.tournament_id = t.id), 0)::bigint AS registered_teams
            FROM tournaments t
            WHERE t.organizer_id = $1
                OR EXISTS (
                    SELECT 1
                    FROM tournament_staff_roles tsr
                    WHERE tsr.tournament_id = t.id
                        AND tsr.user_id = $1
                        AND tsr.role = 'organizer'
                )
            ORDER BY t.created_at DESC",
        )
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn update(
        db: &PgPool,
        tournament_id: Uuid,
        tournament: UpdateTournament<'_>,
    ) -> Result<Tournament, sqlx::Error> {
        sqlx::query_as::<_, Tournament>(
            "UPDATE tournaments
            SET title = $2,
                description = $3,
                rules = $4,
                registration_starts_at = $5,
                registration_ends_at = $6,
                starts_at = $7,
                ends_at = $8,
                max_teams = $9,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, organizer_id, title, description, rules, status, registration_starts_at,
                registration_ends_at, starts_at, ends_at, max_teams",
        )
        .bind(tournament_id)
        .bind(tournament.title)
        .bind(tournament.description)
        .bind(tournament.rules)
        .bind(tournament.registration_starts_at)
        .bind(tournament.registration_ends_at)
        .bind(tournament.starts_at)
        .bind(tournament.ends_at)
        .bind(tournament.max_teams)
        .fetch_one(db)
        .await
    }

    pub async fn update_status(
        db: &PgPool,
        tournament_id: Uuid,
        status: TournamentStatus,
    ) -> Result<Tournament, sqlx::Error> {
        sqlx::query_as::<_, Tournament>(
            "UPDATE tournaments
            SET status = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, organizer_id, title, description, rules, status, registration_starts_at,
                registration_ends_at, starts_at, ends_at, max_teams",
        )
        .bind(tournament_id)
        .bind(status.as_str())
        .fetch_one(db)
        .await
    }

    pub async fn is_organizer(
        db: &PgPool,
        tournament_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar::<_, bool>(
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
        .await?;

        Ok(exists)
    }

    pub async fn count_rounds(db: &PgPool, tournament_id: Uuid) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*)::bigint FROM rounds WHERE tournament_id = $1")
            .bind(tournament_id)
            .fetch_one(db)
            .await
    }

    pub async fn active_round(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<Option<ActiveRoundPreview>, sqlx::Error> {
        sqlx::query_as::<_, ActiveRoundPreview>(
            "SELECT id, title, deadline_at
            FROM rounds
            WHERE tournament_id = $1 AND status = 'active'
            ORDER BY position ASC
            LIMIT 1",
        )
        .bind(tournament_id)
        .fetch_optional(db)
        .await
    }

    pub async fn registered_teams(
        db: &PgPool,
        tournament_id: Uuid,
    ) -> Result<Vec<crate::tournaments::dto::RegisteredTeamPreview>, sqlx::Error> {
        sqlx::query_as::<_, crate::tournaments::dto::RegisteredTeamPreview>(
            "SELECT
                tm.id,
                tm.name,
                COUNT(ms.user_id) FILTER (WHERE ms.status = 'accepted')::bigint AS members_count
            FROM teams tm
            LEFT JOIN team_memberships ms ON ms.team_id = tm.id
            WHERE tm.tournament_id = $1
            GROUP BY tm.id
            ORDER BY tm.created_at DESC",
        )
        .bind(tournament_id)
        .fetch_all(db)
        .await
    }
}

fn push_filters(
    builder: &mut QueryBuilder<'_, Postgres>,
    status: Option<TournamentStatus>,
    search: Option<&str>,
    viewer_id: Option<Uuid>,
) {
    builder.push(" WHERE 1 = 1");

    if let Some(viewer_id) = viewer_id {
        builder.push(" AND (t.status <> 'draft' OR t.organizer_id = ");
        builder.push_bind(viewer_id);
        builder.push(")");
    } else {
        builder.push(" AND t.status <> 'draft'");
    }

    if let Some(status) = status {
        builder.push(" AND t.status = ");
        builder.push_bind(status.as_str());
    }

    if let Some(search) = search.map(str::trim).filter(|value| !value.is_empty()) {
        let pattern = format!("%{}%", escape_like_pattern(&search.to_lowercase()));
        builder.push(" AND (LOWER(t.title) LIKE ");
        builder.push_bind(pattern.clone());
        builder.push(" ESCAPE '\\' OR LOWER(t.description) LIKE ");
        builder.push_bind(pattern);
        builder.push(" ESCAPE '\\')");
    }
}

fn escape_like_pattern(input: &str) -> String {
    input
        .replace('\\', r"\\")
        .replace('%', r"\%")
        .replace('_', r"\_")
}
