use axum::{extract::{Query, State}, Json};

use crate::{
    error::AppResult,
    middleware::auth::AuthUser,
    models::activity::{CalendarEvent, CalendarQuery, CalendarResponse},
    repositories::activity_repo::ActivityRepo,
    state::AppState,
};

pub async fn get_calendar(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Query(query): Query<CalendarQuery>,
) -> AppResult<Json<CalendarResponse>> {
    let user_id = claims.sub.parse().map_err(|_| crate::error::AppError::Unauthorized)?;

    let rows = ActivityRepo::find_by_range(&state.db, user_id, query.from, query.to).await?;
    let overdue_count = ActivityRepo::count_overdue(&state.db, user_id).await?;

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

    Ok(Json(CalendarResponse { events, overdue_count }))
}
