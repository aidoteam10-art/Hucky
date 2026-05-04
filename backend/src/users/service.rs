use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use uuid::Uuid;
use super::{
    model::{Claims, LoginRequest, RegisterUserRequest}, 
    repository::UserRepository
};



pub struct UserService;

impl UserService {
    pub async fn register_user(
        db: &PgPool,
        payload: RegisterUserRequest,
    ) -> Result<Uuid, String>{
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(payload.password.as_bytes(), &salt)
            .map_err(|e| format!("Помилка хешвання паролю: {}", e))?
            .to_string();

       
        let new_user_id = UserRepository::create(
            db,
            &payload.email,
            &payload.full_name,
            &password_hash,
        )
        .await
        .map_err(|e| format!("Помилка бази даних: {}", e))?;

        Ok(new_user_id)
}
    pub async fn login_user(db: &PgPool, payload: LoginRequest) -> Result<String, String> {
        let user = UserRepository::find_by_email(db, &payload.email)
            .await
            .map_err(|e| format!("Помилка бази даних: {}", e))?;

        let user = match user {
            Some(u) => u,
            None => return Err("Користувача не знайдено або пароль неправильний".into()),
        };

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| "Помилка при перевірці хешу".to_string())?;
        
        let is_valid = Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash)
            .is_ok();

        if !is_valid {
            return Err("Користувача не знайдено або пароль неправильний".into());
        }

        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("Помилка генерації часу")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id,
            exp: expiration,
        };

        // ТИМЧАСОВИЙ СЕКРЕТНИЙ КЛЮЧ: в продакшені це має братись з .env!
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("супер_секретний_ключ".as_ref()),
        )
        .map_err(|e| format!("Помилка генерації токену: {}", e))?;

        Ok(token)
    }
}