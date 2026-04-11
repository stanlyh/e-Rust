pub mod activities;
pub mod auth;
pub mod clients;
pub mod leads;
pub mod opportunities;
pub mod vehicles;

use axum::{routing::get, Router};
use crate::{handlers::{calendar, dashboard}, state::AppState};

pub fn all_routes(state: AppState) -> Router {
    Router::new()
        .nest("/api/auth",          auth::router())
        .nest("/api/calendar",      Router::new().route("/", get(calendar::get_calendar)))
        .nest("/api/activities",    activities::router())
        .nest("/api/clients",       clients::router())
        .nest("/api/vehicles",      vehicles::router())
        .nest("/api/leads",         leads::router())
        .nest("/api/opportunities", opportunities::router())
        .nest("/api/dashboard",     Router::new().route("/", get(dashboard::report)))
        .with_state(state)
}
