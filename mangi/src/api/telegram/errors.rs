use frankenstein::{
    api::Error as FrankensteinError, ErrorResponse, HttpError, SendMessageParamsBuilderError,
};

pub type TelegramResult<T> = Result<T, TelegramError>;

pub enum TelegramError {
    ValueError(String),
    HttpError(HttpError),
    ApiError(ErrorResponse),
    DecodeError(String),
    EncodeError(String),
    NothingDone,
}

impl From<FrankensteinError> for TelegramError {
    fn from(error: FrankensteinError) -> Self {
        match error {
            FrankensteinError::HttpError(err) => TelegramError::HttpError(err),
            FrankensteinError::ApiError(error_response) => TelegramError::ApiError(error_response),
            FrankensteinError::DecodeError(_) => todo!(),
            FrankensteinError::EncodeError(_) => todo!(),
        }
    }
}

impl From<SendMessageParamsBuilderError> for TelegramError {
    fn from(error: SendMessageParamsBuilderError) -> Self {
        TelegramError::ValueError(error.to_string())
    }
}
