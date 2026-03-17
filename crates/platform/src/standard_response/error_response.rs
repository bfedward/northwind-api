use serde::Serialize;
use std::collections::HashMap;

// RFC 7807/9457
#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub r#type: String,
    pub title: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
