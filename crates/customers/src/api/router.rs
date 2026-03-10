use axum::{Router, routing::get};

use platform::database::db_pool::AppState;

use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route(
        "/",
        get(crate::api::handlers::list_customers::list_customers),
    )
}
