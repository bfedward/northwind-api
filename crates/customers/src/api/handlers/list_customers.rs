use axum::Json;
use axum::extract::State;

use std::sync::Arc;

use platform::database::db_pool::AppState;

use crate::service;

pub async fn list_customers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<crate::domain::Customer>>, axum::http::StatusCode> {
    let customers = service::list_customers(&state)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(customers))
}
