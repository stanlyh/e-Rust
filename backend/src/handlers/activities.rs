use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::activity::{
        ActivityComplete, ActivityCreate, ActivityReschedule, ActivityResponse, ActivityUpdate,
    },
    repositories::activity_repo::ActivityRepo,
    state::AppState,
};

pub async fn list_upcoming(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<ActivityResponse>>> {
    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    let rows = ActivityRepo::find_upcoming(&state.db, user_id).await?;
    Ok(Json(rows.into_iter().map(ActivityResponse::from).collect()))
}

pub async fn list_overdue(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<ActivityResponse>>> {
    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    let rows = ActivityRepo::find_overdue(&state.db, user_id).await?;
    Ok(Json(rows.into_iter().map(ActivityResponse::from).collect()))
}

pub async fn get_by_id(
    AuthUser(_claims): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<ActivityResponse>> {
    let row = ActivityRepo::find_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Actividad".to_string()))?;
    Ok(Json(ActivityResponse::from(row)))
}

pub async fn create(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Json(req): Json<ActivityCreate>,
) -> AppResult<impl IntoResponse> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    let row = ActivityRepo::create(&state.db, user_id, &req).await?;
    Ok((StatusCode::CREATED, Json(ActivityResponse::from(row))))
}

pub async fn update(
    AuthUser(_claims): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ActivityUpdate>,
) -> AppResult<Json<ActivityResponse>> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let row = ActivityRepo::update(&state.db, id, &req)
        .await?
        .ok_or_else(|| AppError::NotFound("Actividad".to_string()))?;
    Ok(Json(ActivityResponse::from(row)))
}

pub async fn complete(
    AuthUser(_claims): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ActivityComplete>,
) -> AppResult<Json<ActivityResponse>> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let row = ActivityRepo::complete(&state.db, id, &req.outcome, req.next_action.as_deref())
        .await?
        .ok_or_else(|| AppError::NotFound("Actividad".to_string()))?;
    Ok(Json(ActivityResponse::from(row)))
}

pub async fn reschedule(
    AuthUser(_claims): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ActivityReschedule>,
) -> AppResult<Json<ActivityResponse>> {
    let row = ActivityRepo::reschedule(&state.db, id, req.scheduled_start, req.scheduled_end)
        .await?
        .ok_or_else(|| AppError::NotFound("Actividad".to_string()))?;
    Ok(Json(ActivityResponse::from(row)))
}

pub async fn delete(
    AuthUser(_claims): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = ActivityRepo::delete(&state.db, id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound("Actividad".to_string()))
    }
}
