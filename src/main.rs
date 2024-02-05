use axum::{routing::post, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod handlers;
mod model;
mod services;

#[tokio::main]
async fn main() {
    let services = services::make_services().await;
    let services = Arc::new(services);

    let router = Router::new()
        .route("/create-user", post(handlers::create_user))
        .route("/login", post(handlers::login))
        .layer(CorsLayer::permissive().allow_private_network(true))
        .with_state(services);

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
