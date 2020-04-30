use actix_web::ResponseError;
use actix_web::http::{StatusCode, header};

// use diesel::result::Error as DieselError;
use base64::DecodeError;
use failure::Fail;

/// Used for processing logic errors, invalid requests and for handing errors
/// that don't implement ResponseError trait
///
#[derive(Fail, Debug, PartialEq)]
pub enum ApiError {
    #[fail(display = "Name already exists")]
    NameExists,
    #[fail(display = "Base64 decoding failed")]
    Base64Decoding,
    #[fail(display = "Preview generation failed")]
    PreviewGeneration,
    #[fail(display = "Unsupported image format")]
    UnsupportedImageFormat,
    #[fail(display = "Invalid url. Cannot be localhost")]
    LocalhostUrl,
    #[fail(display = "File system error: {}", _0)]
    FileSystemError(String),
}

impl From<DecodeError> for ApiError {
    fn from(_: DecodeError) -> Self {
        ApiError::Base64Decoding
    }
}


impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> Self {
        ApiError::FileSystemError(e.to_string())
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        use ApiError::*;
        match *self {
            PreviewGeneration | FileSystemError(_)
            => StatusCode::INTERNAL_SERVER_ERROR,

            Base64Decoding | NameExists | UnsupportedImageFormat | LocalhostUrl => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let mut resp = actix_web::HttpResponse::build(self.status_code());
        resp.header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/plain; charset=utf-8"),
        );

        match self.status_code() {
            StatusCode::INTERNAL_SERVER_ERROR => resp.finish(),
            _ => resp.body(format!("{}", self))
        }
    }
}
