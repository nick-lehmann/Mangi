use std::num::ParseFloatError;

use crate::helpers::error_chain_fmt;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(thiserror::Error)]
pub enum StorageError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// impl From<ParseFloatError> for StorageError {
//     fn from(_error: ParseFloatError) -> Self {
//         StorageError::ParseError("Unable to parse float".to_string())
//     }
// }

impl From<diesel::result::Error> for StorageError {
    fn from(error: diesel::result::Error) -> Self {
        StorageError::UnexpectedError(error.into())
    }
}

impl From<r2d2::Error> for StorageError {
    fn from(error: r2d2::Error) -> Self {
        StorageError::UnexpectedError(error.into())
    }
}
