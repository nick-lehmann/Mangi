pub enum TelegramBotError {
    InputError(String),
    Unrecoverable(String),
    Retry(String),
}

pub trait AsTelegramBotError {
    fn as_error() -> TelegramBotError;
}
