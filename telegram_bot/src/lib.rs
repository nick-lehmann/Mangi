mod bot;
pub use bot::TelegramBot;
mod dispatcher;
pub use dispatcher::{CallbackDispatcher, CommandDispatcher, Dispatcher};
mod errors;
pub use errors::{AsTelegramBotError, TelegramBotError};

pub type TelegramUserID = u64;
