mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod state;

use anyhow::Result;
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::Config, db::pool::create_pool, routes::all_routes, state::AppState};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "crm_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;

    tracing::info!("Conectando a la base de datos...");
    let pool = create_pool(&config.database_url).await?;

    tracing::info!("Ejecutando migraciones...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState::new(pool, config.clone());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let make_app = || {
        all_routes(state.clone())
            .layer(TraceLayer::new_for_http())
            .layer(cors.clone())
    };

    let addr_v4: SocketAddr = format!("0.0.0.0:{}", config.port).parse()?;
    let addr_v6: SocketAddr = format!("[::]:{}", config.port).parse()?;

    let listener_v4 = tokio::net::TcpListener::bind(addr_v4).await?;

    // Escucha en IPv6 si el sistema lo soporta (permite que `localhost` resuelva a ::1)
    let listener_v6 = tokio::net::TcpListener::bind(addr_v6).await.ok();

    tracing::info!("Servidor escuchando en http://0.0.0.0:{}", config.port);
    if listener_v6.is_some() {
        tracing::info!("Soporte IPv6 activo en [::]:{}", config.port);
    }

    match listener_v6 {
        Some(v6) => {
            tokio::select! {
                res = axum::serve(listener_v4, make_app()) => res?,
                res = axum::serve(v6, make_app()) => res?,
            }
        }
        None => axum::serve(listener_v4, make_app()).await?,
    }

    Ok(())
}
