use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    NotFound,
    DatabaseError,
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        todo!()
    }
}