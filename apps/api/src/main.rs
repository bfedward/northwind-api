use axum::Router;
use customers::api::router::router;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/customers", router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
