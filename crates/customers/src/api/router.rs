use axum::{Router, routing::get};

use crate::api::handlers::list_customers::list_customers;

pub fn router() -> Router {
    Router::new().route("/", get(list_customers))
}
