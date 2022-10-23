use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("json value of card doesn't contain a {0} field")]
    MissingField(&'static str),
    #[error("json value isn't a card")]
    NotACard,
    #[error("invalid legality '{0}'")]
    InvalidLegality(String),
}
