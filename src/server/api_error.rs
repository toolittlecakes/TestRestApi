use actix_web::ResponseError;
use actix_web::http::{StatusCode, header};

// use diesel::result::Error as DieselError;
use base64::DecodeError;
use failure::Fail;

use crate::image::ImageError;
use actix_web::client::{SendRequestError, PayloadError};
use actix_multipart::MultipartError;

/// Used for processing logic errors, invalid requests and for handing errors
/// that don't implement ResponseError trait
///
#[derive(Fail, Debug)]
pub enum ApiError {
    #[fail(display = "Base64 decoding failed")]
    Base64Decoding,
    #[fail(display = "Invalid url. Cannot be localhost")]
    LocalhostUrl,
    #[fail(display = "{}", _0)]
    Image(ImageError),
    #[fail(display = "{}", _0)]
    SendRequest(String),
    #[fail(display = "{}", _0)]
    Payload(String),
    #[fail(display = "{}", _0)]
    Multipart(MultipartError),
    #[fail(display = "{}", _0)]
    IO(std::io::Error)
}

impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> Self {
        ApiError::IO(e)
    }
}

impl From<MultipartError> for ApiError {
    fn from(e: MultipartError) -> Self {
        ApiError::Multipart(e)
    }
}

impl From<PayloadError> for ApiError {
    fn from(e: PayloadError) -> Self {
        ApiError::Payload(format!("{}", e))
    }
}

impl From<SendRequestError> for ApiError {
    fn from(e: SendRequestError) -> Self {
        ApiError::SendRequest(format!("{}", e))
    }
}


impl From<ImageError> for ApiError {
    fn from(e: ImageError) -> Self {
        ApiError::Image(e)
    }
}


impl From<DecodeError> for ApiError {
    fn from(_: DecodeError) -> Self {
        ApiError::Base64Decoding
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        use ApiError::*;
        match *self {
            Base64Decoding | LocalhostUrl => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
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
