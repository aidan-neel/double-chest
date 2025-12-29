use std::env;

use tonic::{Request, Response, Status};
use chrono::Utc;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use common::db::queries::{get_user, insert_refresh_async};
use common::models::refresh::Refresh;
use crate::proto::{GetTokenRequest, GetTokenResponse, RefreshTokenRequest, RefreshTokenResponse, Token};
use crate::proto::token_service_server::TokenService;
use common::jwt::JWTTokenService;
use common::db::connection::DbPool; 
use dotenvy::dotenv;

#[derive(Clone)]
pub struct TokenServiceImpl {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl TokenService for TokenServiceImpl {
    async fn get_token(
        &self,
        request: Request<GetTokenRequest>
    ) -> Result<Response<GetTokenResponse>, Status> {
        let data = request.into_inner();
        dotenv().ok();
        
        let user_opt = get_user(self.pool.clone(), data.email)
            .await
            .map_err(|_| Status::internal("Database error"))?;

        let user = user_opt.ok_or_else(|| Status::unauthenticated("Invalid credentials"))?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| Status::internal("Invalid hash format in DB"))?;

        let is_valid = Argon2::default()
            .verify_password(data.password.as_bytes(), &parsed_hash)
            .is_ok();

        if !is_valid {
            return Err(Status::unauthenticated("Invalid credentials"));
        }

        let access = env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET missing");
        let refresh = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET missing");
        let jwt_service = JWTTokenService::new(access, refresh);
        
        let user_id_str = user.id.unwrap().to_string();

        let access_token = jwt_service.create_access_token(&user_id_str)
            .map_err(|_| Status::internal("Token generation failed"))?;
        
        let refresh_token = jwt_service.create_refresh_token(&user_id_str)
            .map_err(|_| Status::internal("Token generation failed"))?; 

        let now = Utc::now().timestamp();
        insert_refresh_async(self.pool.clone(), Refresh {
            id: None,
            user_id: user.id.expect("User must have ID"),
            token_hash: refresh_token.clone(),
            created_at: now,
            expires_at: now + (jwt_service.refresh_days * 24 * 60 * 60),
        })
        .await
        .map_err(|_| Status::internal("Failed to save refresh token"))?;

        Ok(Response::new(GetTokenResponse {
            token: Some(Token {
                access_token, 
                refresh_token,
            }),
        }))
    }

    async fn refresh_token(
        &self,
        request: Request<RefreshTokenRequest>,
    ) -> Result<Response<RefreshTokenResponse>, Status> {
        dotenv().ok();
        let req = request.into_inner();
        let token_str = req.refresh_token;

        let access = env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET missing");
        let refresh = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET missing");
        let jwt_service = JWTTokenService::new(access, refresh);

        let refresh = common::db::queries::get_refresh_by_token(self.pool.clone(), token_str)
            .await
            .map_err(|_| Status::unauthenticated("Invalid refresh token"))?
            .ok_or_else(|| Status::unauthenticated("Refresh token not found"))?;
        
        let now = chrono::Utc::now().timestamp();
        if refresh.expires_at < now {
            return Err(Status::unauthenticated("Refresh token expired"));
        }
        let access_token = jwt_service.create_access_token(&refresh.user_id.to_string())
            .map_err(|_| Status::internal("Failed to issue access token"))?;

        let refresh_token = jwt_service.create_refresh_token(&refresh.user_id.to_string())
            .map_err(|_| Status::internal("Failed to issue refresh token"))?;
        let now = Utc::now().timestamp();
        insert_refresh_async(self.pool.clone(), Refresh {
            id: None,
            user_id: refresh.user_id,
            token_hash: refresh_token.clone(),
            created_at: now,
            expires_at: now + (jwt_service.refresh_days * 24 * 60 * 60),
        })
        .await
        .map_err(|_| Status::internal("Failed to save refresh token"))?;

        let response = Token {
            access_token: access_token,
            refresh_token: refresh_token
        };

        Ok(Response::new(RefreshTokenResponse {
            token: Some(response),
        }))
    }

}