use axum::http::StatusCode;
use axum::{Json, routing::post, Router}; // Note: Changed to axum::Json for the return type
use axum_macros::debug_handler;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use crate::proto::{GetTokenRequest, Token};
use crate::routes::AppState;
use common::db::queries::{get_user, insert_user_async};
use common::models::user::User;
use chrono::Utc;
use common::hash::hash_password;

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub token: Token
}

#[derive(Deserialize)]
pub struct RegisterPayload {
    email: String,
    display_name: String,
    password: String,
}

#[debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(register_payload): Json<RegisterPayload>
) -> Result<Json<AuthResponse>, StatusCode> {
    let pool = state.db_pool.clone();
    let mut token_client = state.token_grpc_client.clone();
    
    let user_opt = get_user(pool.clone(), register_payload.email.clone())
        .await
        .map_err(|e| {
            eprintln!("Error in register: {:?}", e); // This prints to your terminal
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if user_opt.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let now = Utc::now().timestamp();
    let password_hash = hash_password(&register_payload.password);
    
    // FIX 2: Added .await and .map_err()
    // Futures do nothing unless you await them!
    insert_user_async(pool, User {
        id: None,
        display_name: register_payload.display_name,
        email: register_payload.email.clone(),
        password_hash: password_hash,
        created_at: now,
        updated_at: now,
    })
    .await 
    .map_err(|e| {
        eprintln!("Error in register: {:?}", e); // This prints to your terminal
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let request = GetTokenRequest {
        email: register_payload.email,
        password: register_payload.password
    };
    
    let response = token_client
        .get_token(request)
        .await
        .map_err(|e| {
            eprintln!("Error in register: {:?}", e); // This prints to your terminal
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let inner = response.into_inner();
    let token = inner.token.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        message: "Successfully registered".to_string(),
        token,
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
}