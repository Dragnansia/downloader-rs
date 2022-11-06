use reqwest::{StatusCode, Url};
use std::io::ErrorKind;

pub enum Error {
    Url(Option<Url>),
    Status(Option<StatusCode>),
    FileCreation(ErrorKind),
    GetTotalSize,
    Unknown,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_redirect() || err.is_request() {
            Self::Url(err.url().cloned())
        } else if err.is_status() {
            Self::Status(err.status().clone())
        } else {
            Self::Unknown
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::FileCreation(err.kind())
    }
}
