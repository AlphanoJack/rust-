use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFailed,
    BadRequest(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("--> {:<12} - {self:?}", "INTO_RES");

        match self {
            Error::LoginFailed => {
                (StatusCode::UNAUTHORIZED, "Login failed").into_response()
            }
            Error::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
        }
    }
}