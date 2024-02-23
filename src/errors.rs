use axum::response::IntoResponse;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    InternalServerError,
    UserAlreadyExists
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
