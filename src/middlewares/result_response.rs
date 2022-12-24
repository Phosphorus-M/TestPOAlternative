use graphul::{http::{response::Response, StatusCode}, IntoResponse};

use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum ErrorApps {
    #[error("Some fields are not available to be edited")]
    FieldsAreNotAvailableToUpdate,
    #[error("The payload has invalid data")]
    JsonRejection,
    #[error("The Id is missing and is required to follow")]
    IdIsRequired,
    #[error("The Id most be a positive number")]
    ParseIntError,
    #[error("An unexpected error occurred. Try again.")]
    Unknown,
}


impl IntoResponse for ErrorApps{
    fn into_response(self) -> Response {
        match self {
            ErrorApps::IdIsRequired => (StatusCode::BAD_REQUEST, self.to_string()),
            ErrorApps::FieldsAreNotAvailableToUpdate => (StatusCode::BAD_REQUEST, self.to_string()),
            ErrorApps::JsonRejection => (StatusCode::BAD_REQUEST, self.to_string()),
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