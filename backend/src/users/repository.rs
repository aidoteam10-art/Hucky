use super::model::{AccountRole, User};
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

pub struct UserRepository;

#[derive(Debug, sqlx::FromRow)]
pub struct AdminUserRow {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub account_role: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl UserRepository {
    pub async fn count(db: &PgPool) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar::<_, i64>("SELECT count(*)::bigint FROM users")
            .fetch_one(db)
            .await?;

        Ok(count)
    }

    pub async fn create(
        db: &PgPool,
        email: &str,
        full_name: &str,
        password_hash: &str,
        account_role: AccountRole,
    ) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO users (email, full_name, password_hash, account_role)
            VALUES ($1, $2, $3, $4)
            RETURNING id",
        )
        .bind(email)
        .bind(full_name)
        .bind(password_hash)
        .bind(account_role.as_str())
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    pub async fn find_by_email(db: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, full_name, account_role, password_hash, created_at, updated_at
            FROM users
            WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(db)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(db: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, full_name, account_role, password_hash, created_at, updated_at
            FROM users
            WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(db)
        .await?;

        Ok(user)
    }

    pub async fn account_role(
        db: &PgPool,
        user_id: Uuid,
    ) -> Result<Option<AccountRole>, sqlx::Error> {
        let role = sqlx::query_scalar::<_, String>(
            "SELECT account_role
            FROM users
            WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(db)
        .await?;

        Ok(role.and_then(|value| value.parse::<AccountRole>().ok()))
    }

    pub async fn promote_to_jury_if_participant(
        db: &PgPool,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users
            SET account_role = 'jury', updated_at = NOW()
            WHERE id = $1 AND account_role = 'participant'",
        )
        .bind(user_id)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn set_role(
        db: &PgPool,
        user_id: Uuid,
        role: AccountRole,
    ) -> Result<Option<AdminUserRow>, sqlx::Error> {
        sqlx::query_as::<_, AdminUserRow>(
            "UPDATE users
            SET account_role = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, email, full_name, account_role, created_at",
        )
        .bind(user_id)
        .bind(role.as_str())
        .fetch_optional(db)
        .await
    }

    pub async fn bootstrap_superadmin(
        db: &PgPool,
        email: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users
            SET account_role = 'admin', updated_at = NOW()
            WHERE lower(email) = lower($1)",
        )
        .bind(email)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn list_admin_users(
        db: &PgPool,
        role: Option<AccountRole>,
        search: Option<&str>,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<AdminUserRow>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let mut list_builder = QueryBuilder::<Postgres>::new(
            "SELECT id, email, full_name, account_role, created_at FROM users",
        );
        push_admin_user_filters(&mut list_builder, role, search);
        list_builder
            .push(" ORDER BY created_at DESC, full_name ASC LIMIT ")
            .push_bind(per_page)
            .push(" OFFSET ")
            .push_bind(offset);

        let items = list_builder
            .build_query_as::<AdminUserRow>()
            .fetch_all(db)
            .await?;

        let mut count_builder = QueryBuilder::<Postgres>::new("SELECT COUNT(*)::bigint FROM users");
        push_admin_user_filters(&mut count_builder, role, search);
        let total = count_builder
            .build_query_scalar::<i64>()
            .fetch_one(db)
            .await?;

        Ok((items, total))
    }
}

fn push_admin_user_filters(
    builder: &mut QueryBuilder<'_, Postgres>,
    role: Option<AccountRole>,
    search: Option<&str>,
) {
    builder.push(" WHERE 1 = 1");

    if let Some(role) = role {
        builder.push(" AND account_role = ");
        builder.push_bind(role.as_str());
    }

    if let Some(search) = search.map(str::trim).filter(|value| !value.is_empty()) {
        let pattern = format!("%{}%", escape_like_pattern(&search.to_lowercase()));
        builder.push(" AND (lower(full_name) LIKE ");
        builder.push_bind(pattern.clone());
        builder.push(" ESCAPE '\\' OR lower(email) LIKE ");
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
