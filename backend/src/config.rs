use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
    pub frontend_origin: String,
    pub superadmin_email: Option<String>,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn init() -> &'static Self {
        CONFIG.get_or_init(Self::from_env)
    }

    pub fn get() -> &'static Self {
        CONFIG.get().expect("Config must be initialized before use")
    }

    pub fn jwt_secret_bytes(&self) -> &[u8] {
        self.jwt_secret.as_bytes()
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    fn from_env() -> Self {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");

        if jwt_secret.chars().count() < 32 {
            panic!("JWT_SECRET must be at least 32 characters long");
        }

        let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = std::env::var("SERVER_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(8080);
        let frontend_origin = std::env::var("FRONTEND_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:5173".to_string());
        let superadmin_email = std::env::var("SUPERADMIN_EMAIL")
            .or_else(|_| std::env::var("ADMIN_EMAIL"))
            .ok()
            .map(|value| value.trim().to_lowercase())
            .filter(|value| !value.is_empty());

        Self {
            database_url,
            jwt_secret,
            server_host,
            server_port,
            frontend_origin,
            superadmin_email,
        }
    }
}
