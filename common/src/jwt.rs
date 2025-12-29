use jwt_simple::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomClaims {
    pub is_refresh: bool,
}

pub struct JWTTokenService {
    access_key: HS256Key,
    refresh_key: HS256Key,
    pub access_mins: i64,
    pub refresh_days: i64,
}

impl JWTTokenService {
    pub fn new(access_secret: String, refresh_secret: String) -> Self {
        Self {
            access_key: HS256Key::from_bytes(access_secret.as_bytes()),
            refresh_key: HS256Key::from_bytes(refresh_secret.as_bytes()),
            access_mins: 15,
            refresh_days: 14,
        }
    }
    
    pub fn create_access_token(&self, user_id: &str) -> Result<String, jwt_simple::Error> {
        let claims = Claims::with_custom_claims(
            CustomClaims { is_refresh: false },
            Duration::from_mins(self.access_mins as u64), 
        )
        .with_subject(user_id);

        self.access_key.authenticate(claims)
    }

    pub fn create_refresh_token(&self, user_id: &str) -> Result<String, jwt_simple::Error> {
        let claims = Claims::with_custom_claims(
            CustomClaims { is_refresh: true },
            Duration::from_days(self.refresh_days as u64),
        )
        .with_subject(user_id);

        self.refresh_key.authenticate(claims)
    }

    pub fn verify_token(&self, token: &str, is_refresh: bool) -> Result<JWTClaims<CustomClaims>, jwt_simple::Error> {
        let key = if is_refresh { &self.refresh_key } else { &self.access_key };
        
        let claims = key.verify_token::<CustomClaims>(token, None)?;

        if claims.custom.is_refresh != is_refresh {
            return Err(jwt_simple::Error::msg("Token type mismatch"));
        }

        Ok(claims)
    }
}