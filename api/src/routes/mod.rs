pub mod auth;
pub mod storage;

use std::env;
use std::sync::Arc;
use common::db::connection::{DbPool, establish_pool};
use common::jwt::JWTTokenService;
use crate::middleware::jwt::jwt_auth;
use crate::proto::upload_service_client::UploadServiceClient;
use dotenvy::dotenv;
use axum::{Router};
use crate::proto::token_service_client::TokenServiceClient;

#[derive(Clone)]
pub struct AppState {
    db_pool: DbPool,
    token_grpc_client: TokenServiceClient<tonic::transport::Channel>,
    upload_grpc_client: UploadServiceClient<tonic::transport::Channel>,
    pub jwt: Arc<JWTTokenService>
}

pub async fn create_router() -> Result<Router, tonic::transport::Error> {
    dotenv().ok();

    let access = env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET missing");
    let refresh = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET missing");
    
    let pool = establish_pool();
    let token_client = TokenServiceClient::connect("http://[::1]:50052").await?;
    let upload_client = UploadServiceClient::connect("http://[::1]:50051").await?;
    let jwt_service = Arc::new(JWTTokenService::new(access, refresh));

    let state = AppState { 
        db_pool: pool, 
        token_grpc_client: token_client,
        upload_grpc_client: upload_client,
        jwt: jwt_service
    };
    
    Ok(Router::new()
        .nest("/auth", auth::router())
        .nest("/storage", storage::router()
            .layer(axum::middleware::from_fn_with_state(state.clone(), jwt_auth)))
        .fallback(handler_404)
        .with_state(state)
    )
}

async fn handler_404() -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "Route not found - check your nesting!")
}