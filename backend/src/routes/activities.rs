use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

use crate::{handlers::activities, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/upcoming", get(activities::list_upcoming))
        .route("/overdue",  get(activities::list_overdue))
        .route("/",         post(activities::create))
        .route("/{id}",     get(activities::get_by_id))
        .route("/{id}",     put(activities::update))
        .route("/{id}",     delete(activities::delete))
        .route("/{id}/complete",   patch(activities::complete))
        .route("/{id}/reschedule", patch(activities::reschedule))
}
