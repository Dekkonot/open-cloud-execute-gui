use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Serialize)]
#[error("{0}")]
// I'm not pleased with this being a String, but the other option is
// jumping through hoops and life is too short.
pub struct Error(String);

impl Error {
    pub fn new<S: Into<String>>(str: S) -> Self {
        Self(str.into())
    }
}

macro_rules! from_err {
    ($($err:ty),*$(,)?) => {
        $(
            impl From<$err> for Error {
                fn from(value: $err) -> Self {
                    Self(value.to_string())
                }
            }
        )*
    };
}

from_err!(
    reqwest::Error,
    reqwest::header::InvalidHeaderValue,
    serde_json::Error
);
