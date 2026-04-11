pub mod activities;
pub mod auth;
pub mod clients;
pub mod leads;
pub mod vehicles;

use axum::Router;
use crate::{handlers::calendar, state::AppState};

pub fn all_routes(state: AppState) -> Router {
    Router::new()
        .nest("/api/auth",        auth::router())
        .nest("/api/calendar",    axum::Router::new()
            .route("/", axum::routing::get(calendar::get_calendar)))
        .nest("/api/activities",  activities::router())
        .nest("/api/clients",     clients::router())
        .nest("/api/vehicles",    vehicles::router())
        .nest("/api/leads",       leads::router())
        .with_state(state)
}
