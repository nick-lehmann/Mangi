use std::num::ParseFloatError;

#[derive(Debug)]
pub enum StorageError {
    ParseError,
    ConnectionError(r2d2::Error),
    DatabaseError(diesel::result::Error),
}

impl From<ParseFloatError> for StorageError {
    fn from(error: ParseFloatError) -> Self {
        return StorageError::ParseError;
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
