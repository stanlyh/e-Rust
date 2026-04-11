use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::{
        common::PaginatedResponse,
        lead::{LeadCreate, LeadFilters, LeadResponse, LeadUpdate},
    },
    repositories::lead_repo::LeadRepo,
    state::AppState,
};

pub async fn list(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Query(filters): Query<LeadFilters>,
) -> AppResult<Json<PaginatedResponse<LeadResponse>>> {
    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    let is_manager = matches!(claims.role.as_str(), "admin" | "manager");
    let page = filters.page.unwrap_or(1);
    let per_page = filters.per_page.unwrap_or(20);
    let (rows, total) = LeadRepo::list(&state.db, &filters, user_id, is_manager).await?;
    let data = rows.into_iter().map(LeadResponse::from).collect();
    Ok(Json(PaginatedResponse::new(data, total, page, per_page)))
}

pub async fn get_by_id(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<LeadResponse>> {
    let row = LeadRepo::find_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Lead".to_string()))?;
    Ok(Json(LeadResponse::from(row)))
}

pub async fn create(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Json(req): Json<LeadCreate>,
) -> AppResult<impl IntoResponse> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    let row = LeadRepo::create(&state.db, &req, user_id).await?;
    Ok((StatusCode::CREATED, Json(LeadResponse::from(row))))
}

pub async fn update(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<LeadUpdate>,
) -> AppResult<Json<LeadResponse>> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let row = LeadRepo::update(&state.db, id, &req)
        .await?
        .ok_or_else(|| AppError::NotFound("Lead".to_string()))?;
    Ok(Json(LeadResponse::from(row)))
}

pub async fn delete(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = LeadRepo::delete(&state.db, id).await?;
    if deleted { Ok(StatusCode::NO_CONTENT) } else { Err(AppError::NotFound("Lead".to_string())) }
}
