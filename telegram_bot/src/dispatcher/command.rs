use frankenstein::Update;
use log::info;

use crate::TelegramResult;

use super::Dispatcher;

pub type CommandHandler<'a> = &'a dyn Fn(Update) -> TelegramResult<()>;

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
    fn register_command(
        &mut self,
        name: String,
        description: String,
        handler: &'a dyn Fn(Update) -> TelegramResult<()>,
    );
    fn handle_command(&self, update: Update);
}

impl<'a> CommandDispatcher<'a> for Dispatcher<'a> {
    fn handle_command(&self, update: Update) {
        let message = &update.message.as_ref().unwrap();
        let text = message.text.as_ref().unwrap();

        if !text.starts_with('/') {
            return;
        }

        let command_text = text[1..].to_string();

        let command = self
            .commands
            .get(&command_text)
            .expect(&format!("No command for {} registered", command_text));

        let _result = (command.handler)(update);
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
