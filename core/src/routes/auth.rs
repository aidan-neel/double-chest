use axum::http::StatusCode;
use axum::{Json, routing::post, Router};
use axum_macros::debug_handler;
use axum::extract::State;
use common::utils::tonic_to_http_response;
use serde::{Deserialize, Serialize};
use crate::proto::{GetTokenRequest, GetTokenResponse, RefreshTokenRequest, RefreshTokenResponse, Token};
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

#[derive(Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct RefreshPayload {
    refresh_token: String,
}

#[debug_handler]
async fn login(
    State(state): State<AppState>,
    Json(login_payload): Json<LoginPayload>
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let mut token_client = state.token_grpc_client.clone();
    let request = GetTokenRequest {
        email: login_payload.email,
        password: login_payload.password
    };
    
    let response: tonic::Response<GetTokenResponse> = token_client
        .get_token(request)
        .await
        .map_err(|e| {
            eprintln!("Error in register: {:?}", e);
            tonic_to_http_response(e)
        })?;

    let inner = response.into_inner();
    let token = inner.token.ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()))?;

    Ok(Json(AuthResponse {
        message: "Successfully signed in".to_string(),
        token: token,
    }))
}

#[debug_handler]
async fn refresh(
    State(state): State<AppState>,
    Json(refresh_payload): Json<RefreshPayload>
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let mut token_client = state.token_grpc_client.clone();

    let request = RefreshTokenRequest {
        refresh_token: refresh_payload.refresh_token
    };

    let response: tonic::Response<RefreshTokenResponse> = token_client
        .refresh_token(request)
        .await
        .map_err(|e| {
            eprintln!("Error in refresh: {:?}", e);
            tonic_to_http_response(e)
        })?;

    let inner = response.into_inner();
    let token = inner.token.ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()))?;

    Ok(Json(AuthResponse {
        message: "Successfully refreshed".to_string(),
        token: token,
    }))

}

#[debug_handler]
async fn register(
    State(state): State<AppState>,
    Json(register_payload): Json<RegisterPayload>
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let pool = state.db_pool.clone();
    let mut token_client = state.token_grpc_client.clone();
    
    let user_opt = get_user(pool.clone(), register_payload.email.clone())
        .await
        .map_err(|e| {
            eprintln!("Error in register: {:?}", e); // This prints to your terminal
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    if user_opt.is_some() {
        return Err((StatusCode::CONFLICT, "User already exists".to_string()));
    }

    let now = Utc::now().timestamp();
    let password_hash = hash_password(&register_payload.password);
    
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
        eprintln!("Error in register: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    
    let request = GetTokenRequest {
        email: register_payload.email,
        password: register_payload.password
    };
    
    let response = token_client
        .get_token(request)
        .await
        .map_err(|e| {
            eprintln!("Error in register: {:?}", e);
            tonic_to_http_response(e)
        })?;

    let inner = response.into_inner();
    let token = inner.token.ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()))?;

    Ok(Json(AuthResponse {
        message: "Successfully registered".to_string(),
        token,
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
}