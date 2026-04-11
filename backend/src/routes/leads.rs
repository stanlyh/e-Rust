use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{handlers::leads, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/",     get(leads::list).post(leads::create))
        .route("/{id}", get(leads::get_by_id)
                        .put(leads::update)
                        .delete(leads::delete))
}
