use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use validator::Validate;

use crate::{
    error::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::user::{LoginRequest, RegisterRequest, UserResponse},
    services::auth_service::AuthService,
    state::AppState,
};

const REFRESH_TOKEN_COOKIE: &str = "refresh_token";

pub async fn register(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(req): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    if claims.role != "admin" {
        return Err(AppError::Forbidden);
    }

    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = AuthService::register(&state, req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let (auth_response, refresh_token) = AuthService::login(&state, req).await?;

    let cookie = format!(
        "{}={}; HttpOnly; SameSite=Strict; Path=/; Max-Age={}",
        REFRESH_TOKEN_COOKIE,
        refresh_token,
        state.config.jwt_refresh_expiry_days * 24 * 3600
    );

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    Ok((StatusCode::OK, headers, Json(auth_response)))
}

pub async fn refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> AppResult<impl IntoResponse> {
    let refresh_token = extract_cookie(&headers, REFRESH_TOKEN_COOKIE)
        .ok_or(AppError::Unauthorized)?;

    let (auth_response, new_refresh_token) = AuthService::refresh(&state, refresh_token).await?;

    let cookie = format!(
        "{}={}; HttpOnly; SameSite=Strict; Path=/; Max-Age={}",
        REFRESH_TOKEN_COOKIE,
        new_refresh_token,
        state.config.jwt_refresh_expiry_days * 24 * 3600
    );

    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    Ok((StatusCode::OK, resp_headers, Json(auth_response)))
}

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> AppResult<impl IntoResponse> {
    if let Some(refresh_token) = extract_cookie(&headers, REFRESH_TOKEN_COOKIE) {
        AuthService::logout(&state, refresh_token).await?;
    }

    let clear_cookie = format!(
        "{}=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0",
        REFRESH_TOKEN_COOKIE
    );

    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::SET_COOKIE, clear_cookie.parse().unwrap());

    Ok((StatusCode::OK, resp_headers, Json(serde_json::json!({ "message": "Sesion cerrada" }))))
}

pub async fn me(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<UserResponse>> {
    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    let user = crate::repositories::user_repo::UserRepo::find_by_id(&state.db, user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    Ok(Json(UserResponse::from(user)))
}

fn extract_cookie<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    cookie_header
        .split(';')
        .find(|c| c.trim().starts_with(name))
        .and_then(|c| c.trim().strip_prefix(&format!("{}=", name)))
}
