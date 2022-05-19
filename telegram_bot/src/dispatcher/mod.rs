mod callback;
mod command;

pub use callback::CallbackDispatcher;
pub use command::CommandDispatcher;

use std::collections::HashMap;

use frankenstein::Update;

use crate::errors::AsTelegramBotError;

use self::command::Command;

/// Accepts an update from the telegram api and dispatches it to one of the preregistered handlers.
pub struct Dispatcher<'a, Error: AsTelegramBotError + Clone> {
    commands: HashMap<String, command::Command<'a, Error>>,
    callbacks: HashMap<String, callback::CallbackHandler<'a, Error>>,
}

impl<'a, Error: AsTelegramBotError + Clone + Send + Sync + std::error::Error>
    Dispatcher<'a, Error>
{
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            callbacks: HashMap::new(),
        }
    }

    pub fn dispatch(&self, update: Update) -> Result<(), Error> {
        if let Some(callback_query) = update.callback_query {
            self.handle_callback(callback_query)?;
            return Ok(());
        }

        if let Some(message) = update.message {
            if let Some(command_name) = self.command_from_message(&message) {
                self.handle_command(command_name, message)?;
                return Ok(());
            } else {
                // TODO: Handle normal messages
            }
        }

        Ok(())
    }

    // TODO: Return some form of iterator instead
    pub fn get_commands(&self) -> Vec<Command<Error>> {
        self.commands
            .values()
            .map(|command| command.clone())
            .collect()
    }
}
