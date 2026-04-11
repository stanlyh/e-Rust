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
    models::opportunity::{
        CloseWon, CloseLost, OpportunityCreate, OpportunityResponse,
        PipelineResponse, StatusUpdate,
    },
    repositories::opportunity_repo::OpportunityRepo,
    state::AppState,
};

pub async fn pipeline(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<PipelineResponse>> {
    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    let is_manager = matches!(claims.role.as_str(), "admin" | "manager");
    let result = OpportunityRepo::pipeline(&state.db, user_id, is_manager).await?;
    Ok(Json(result))
}

pub async fn get_by_id(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<OpportunityResponse>> {
    let row = OpportunityRepo::find_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Oportunidad".to_string()))?;
    Ok(Json(OpportunityResponse::from(row)))
}

pub async fn create(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Json(req): Json<OpportunityCreate>,
) -> AppResult<impl IntoResponse> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    let row = OpportunityRepo::create(&state.db, &req, user_id).await?;
    Ok((StatusCode::CREATED, Json(OpportunityResponse::from(row))))
}

pub async fn update_status(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<StatusUpdate>,
) -> AppResult<Json<OpportunityResponse>> {
    let row = OpportunityRepo::update_status(&state.db, id, &body.status)
        .await?
        .ok_or_else(|| AppError::NotFound("Oportunidad".to_string()))?;
    Ok(Json(OpportunityResponse::from(row)))
}

pub async fn close_won(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CloseWon>,
) -> AppResult<Json<OpportunityResponse>> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let row = OpportunityRepo::close_won(&state.db, id, &req)
        .await?
        .ok_or_else(|| AppError::NotFound("Oportunidad".to_string()))?;
    Ok(Json(OpportunityResponse::from(row)))
}

pub async fn close_lost(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CloseLost>,
) -> AppResult<Json<OpportunityResponse>> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let row = OpportunityRepo::close_lost(&state.db, id, &req)
        .await?
        .ok_or_else(|| AppError::NotFound("Oportunidad".to_string()))?;
    Ok(Json(OpportunityResponse::from(row)))
}
