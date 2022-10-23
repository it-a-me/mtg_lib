use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error parsing json\n\t{0}")]
    JsonParse(#[from] json::Error),
    #[error("Io error\n\t{0}")]
    Io(#[from] std::io::Error),
    #[error("invalid utf8\n\t{0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}
