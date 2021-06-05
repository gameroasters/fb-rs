use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("parse error:{0}")]
    UrlParse(#[from] url::ParseError),

    #[error("utf8 error:{0}")]
    Utf8Parse(#[from] FromUtf8Error),

    #[error("hyper error:{0}")]
    Hyper(#[from] hyper::Error),

    #[error("http error:{0}")]
    Http(#[from] hyper::http::Error),

    #[error("json error:{0}")]
    JsonParse(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
