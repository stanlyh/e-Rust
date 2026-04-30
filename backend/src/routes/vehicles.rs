use axum::{
    extract::DefaultBodyLimit,
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
        .route(
            "/{id}/images",
            post(vehicles::upload_image).layer(DefaultBodyLimit::max(10 * 1024 * 1024)),
        )
        .route("/{id}/images/{filename}", delete(vehicles::delete_image))
}
