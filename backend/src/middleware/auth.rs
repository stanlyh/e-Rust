use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, HeaderMap},
};

use crate::{
    error::AppError,
    models::user::Claims,
    state::AppState,
};

/// Extractor de claims del JWT desde el header Authorization
pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = extract_bearer_token(&parts.headers)
            .ok_or(AppError::Unauthorized)?;

        let claims = crate::services::auth_service::AuthService::validate_access_token(
            token,
            &state.config,
        )?;

        Ok(AuthUser(claims))
    }
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<&str> {
    let auth_header = headers.get("Authorization")?.to_str().ok()?;
    auth_header.strip_prefix("Bearer ")
}

/// Guard para roles especificos
pub struct RequireRole(pub Claims);

impl RequireRole {
    pub fn is_admin(&self) -> bool {
        self.0.role == "admin"
    }

    pub fn is_manager_or_above(&self) -> bool {
        matches!(self.0.role.as_str(), "admin" | "manager")
    }
}
