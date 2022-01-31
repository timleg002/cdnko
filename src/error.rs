use actix_web::{
    ResponseError,
    HttpResponse,
    dev::HttpResponseBuilder,
    http::{
        StatusCode, header,
    }
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum CdnkoError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl ResponseError for CdnkoError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadClientData => StatusCode::BAD_REQUEST,
            Self::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}