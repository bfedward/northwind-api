use thiserror::Error;
use validator::ValidationErrors;
use validator::ValidationErrorsKind;

use std::collections::HashMap;
use std::fmt;

use crate::standard_response::ApiResponse;
use crate::standard_response::error_response::ApiErrorResponse;

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

impl<T> From<AppError> for ApiResponse<T> {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Validation(e) => ApiResponse::Error(ApiErrorResponse {
                r#type: "validation".into(),
                title: "Validation error".into(),
                status: 400,
                errors: Some(map_validation_errors(e)),
                detail: None,
            }),

            AppError::EntityNotFound { id, entity } => ApiResponse::Error(ApiErrorResponse {
                r#type: "not-found".into(),
                title: "Entity not found".into(),
                status: 404,
                errors: None,
                detail: Some(format!("{entity} with id {id} not found")),
            }),

            AppError::Conflict { entity } => ApiResponse::Error(ApiErrorResponse {
                r#type: "conflict".into(),
                title: "Conflict".into(),
                status: 409,
                errors: None,
                detail: Some(format!("{entity} causes a conflict")),
            }),

            AppError::DatabaseQuery(e) => ApiResponse::Error(ApiErrorResponse {
                r#type: "internal-error".into(),
                title: "Internal error".into(),
                status: 500,
                errors: None,
                detail: Some(e.to_string()),
            }),
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
