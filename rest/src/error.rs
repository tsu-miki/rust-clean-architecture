use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("{0}")]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        match self {
            HandlerError::Unknown(error) => {
                tracing::error!("unknown error: {}", error);
                (StatusCode::INTERNAL_SERVER_ERROR, "unknown error").into_response()
            }
        }
    }
}

pub type HandlerResult<T> = Result<T, HandlerError>;
