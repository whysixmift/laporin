use crate::auth::hash_token;
use crate::db::session_repo::SessionRepo;
use crate::errors::AppError;
use crate::state::AppState;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::cookie::CookieJar;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub token: String,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1. Try to read session token from cookie
        let jar = CookieJar::from_headers(&parts.headers);
        let token_opt = jar
            .get(&state.config.session_cookie_name)
            .map(|c| c.value().to_string())
            .or_else(|| {
                // Fallback: check Authorization: Bearer <token> for testing/API clients
                parts
                    .headers
                    .get("authorization")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.strip_prefix("Bearer "))
                    .map(|s| s.to_string())
            });

        let token =
            token_opt.ok_or_else(|| AppError::Unauthorized("Authentication required".into()))?;

        // 2. Hash token and lookup in sessions table
        let token_hash = hash_token(&token);
        let session = SessionRepo::find_by_token_hash(&state.db, &token_hash)
            .await
            .map_err(|_| AppError::Internal("Database error looking up session".into()))?
            .ok_or_else(|| AppError::Unauthorized("Invalid session token".into()))?;

        // 3. Verify session state
        if session.revoked {
            return Err(AppError::SessionRevoked);
        }

        if session.expires_at < Utc::now() {
            return Err(AppError::SessionExpired);
        }

        Ok(AuthenticatedUser {
            user_id: session.user_id,
            session_id: session.id,
            token,
        })
    }
}
