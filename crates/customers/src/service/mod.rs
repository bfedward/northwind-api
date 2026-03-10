use crate::domain::Customer;
use crate::repository;
use platform::database::db_pool::AppState;

pub async fn list_customers(state: &AppState) -> Result<Vec<Customer>, sqlx::Error> {
    repository::get_all_customers(&state.db).await
}
