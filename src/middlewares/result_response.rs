use graphul::{http::{response::Response, StatusCode}, IntoResponse};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorApps {
    #[error("The Id most be a positive number")]
    ParseIntError,
    #[error("An unexpected error occurred. Try again.")]
    Unknown,
}


impl IntoResponse for ErrorApps{
    fn into_response(self) -> Response {
        match self {
            ErrorApps::ParseIntError => (StatusCode::BAD_REQUEST, self.to_string()),
            ErrorApps::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        }.into_response()
    }
}

impl From<ErrorApps> for Response{
    fn from(value: ErrorApps) -> Response {
        value.into_response()
    }
}