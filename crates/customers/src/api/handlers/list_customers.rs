use axum::Json;
use axum::extract::State;

use platform::conversion::dto::IntoVecDto;
use platform::errors::app_error::AppError;

use std::sync::Arc;

use crate::api::dto::CustomerResponseDto;
use platform::database::db_pool::AppState;

use crate::service;

pub async fn list_customers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CustomerResponseDto>>, AppError> {
    Ok(Json(service::list_customers(&state).await?.to_dto()))
}
