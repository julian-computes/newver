#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error("Failed to parse time")]
    TimeParseFail,

    #[error("Error")]
    Error(String),
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Error::Error(value.to_string())
    }
}