use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use tokio::net::TcpListener;


#[derive(Serialize)]
struct HealthResponse{
    status: String,
    message : String,
}

#[tokio::main]
async fn main(){
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/ping", get(ping_handler))
        .layer(cors);


    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("backend runs on 127.0.0.1:8080");


    axum::serve(listener, app).await.unwrap();
}


async fn ping_handler() -> Json<HealthResponse>{
    let response = HealthResponse {
        status : "success".to_string(),
        message : "Api works!!!".to_string(),};
    Json(response)
    }