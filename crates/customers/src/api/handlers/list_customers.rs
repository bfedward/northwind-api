use axum::{
    Json,
    extract::{Query, State},
};
use validator::Validate;

use platform::errors::app_error::AppError;
use platform::pagination::cursor::CursorPagination;
use platform::pagination::page::CursorPage;

use std::sync::Arc;

use crate::api::dto::CustomerResponseDto;
use platform::database::db_pool::AppState;

use crate::service;

pub async fn list_customers(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<CursorPagination>,
) -> Result<Json<CursorPage<CustomerResponseDto>>, AppError> {
    pagination.validate().map_err(|e| AppError::Validation(e))?;

    let page = service::list_customers(&state, pagination).await?;

    Ok(Json(page.map(CustomerResponseDto::from)))
}
