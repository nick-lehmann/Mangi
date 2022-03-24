use frankenstein::Message;
use log::info;

use crate::errors::AsTelegramBotError;

use super::Dispatcher;

pub type CommandHandler<'a, Error> = &'a dyn Fn(Message) -> Result<(), Error>;
// The name of a command without the leading slash
pub type CommandName = String;

#[derive(Clone)]
pub struct Command<'a, Error: AsTelegramBotError + Clone> {
    pub name: &'a str,
    pub description: &'a str,
    handler: CommandHandler<'a, Error>,
}

impl<'a, Error: AsTelegramBotError + Clone> Into<frankenstein::BotCommand> for Command<'a, Error> {
    fn into(self) -> frankenstein::BotCommand {
        frankenstein::BotCommand {
            command: self.name.to_owned(),
            description: self.description.to_owned(),
        }
    }
}

pub trait CommandDispatcher<'a, Error: AsTelegramBotError> {
    fn command_from_message(&self, message: &Message) -> Option<CommandName>;
    fn register_command(
        &mut self,
        name: &'a str,
        description: &'a str,
        handler: CommandHandler<'a, Error>,
    );
    fn handle_command(&self, command: String, message: Message) -> Result<(), Error>;
}

impl<'a, Error: AsTelegramBotError + Clone> CommandDispatcher<'a, Error> for Dispatcher<'a, Error> {
    fn command_from_message(&self, message: &Message) -> Option<CommandName> {
        let text = message.text.as_ref()?;
        if !text.starts_with('/') {
            return None;
        }

        Some(text[1..].to_string())
    }

    fn handle_command(&self, command: CommandName, message: Message) -> Result<(), Error> {
        let command = self
            .commands
            .get(&command)
            .expect(&format!("No command for {} registered", command));

        (command.handler)(message)?;
        Ok(())
    }

    fn register_command(
        &mut self,
        name: &'a str,
        description: &'a str,
        handler: CommandHandler<'a, Error>,
    ) {
        info!("Register command {}", &name);
        self.commands.insert(
            name.to_owned(),
            Command {
                name,
                description,
                handler: handler,
            },
        );
    }
}
