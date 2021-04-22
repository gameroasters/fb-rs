#![allow(clippy::pub_enum_variant_names)]

use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("parse error:{0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("utf8 error:{0}")]
    Utf8Error(#[from] FromUtf8Error),

    #[error("hyper error:{0}")]
    HyperError(#[from] hyper::Error),

    #[error("http error:{0}")]
    HttpError(#[from] hyper::http::Error),

    #[error("json error:{0}")]
    JsonParseError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
