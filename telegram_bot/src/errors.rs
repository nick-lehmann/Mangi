pub enum TelegramBotError {
    InputError(anyhow::Error),
    Unrecoverable(anyhow::Error),
    Retry(anyhow::Error),
}

pub trait AsTelegramBotError {
    fn as_error() -> TelegramBotError;
}
