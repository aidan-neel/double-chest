use tonic::{Request, Response, Status};
use chrono::Utc;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use common::db::queries::{get_user, insert_refresh_async};
use common::models::refresh::Refresh;
use crate::proto::{GetTokenRequest, GetTokenResponse, RefreshTokenRequest, RefreshTokenResponse, Token};
use crate::proto::token_service_server::TokenService;
use crate::jwt::JWTTokenService;
use common::db::connection::DbPool; 

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

        let jwt_service = JWTTokenService::new("access_secret".into(), "refresh_secret".into());
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
        _request: Request<RefreshTokenRequest>
    ) -> Result<Response<RefreshTokenResponse>, Status> {
        // Implementation here...
        Ok(Response::new(RefreshTokenResponse { token: None }))
    }
}