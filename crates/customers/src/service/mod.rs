use crate::domain::Customer;
use crate::repository;
use platform::database::db_pool::AppState;
use platform::errors::app_error::AppError;
use platform::pagination::cursor::CursorPagination;
use platform::pagination::page::CursorPage;

pub async fn list_customers(
    state: &AppState,
    pagination: CursorPagination,
) -> Result<CursorPage<Customer>, AppError> {
    let page = repository::list_customers(&state.db, pagination).await?;

    Ok(page)
}
