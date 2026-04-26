use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::{
        common::PaginatedResponse,
        vehicle::{VehicleCreate, VehicleFilters, VehicleResponse, VehicleUpdate},
    },
    repositories::vehicle_repo::VehicleRepo,
    state::AppState,
};

pub async fn list(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Query(filters): Query<VehicleFilters>,
) -> AppResult<Json<PaginatedResponse<VehicleResponse>>> {
    let page = filters.page.unwrap_or(1);
    let per_page = filters.per_page.unwrap_or(20);
    let (rows, total) = VehicleRepo::list(&state.db, &filters).await?;
    let data = rows.into_iter().map(VehicleResponse::from).collect();
    Ok(Json(PaginatedResponse::new(data, total, page, per_page)))
}

pub async fn get_by_id(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<VehicleResponse>> {
    let row = VehicleRepo::find_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Vehiculo".to_string()))?;
    Ok(Json(VehicleResponse::from(row)))
}

pub async fn create(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Json(req): Json<VehicleCreate>,
) -> AppResult<impl IntoResponse> {
    if !matches!(claims.role.as_str(), "admin" | "manager") {
        return Err(AppError::Forbidden);
    }
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let row = VehicleRepo::create(&state.db, &req).await?;
    Ok((StatusCode::CREATED, Json(VehicleResponse::from(row))))
}

pub async fn update(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<VehicleUpdate>,
) -> AppResult<Json<VehicleResponse>> {
    if !matches!(claims.role.as_str(), "admin" | "manager") {
        return Err(AppError::Forbidden);
    }
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let row = VehicleRepo::update(&state.db, id, &req)
        .await?
        .ok_or_else(|| AppError::NotFound("Vehiculo".to_string()))?;
    Ok(Json(VehicleResponse::from(row)))
}

#[derive(Deserialize)]
pub struct AvailabilityBody {
    pub available: bool,
}

pub async fn set_availability(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<AvailabilityBody>,
) -> AppResult<Json<VehicleResponse>> {
    if !matches!(claims.role.as_str(), "admin" | "manager") {
        return Err(AppError::Forbidden);
    }
    let row = VehicleRepo::set_availability(&state.db, id, body.available)
        .await?
        .ok_or_else(|| AppError::NotFound("Vehiculo".to_string()))?;
    Ok(Json(VehicleResponse::from(row)))
}

pub async fn delete(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    if !matches!(claims.role.as_str(), "admin" | "manager") {
        return Err(AppError::Forbidden);
    }
    let deleted = VehicleRepo::delete(&state.db, id).await?;
    if deleted { Ok(StatusCode::NO_CONTENT) } else { Err(AppError::NotFound("Vehiculo".to_string())) }
}
