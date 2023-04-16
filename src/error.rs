use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error("Failed to parse time")]
    TimeParseFail,

    #[error(transparent)]
    IgnoreBeforeFail(#[from] ParseIntError),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),

    #[error("Error")]
    NewVerError(String),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::NewVerError(value)
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Error::NewVerError(value.to_string())
    }
}