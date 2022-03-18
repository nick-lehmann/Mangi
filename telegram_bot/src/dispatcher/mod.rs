mod callback;
mod command;

pub use callback::CallbackDispatcher;
pub use command::CommandDispatcher;

use std::collections::HashMap;

use frankenstein::Update;

use self::command::Command;

/// Accepts an update from the telegram api and dispatches it to one of the preregistered handlers.
pub struct Dispatcher<'a> {
    commands: HashMap<String, command::Command<'a>>,
    callbacks: HashMap<String, callback::CallbackHandler<'a>>,
}

pub trait GeneralDispatcher<'a>: CommandDispatcher<'a> + CallbackDispatcher<'a> {
    fn dispatch(&self, update: Update) -> Result<(), &'static str>;
}

impl<'a> Dispatcher<'a> {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            callbacks: HashMap::new(),
        }
    }

    pub fn dispatch(&self, update: Update) -> Result<(), &'static str> {
        if (&update.callback_query).is_some() {
            self.handle_callback(update);
            return Ok(());
        }

        if (&update.message).is_some() {
            self.handle_command(update);
            return Ok(());
        }

        Ok(())
    }

    // TODO: Return some form of iterator instead
    pub fn get_commands(&self) -> Vec<Command> {
        self.commands
            .values()
            .map(|command| command.clone())
            .collect()
    }
}
