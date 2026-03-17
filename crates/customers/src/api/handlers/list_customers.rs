use axum::extract::{Query, State};
use validator::Validate;

use platform::errors::app_error::AppError;
use platform::pagination::cursor::CursorPagination;
use platform::pagination::page::CursorPage;

use std::sync::Arc;

use crate::api::dto::CustomerResponseDto;
use platform::database::db_pool::AppState;
use platform::standard_response::ApiResponse;

use crate::service;

#[axum::debug_handler]
pub async fn list_customers(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<CursorPagination>,
) -> ApiResponse<CursorPage<CustomerResponseDto>> {
    let result: Result<CursorPage<CustomerResponseDto>, AppError> = async {
        pagination.validate()?;

        let page = service::list_customers(&state, pagination).await?;

        Ok(page.map(CustomerResponseDto::from))
    }
    .await;

    match result {
        Ok(data) => ApiResponse::ok(data),
        Err(err) => err.into(),
    }
}
