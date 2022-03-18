pub mod food;
pub mod user_settings;

pub use food::FoodController;
pub use user_settings::UserSettingsController;

use frankenstein::{CallbackQuery, Update};

pub enum CommandError {
    CannotHandle,
}

pub type CommandResult<T> = Result<T, CommandError>;
