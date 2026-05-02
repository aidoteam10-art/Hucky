#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use uuid::Uuid;
    use crate::teams::routes::CreateTeamRequest;
    use crate::teams::service;

    // --- UNIT TESTS (Перевірка логіки в пам'яті) ---

    #[test]
    fn test_email_deduplication() {
        let emails = vec![
            "test@mail.com".to_string(),
            "test@mail.com".to_string(),
            "other@mail.com".to_string()
        ];
        let mut unique = emails.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), 2);
    }

    #[test]
    fn test_team_name_validation() {
        let short_name = "A ";
        let valid_name = "Code Masters";
        assert!(short_name.trim().len() < 2);
        assert!(valid_name.trim().len() >= 2);
    }

    #[test]
    fn test_organization_name_length() {
        let long_org = "A".repeat(121);
        let valid_org = "KPI Lyceum";
        assert!(long_org.len() > 120);
        assert!(valid_org.len() <= 120);
    }

    // --- INTEGRATION TESTS (Перевірка роботи з БД) ---

        #[sqlx::test]
        async fn test_create_team_in_db(pool: PgPool) {
            // 1. Створюємо капітана
            let creator_id = Uuid::new_v4();
            sqlx::query!(
                "INSERT INTO users (id, email, full_name, password_hash) VALUES ($1, $2, $3, $4)",
                creator_id,
                format!("test_{}@example.com", creator_id),
                "Test User",
                "hash"
            ).execute(&pool).await.unwrap();

            // 2. Створюємо турнір (враховуючи ВСІ обов'язкові поля зі схеми SQL)
            let tournament_id = Uuid::new_v4();
            sqlx::query!(
                r#"INSERT INTO tournaments
                   (id, organizer_id, title, description, registration_starts_at, registration_ends_at, starts_at)
                   VALUES ($1, $2, $3, $4, NOW(), NOW() + interval '1 day', NOW() + interval '2 days')"#,
                tournament_id,
                creator_id, // organizer_id
                "Test Tournament",
                "Description"
            ).execute(&pool).await.unwrap();

            let payload = CreateTeamRequest {
                name: "Команда Тест".to_string(),
                organization: None,
                contact: None,
                member_emails: vec![],
            };

            // 3. Реєстрація
            let result = service::register_new_team(&pool, tournament_id, creator_id, payload).await;

            if let Err(ref e) = result {
                eprintln!("Деталі помилки: {}", e);
            }

            assert!(result.is_ok(), "Команда мала створитися успішно");
            let team = result.unwrap();
            assert_eq!(team.name, "Команда Тест");
        }

        #[sqlx::test]
        async fn test_tournament_team_limit(pool: PgPool) {
            let creator_id = Uuid::new_v4();
            let tournament_id = Uuid::new_v4();

            // 1. Вставляємо користувача
            sqlx::query!(
                "INSERT INTO users (id, email, full_name, password_hash) VALUES ($1, $2, $3, $4)",
                creator_id,
                format!("captain_{}@test.com", creator_id),
                "Captain America",
                "hash"
            ).execute(&pool).await.unwrap();

            // 2. Вставляємо турнір
            sqlx::query!(
                r#"INSERT INTO tournaments
                   (id, organizer_id, title, description, registration_starts_at, registration_ends_at, starts_at)
                   VALUES ($1, $2, $3, $4, NOW(), NOW() + interval '1 day', NOW() + interval '2 days')"#,
                tournament_id,
                creator_id,
                "Mega Tournament",
                "Description"
            ).execute(&pool).await.unwrap();

            // Спроба 1: Перша команда (ОК)
            let payload1 = CreateTeamRequest {
                name: "Перша Команда".to_string(),
                organization: None,
                contact: None,
                member_emails: vec![],
            };
            let result1 = service::register_new_team(&pool, tournament_id, creator_id, payload1).await;
            assert!(result1.is_ok(), "Перша команда мала створитися");

            // Спроба 2: Друга команда тим самим юзером (Має бути помилка через UNIQUE INDEX)
            let payload2 = CreateTeamRequest {
                name: "Друга Команда".to_string(),
                organization: None,
                contact: None,
                member_emails: vec![],
            };
            let result2 = service::register_new_team(&pool, tournament_id, creator_id, payload2).await;

            assert!(result2.is_err(), "Має бути помилка: один юзер - одна команда в турнірі");
        }
}