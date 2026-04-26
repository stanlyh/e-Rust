use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("No autenticado")]
    Unauthorized,

    #[error("Sin permisos para esta accion")]
    Forbidden,

    #[error("{0} no encontrado")]
    NotFound(String),

    #[error("Datos invalidos: {0}")]
    ValidationError(String),

    #[error("Conflicto: {0}")]
    Conflict(String),

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Error interno del servidor")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, format!("{msg} no encontrado")),
            AppError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::Database(e) => {
                tracing::error!("Database error: {e}");
                // Unique constraint violation (PG code 23505)
                if let sqlx::Error::Database(db_err) = e {
                    if db_err.code().as_deref() == Some("23505") {
                        let detail = db_err.message();
                        let field = if detail.contains("stock_number") {
                            "El numero de stock ya esta registrado"
                        } else if detail.contains("vin") {
                            "El VIN ya esta registrado"
                        } else if detail.contains("email") {
                            "El email ya esta registrado"
                        } else {
                            "Ya existe un registro con esos datos"
                        };
                        return (StatusCode::CONFLICT, Json(json!({ "error": field }))).into_response();
                    }
                }
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error de base de datos".to_string(),
                )
            }
            AppError::Internal(e) => {
                tracing::error!("Internal error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error interno del servidor".to_string(),
                )
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
