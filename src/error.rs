use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("unknown data store error: {0}")]
    Unknown(String),
    #[error("Database doesn't exist. If this is OK change create_if_missing to true in Options")]
    DatabaseMissing
}