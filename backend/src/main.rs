use backend::{automation, build_app, config::Config, state::AppState, users};
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::init().clone();

    println!("Connecting to db...");
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to db");
    println!("Successfully connected to db!!!");
    users::service::UserService::bootstrap_superadmin(&pool)
        .await
        .expect("Failed to bootstrap superadmin");
    automation::spawn_status_automation(pool.clone());

    let app = build_app(AppState { db: pool }, &config);
    let bind_addr = config.bind_addr();
    let listener = TcpListener::bind(&bind_addr).await.unwrap();
    println!("Backend runs on http://{}", bind_addr);

    axum::serve(listener, app).await.unwrap();
}
