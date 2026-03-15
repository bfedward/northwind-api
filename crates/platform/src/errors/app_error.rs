use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;
use validator::ValidationErrorsKind;

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
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

// RFC 7807/9457
#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub r#type: String,
    pub title: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database query failed")]
    DatabaseQuery(#[from] sqlx::Error),

    #[error("{entity} with id {id} not found")]
    EntityNotFound { id: i64, entity: EntityKind },

    #[error("{entity} causes a conflict")]
    Conflict { entity: EntityKind },

    #[error("Validation error")]
    Validation(#[from] ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(err) => {
                let body = ApiErrorResponse {
                    r#type: "validation_error".into(),
                    title: "One or more validation errors occurred".into(),
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    errors: Some(map_validation_errors(err)),
                    detail: None,
                };

                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }

            AppError::EntityNotFound { id, entity } => {
                let body = ApiErrorResponse {
                    r#type: "not_found".into(),
                    title: "Entity not found".into(),
                    status: StatusCode::NOT_FOUND.as_u16(),
                    errors: None,
                    detail: Some(format!("{entity} with id {id} not found")),
                };

                (StatusCode::NOT_FOUND, Json(body)).into_response()
            }

            AppError::Conflict { entity } => {
                let body = ApiErrorResponse {
                    r#type: "conflict".into(),
                    title: "Conflict".into(),
                    status: StatusCode::CONFLICT.as_u16(),
                    errors: None,
                    detail: Some(format!("{entity} causes a conflict")),
                };

                (StatusCode::CONFLICT, Json(body)).into_response()
            }

            AppError::DatabaseQuery(e) => {
                let body = ApiErrorResponse {
                    r#type: "internal_error".into(),
                    title: "Database error".into(),
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    errors: None,
                    detail: Some(e.to_string()),
                };

                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}

fn map_validation_errors(errors: ValidationErrors) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();

    for (field, kind) in errors.errors() {
        if let ValidationErrorsKind::Field(field_errors) = kind {
            let messages = field_errors
                .iter()
                .map(|e| {
                    e.message
                        .clone()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("validation error: {}", e.code))
                })
                .collect();

            result.insert(field.to_string(), messages);
        }
    }

    result
}
