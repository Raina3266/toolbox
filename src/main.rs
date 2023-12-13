use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};
use model::User;
use services::Services;
use tokio::net::TcpListener;

mod model;
mod services;

#[tokio::main]
async fn main() {
    let services = services::make_services().await;
    let services = Arc::new(services);

    let router = Router::new()
        .route("/create_user", post(create_user_handler))
        .with_state(services);
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn create_user_handler(state: State<Arc<Services>>, user: Json<User>) {
    println!("creating user: {user:#?}");
    state.insert_user(user.0).await;

}
