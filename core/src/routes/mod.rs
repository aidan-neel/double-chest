pub mod auth;

use common::db::connection::{DbPool, establish_pool};

use axum::Router;

use crate::proto::token_service_client::TokenServiceClient;

#[derive(Clone)]
struct AppState {
    db_pool: DbPool,
    token_grpc_client: TokenServiceClient<tonic::transport::Channel>
}

pub async fn create_router() -> Result<Router, tonic::transport::Error> {
    let pool = establish_pool();
    let token_client = TokenServiceClient::connect("http://[::1]:50051").await?;
    let state = AppState { 
        db_pool: pool, 
        token_grpc_client: token_client 
    };
    
    Ok(Router::new()
        .nest("/auth", auth::router())
        .fallback(handler_404)
        .with_state(state))
}

async fn handler_404() -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "Route not found - check your nesting!")
}