use thiserror::Error;

#[derive(Error, Debug)]
pub enum NoemaError {
    #[error("Database Error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Editor Failed with an error")]
    SearchFailed(String),
}

pub type NoemaResult<T = ()> = Result<T, NoemaError>;
