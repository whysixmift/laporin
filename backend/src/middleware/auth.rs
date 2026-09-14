use crate::auth::hash_token;
use crate::db::session_repo::SessionRepo;
use crate::db::user_repo::UserRepo;
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
    pub email: String,
    pub role: String,
    pub is_active: bool,
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

        // 4. Fetch user details
        let user = UserRepo::find_by_id(&state.db, session.user_id)
            .await
            .map_err(|_| AppError::Internal("Database error looking up user".into()))?
            .ok_or_else(|| AppError::Unauthorized("User no longer exists".into()))?;

        if !user.is_active {
            return Err(AppError::Unauthorized("User account has been deactivated".into()));
        }

        Ok(AuthenticatedUser {
            user_id: session.user_id,
            session_id: session.id,
            token,
            email: user.email,
            role: user.role,
            is_active: user.is_active,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AdminUser(pub AuthenticatedUser);

#[async_trait]
impl FromRequestParts<AppState> for AdminUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_user = AuthenticatedUser::from_request_parts(parts, state).await?;
        if auth_user.role != "admin" {
            return Err(AppError::Forbidden("Admin privileges required".into()));
        }
        Ok(AdminUser(auth_user))
    }
}

