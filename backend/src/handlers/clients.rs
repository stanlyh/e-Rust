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
        client::{ClientCreate, ClientFilters, ClientResponse, ClientUpdate},
        common::PaginatedResponse,
    },
    repositories::client_repo::ClientRepo,
    state::AppState,
};

pub async fn list(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Query(filters): Query<ClientFilters>,
) -> AppResult<Json<PaginatedResponse<ClientResponse>>> {
    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    let is_manager = matches!(claims.role.as_str(), "admin" | "manager");
    let page = filters.page.unwrap_or(1);
    let per_page = filters.per_page.unwrap_or(20);

    let (rows, total) = ClientRepo::list(&state.db, &filters, user_id, is_manager).await?;
    let data = rows.into_iter().map(ClientResponse::from).collect();
    Ok(Json(PaginatedResponse::new(data, total, page, per_page)))
}

pub async fn get_by_id(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<ClientResponse>> {
    let row = ClientRepo::find_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Cliente".to_string()))?;
    Ok(Json(ClientResponse::from(row)))
}

pub async fn create(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Json(req): Json<ClientCreate>,
) -> AppResult<impl IntoResponse> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    let row = ClientRepo::create(&state.db, &req, user_id).await?;
    Ok((StatusCode::CREATED, Json(ClientResponse::from(row))))
}

pub async fn update(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ClientUpdate>,
) -> AppResult<Json<ClientResponse>> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let row = ClientRepo::update(&state.db, id, &req)
        .await?
        .ok_or_else(|| AppError::NotFound("Cliente".to_string()))?;
    Ok(Json(ClientResponse::from(row)))
}

pub async fn delete(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = ClientRepo::delete(&state.db, id).await?;
    if deleted { Ok(StatusCode::NO_CONTENT) } else { Err(AppError::NotFound("Cliente".to_string())) }
}
