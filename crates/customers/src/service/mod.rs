use crate::domain::Customer;
use crate::repository;
use platform::database::db_pool::AppState;
use platform::errors::app_error::AppError;

pub async fn list_customers(state: &AppState) -> Result<Vec<Customer>, AppError> {
    repository::get_all_customers(&state.db).await
}
