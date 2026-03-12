use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use std::fmt;

pub enum EntityKind {
    Customer,
}

impl fmt::Display for EntityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityKind::Customer => write!(f, "customer"),
        }
    }
}

pub enum AppError {
    DatabaseQueryError(sqlx::Error),
    EntityNotFound { id: i64, entity: EntityKind },
    RelatedEntity(String),
    ConstraintError(String),
    InvalidStructArgs(String),
    ValidationFailed(String),
    DuplicateNamedEntity(EntityKind),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::DatabaseQueryError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::EntityNotFound { id, entity } => (
                StatusCode::NOT_FOUND,
                format!("{entity} with id {id} not found!"),
            ),
            Self::RelatedEntity(name) => (
                StatusCode::CONFLICT,
                format!("Cannot delete {name} due to related entity!"),
            ),
            Self::ConstraintError(error) => (
                StatusCode::CONFLICT,
                format!("Database constraint error: {error}"),
            ),
            Self::InvalidStructArgs(error) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid struct args: {error}"),
            ),
            Self::ValidationFailed(message) => (StatusCode::BAD_REQUEST, message),
            Self::DuplicateNamedEntity(error) => (
                StatusCode::CONFLICT,
                format!("{error} with this name already exists!"),
            ),
        };

        // Return a JSON response
        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
