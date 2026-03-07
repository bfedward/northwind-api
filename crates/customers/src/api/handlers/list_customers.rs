use axum::Json;

pub async fn list_customers() -> Json<Vec<String>> {
    Json(vec!["ALFKI".into(), "ANATR".into()])
}
