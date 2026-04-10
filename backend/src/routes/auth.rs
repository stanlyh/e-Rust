use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers::auth, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/refresh", post(auth::refresh))
        .route("/register", post(auth::register))
        .route("/me", get(auth::me))
}
