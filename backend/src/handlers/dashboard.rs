use axum::{extract::State, Json};

use crate::{
    error::{AppError, AppResult},
    middleware::auth::AuthUser,
    models::dashboard::DashboardReport,
    repositories::dashboard_repo::DashboardRepo,
    state::AppState,
};

pub async fn report(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<DashboardReport>> {
    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    let is_manager = matches!(claims.role.as_str(), "admin" | "manager");

    let (kpis, monthly_sales, funnel, agents) = tokio::try_join!(
        DashboardRepo::kpis(&state.db, user_id, is_manager),
        DashboardRepo::monthly_sales(&state.db, user_id, is_manager),
        DashboardRepo::funnel(&state.db, user_id, is_manager),
        DashboardRepo::agent_stats(&state.db),
    )?;

    Ok(Json(DashboardReport { kpis, monthly_sales, funnel, agents }))
}
