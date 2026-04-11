use axum::{extract::{Query, State}, Json};

use crate::{
    error::AppResult,
    middleware::auth::AuthUser,
    models::activity::{CalendarEvent, CalendarQuery, CalendarResponse},
    repositories::{activity_repo::ActivityRepo, opportunity_repo::OpportunityRepo},
    state::AppState,
};

pub async fn get_calendar(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<CalendarQuery>,
) -> AppResult<Json<CalendarResponse>> {
    let user_id = claims.sub.parse().map_err(|_| crate::error::AppError::Unauthorized)?;
    let is_manager = matches!(claims.role.as_str(), "admin" | "manager");

    let (rows, overdue_count, expiring_opportunities) = tokio::try_join!(
        ActivityRepo::find_by_range(&state.db, user_id, query.from, query.to),
        ActivityRepo::count_overdue(&state.db, user_id),
        OpportunityRepo::find_expiring(&state.db, user_id, is_manager, 7),
    )?;

    let events = rows
        .into_iter()
        .map(|row| {
            let response = crate::models::activity::ActivityResponse::from(row.clone());
            CalendarEvent {
                id: row.id,
                title: row.title,
                start: row.scheduled_start,
                end: row.scheduled_end,
                activity_type: row.r#type,
                status: row.status,
                extended_props: response,
            }
        })
        .collect();

    Ok(Json(CalendarResponse {
        events,
        overdue_count,
        expiring_opportunities,
    }))
}
