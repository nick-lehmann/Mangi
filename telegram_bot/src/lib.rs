mod bot;
pub use bot::TelegramBot;
mod dispatcher;
pub use dispatcher::{CallbackDispatcher, CommandDispatcher, Dispatcher};
mod errors;
pub use errors::{TelegramError, TelegramResult};

pub type TelegramUserID = u64;
