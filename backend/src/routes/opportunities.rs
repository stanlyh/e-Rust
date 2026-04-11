use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{handlers::opportunities, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/pipeline", get(opportunities::pipeline))
        .route("/",         post(opportunities::create))
        .route("/{id}",     get(opportunities::get_by_id))
        .route("/{id}/status",     patch(opportunities::update_status))
        .route("/{id}/close-won",  post(opportunities::close_won))
        .route("/{id}/close-lost", post(opportunities::close_lost))
}
