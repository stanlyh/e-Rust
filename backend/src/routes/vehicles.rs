use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

use crate::{handlers::vehicles, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/",     get(vehicles::list).post(vehicles::create))
        .route("/{id}", get(vehicles::get_by_id)
                        .put(vehicles::update)
                        .delete(vehicles::delete))
        .route("/{id}/availability", patch(vehicles::set_availability))
}
