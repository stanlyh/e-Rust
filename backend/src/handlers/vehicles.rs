use axum::{
    extract::{Multipart, Path, Query, State},
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

pub async fn upload_image(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> AppResult<Json<VehicleResponse>> {
    if !matches!(claims.role.as_str(), "admin" | "manager") {
        return Err(AppError::Forbidden);
    }

    let upload_dir = format!("uploads/vehicles/{}", id);
    tokio::fs::create_dir_all(&upload_dir)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

    let mut saved_url: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::ValidationError(e.to_string()))?
    {
        if field.name() != Some("image") {
            continue;
        }

        let original_name = field.file_name().unwrap_or("image.jpg").to_string();
        let ext = std::path::Path::new(&original_name)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .filter(|e| matches!(e.as_str(), "jpg" | "jpeg" | "png" | "webp"))
            .unwrap_or_else(|| "jpg".to_string());

        let filename = format!("{}.{}", Uuid::new_v4(), ext);
        let filepath = format!("{}/{}", upload_dir, filename);

        let data = field
            .bytes()
            .await
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        tokio::fs::write(&filepath, &data)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        saved_url = Some(format!("/uploads/vehicles/{}/{}", id, filename));
        break;
    }

    let url = saved_url
        .ok_or_else(|| AppError::ValidationError("No se recibio imagen".to_string()))?;

    let row = VehicleRepo::add_image(&state.db, id, &url)
        .await?
        .ok_or_else(|| AppError::NotFound("Vehiculo".to_string()))?;

    Ok(Json(VehicleResponse::from(row)))
}

pub async fn delete_image(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Path((id, filename)): Path<(Uuid, String)>,
) -> AppResult<Json<VehicleResponse>> {
    if !matches!(claims.role.as_str(), "admin" | "manager") {
        return Err(AppError::Forbidden);
    }

    let safe = !filename.contains("..") && !filename.contains('/')
        && filename.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '.');
    if !safe {
        return Err(AppError::ValidationError("Nombre de archivo invalido".to_string()));
    }

    let url = format!("/uploads/vehicles/{}/{}", id, filename);
    let filepath = format!("uploads/vehicles/{}/{}", id, filename);

    let row = VehicleRepo::remove_image(&state.db, id, &url)
        .await?
        .ok_or_else(|| AppError::NotFound("Vehiculo".to_string()))?;

    let _ = tokio::fs::remove_file(&filepath).await;

    Ok(Json(VehicleResponse::from(row)))
}
