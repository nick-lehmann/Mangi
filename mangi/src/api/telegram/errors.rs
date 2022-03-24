use {
    frankenstein::{
        DeleteMessageParamsBuilderError, InlineKeyboardButtonBuilderError,
        SendMessageParamsBuilderError,
    },
    telegram_bot::AsTelegramBotError,
};

#[derive(thiserror::Error, Debug, Clone)]
pub enum MangiTelegramError {
    #[error("{0}")]
    InputError(String),
    #[error("{0}")]
    Retry(String),
    #[error("{0}")]
    Unrecoverable(String),
}

pub type MangiCommandResult = Result<(), MangiTelegramError>;
pub type MangiTelegramResult<T> = Result<T, MangiTelegramError>;

impl AsTelegramBotError for MangiTelegramError {
    fn as_error() -> telegram_bot::TelegramBotError {
        todo!()
    }
}

impl From<ureq::Error> for MangiTelegramError {
    fn from(error: ureq::Error) -> Self {
        // TODO: Differentiate more
        MangiTelegramError::Unrecoverable(error.to_string())
    }
}

// Errors from the telegram api client
// TODO: Move to `telegram_bot` crate
impl From<frankenstein::Error> for MangiTelegramError {
    fn from(_error: frankenstein::Error) -> Self {
        todo!()
    }
}

impl From<SendMessageParamsBuilderError> for MangiTelegramError {
    fn from(error: SendMessageParamsBuilderError) -> Self {
        MangiTelegramError::Unrecoverable(error.to_string())
    }
}

impl From<InlineKeyboardButtonBuilderError> for MangiTelegramError {
    fn from(error: InlineKeyboardButtonBuilderError) -> Self {
        MangiTelegramError::Unrecoverable(error.to_string())
    }
}

impl From<DeleteMessageParamsBuilderError> for MangiTelegramError {
    fn from(error: DeleteMessageParamsBuilderError) -> Self {
        MangiTelegramError::Unrecoverable(error.to_string())
    }
}
