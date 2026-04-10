pub mod auth;

use axum::Router;
use crate::state::AppState;

pub fn all_routes(state: AppState) -> Router {
    Router::new()
        .nest("/api/auth", auth::router())
        .with_state(state)
}
