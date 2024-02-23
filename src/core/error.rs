#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),
    #[error("Static error: {0}")]
    Static(&'static str),

    #[error("Failed to connect to DB")]
    DieselConnection(#[from] diesel::result::ConnectionError),

    #[error(transparent)]
    DieselQuery(#[from] diesel::result::Error),
}
