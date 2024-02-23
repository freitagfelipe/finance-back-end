use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    InternalServerError,
    UserAlreadyExists,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
            Self::UserAlreadyExists => {
                (StatusCode::CONFLICT, "User already exists").into_response()
            }
        }
    }
}
