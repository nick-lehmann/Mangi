pub mod food;
pub mod list_user_settings;

pub use food::FoodCommand;
pub use list_user_settings::ListUserSettingsCommand;

use frankenstein::{CallbackQuery, Update};

use super::TelegramResult;

pub trait TelegramCommand {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self, update: Update) -> TelegramResult<()>;
    fn handle_callback(&self, update: &Update) -> TelegramResult<()>;
}

pub enum CommandError {
    CannotHandle,
}

pub type CommandResult<T> = Result<T, CommandError>;
