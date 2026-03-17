pub mod error_response;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::standard_response::error_response::ApiErrorResponse;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success {
        status: u16,
        trace_id: String,
        data: T,
    },
    Error(ApiErrorResponse),
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self::Success {
            status: 200,
            trace_id: "static-trace-id".into(),
            data,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Success { status, .. } => {
                let status = StatusCode::from_u16(status).unwrap_or(StatusCode::OK);

                (status, Json(self)).into_response()
            }
            ApiResponse::Error(err) => {
                let status =
                    StatusCode::from_u16(err.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

                (status, Json(err)).into_response()
            }
        }
    }
}
