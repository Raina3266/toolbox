use std::{sync::Arc, time::Duration};

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{model::User, services::Services};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}
/// { "session_token": "12345678-abcd-1234..."}
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    session_token: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    session_token: Uuid,
}

pub async fn create_user(
    state: State<Arc<Services>>,
    Json(request2): Json<CreateUserRequest>,
) -> Json<CreateUserResponse> {
    let email = request2.username;
    let password = request2.password;
    let id = Uuid::new_v4();

    let new_user = User {
        id,
        email,
        password,
    };

    state.db.insert_user(new_user).await;
    let token = state
        .db
        .create_session(id, Duration::from_secs(1_000))
        .await;
    let create_response = CreateUserResponse {
        session_token: token,
    };

    Json(create_response)
}

pub async fn login(
    state: State<Arc<Services>>,
    Json(request): Json<LoginRequest>,
) -> Json<LoginResponse> {
    let email = request.username;
    let result = state.db.find_user_by_email(email).await;
    match result {
        None => panic!("no user with that email :("),
        Some(user) => {
            if user.password == request.password {
                let token = state
                    .db
                    .create_session(user.id, Duration::from_secs(1_000))
                    .await;
                let login_response = LoginResponse {
                    session_token: token,
                };
                Json(login_response)
            } else {
                panic!("password doesn't match")
            }
        }
    }
}
