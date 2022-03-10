pub mod list_canteen;
pub mod list_user_settings;

pub use list_canteen::ListMensaCommand;
pub use list_user_settings::ListUserSettingsCommand;

use frankenstein::{CallbackQuery, Update};

pub trait TelegramCommand {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self, update: Update);
    fn handle_callback(&self, update: &Update) -> Option<()>;
}

pub enum CommandError {
    CannotHandle,
}

pub type CommandResult<T> = Result<T, CommandError>;
