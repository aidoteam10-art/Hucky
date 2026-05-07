use sqlx::PgPool;
use uuid::Uuid;
use super::model::User;

pub struct UserRepository;


impl UserRepository{

    pub async fn count(db: &PgPool) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!("SELECT count(*) FROM users")
            .fetch_one(db)
            .await?;

        Ok(count.unwrap_or(0) as i64)
    }

    pub async fn create(
        db: &PgPool,
        email: &str,
        full_name: &str,
        password_hash: &str
    ) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar!(
            "INSERT INTO users (email, full_name, password_hash) VALUES ($1, $2, $3) RETURNING id",
            email, full_name, password_hash
        )
        .fetch_one(db)
        .await?;


        Ok(id)
    }

    pub async fn find_by_email(
        db: &PgPool,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, full_name, password_hash, created_at, updated_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(db)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(
        db: &PgPool,
        id: Uuid,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, full_name, password_hash, created_at, updated_at FROM users WHERE id = $1",
            id
        )
        .fetch_optional(db)
        .await?;

        Ok(user)
    }
}