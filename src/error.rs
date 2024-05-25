use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("unknown data store error: {0}")]
    Unknown(String),
}