use frankenstein::Message;
use log::info;

use crate::TelegramResult;

use super::Dispatcher;

pub type CommandHandler<'a> = &'a dyn Fn(Message) -> TelegramResult<()>;
// The name of a command without the leading slash
pub type CommandName = String;

#[derive(Clone)]
pub struct Command<'a> {
    pub name: String,
    pub description: String,
    handler: CommandHandler<'a>,
}

impl<'a> Into<frankenstein::BotCommand> for Command<'a> {
    fn into(self) -> frankenstein::BotCommand {
        frankenstein::BotCommand {
            command: self.name.to_owned(),
            description: self.description.to_owned(),
        }
    }
}

pub trait CommandDispatcher<'a> {
    fn command_from_message(&self, message: &Message) -> Option<CommandName>;
    fn register_command(&mut self, name: String, description: String, handler: CommandHandler<'a>);
    fn handle_command(&self, command: String, message: Message);
}

impl<'a> CommandDispatcher<'a> for Dispatcher<'a> {
    fn command_from_message(&self, message: &Message) -> Option<CommandName> {
        let text = message.text.as_ref()?;
        if !text.starts_with('/') {
            return None;
        }

        Some(text[1..].to_string())
    }

    fn handle_command(&self, command: CommandName, message: Message) {
        let command = self
            .commands
            .get(&command)
            .expect(&format!("No command for {} registered", command));

        let _result = (command.handler)(message);
    }

    fn register_command(&mut self, name: String, description: String, handler: CommandHandler<'a>) {
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
