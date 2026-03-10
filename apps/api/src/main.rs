use axum::{
    Router,
    http::{HeaderName, Method},
};
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use platform::configuration::get_configuration;
use platform::database::db_pool::AppState;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let configuration = get_configuration().expect("Failed to read configuration.");

    let state = Arc::new(AppState::new(&configuration).await);

    let app = app_router(state);

    let address = format!(
        "{}:{}",
        configuration.application.application_host, configuration.application.application_port
    );

    let listener = TcpListener::bind(address)
        .await
        .expect("Could not bind address");

    axum::serve(listener, app)
        .await
        .expect("Could not serve app");
}

pub fn app_router(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/customers", customers::api::router::router())
        .layer(get_cors_layer())
        .with_state(state)
}

fn get_cors_layer() -> CorsLayer {
    let content_type_header = HeaderName::from_static("content-type");

    CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_origin(Any)
        .allow_headers([content_type_header])
}
