use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::routes::AppState;

#[derive(Clone)]
pub struct UserId(pub i32);

pub async fn jwt_auth(
    State(state): State<AppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let auth = req.headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".into()))?;
    
    let claims = state
        .jwt
        .verify_token(auth, false)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token".into()))?;

    let user_id = claims
        .subject
        .ok_or((StatusCode::UNAUTHORIZED, "Missing subject".into()))?;
    let i32_user_id = user_id.parse::<i32>()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    println!("{}", user_id.to_string());
    req.extensions_mut().insert(UserId(i32_user_id));

    Ok(next.run(req).await)
}
