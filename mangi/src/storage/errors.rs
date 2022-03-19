use std::num::ParseFloatError;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug)]
pub enum StorageError {
    ParseError(String),
    ConnectionError(r2d2::Error),
    DatabaseError(diesel::result::Error),
}

impl From<ParseFloatError> for StorageError {
    fn from(_error: ParseFloatError) -> Self {
        StorageError::ParseError("Unable to parse float".to_string())
    }
}

impl From<diesel::result::Error> for StorageError {
    fn from(error: diesel::result::Error) -> Self {
        StorageError::DatabaseError(error)
    }
}

impl From<r2d2::Error> for StorageError {
    fn from(error: r2d2::Error) -> Self {
        StorageError::ConnectionError(error)
    }
}
