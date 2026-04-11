use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{handlers::clients, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/",     get(clients::list).post(clients::create))
        .route("/{id}", get(clients::get_by_id)
                        .put(clients::update)
                        .delete(clients::delete))
}
