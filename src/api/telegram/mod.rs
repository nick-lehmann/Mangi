use std::time::Duration;

use frankenstein::Api;
use frankenstein::BotCommand;
use frankenstein::CallbackQuery;
use frankenstein::GetUpdatesParams;
use frankenstein::GetUpdatesParamsBuilder;
use frankenstein::SetMyCommandsParams;
use frankenstein::SetMyCommandsParamsBuilder;
use frankenstein::TelegramApi;

use commands::{ListMensaCommand, ListUserSettingsCommand};
use frankenstein::Update;
use log::debug;
use log::error;

use crate::internal::mensa::scraper::OpenMensaClient;

use self::commands::TelegramCommand;

pub mod commands;

pub struct TelegramBot<'a> {
    api: &'a Api,
    commands: Vec<&'a dyn TelegramCommand>,
}

impl<'a> TelegramBot<'a> {
    pub fn new(api: &'a Api, commands: Vec<&'a dyn TelegramCommand>) -> Self {
        Self { api, commands }
    }

    fn build_update_params(&self, offset: Option<u32>) -> GetUpdatesParams {
        let mut update_params_builder = GetUpdatesParamsBuilder::default();
        // TODO: Replace magic strings by enum
        update_params_builder
            .allowed_updates(vec!["message".to_string(), "callback_query".to_string()]);

        if let Some(offset_value) = offset {
            update_params_builder.offset(offset_value);
        }

        update_params_builder.build().unwrap()
    }

    fn handle_command(&self, update: Update) -> Option<()> {
        let message = &update.message.as_ref().unwrap();

        let text = message.text.as_ref()?;

        if !text.starts_with('/') {
            return None;
        }

        let command_text = text[1..].to_string();

        let command = self
            .commands
            .iter()
            .find(|command| command.name() == command_text)?;

        command.execute(update);
        Some(())
    }

    fn handle_callback(&self, update: Update) {
        for command in &self.commands {
            debug!("Trying to handle callback with command {}", command.name());
            let handled = command.handle_callback(&update);
            debug!("Command {} did not handle callback", command.name());
            if handled.is_some() {
                return;
            }
        }
    }

    fn execute(&self) {
        let mut offset = None;

        loop {
            let update_params = self.build_update_params(offset);
            let result = self.api.get_updates(&update_params);

            if result.is_err() {
                error!("Failed to get updates: {:?}", result);
                continue;
            }

            let response = result.unwrap();

            for update in response.result {
                let update_id = update.update_id;

                if update.callback_query.is_some() {
                    self.handle_callback(update);
                    offset = Some(update_id + 1);
                    continue;
                }

                self.handle_command(update);
                offset = Some(update_id + 1);
            }
        }
    }
}

fn get_bot_commands(commands: &Vec<&dyn TelegramCommand>) -> Vec<frankenstein::BotCommand> {
    commands
        .iter()
        .map(|command| frankenstein::BotCommand {
            command: command.name().to_string(),
            description: command.description().to_string(),
        })
        .collect()
}

pub fn start_telegram_bot(token: String, open_mensa_client: &OpenMensaClient) {
    let api = Api::new(&token);
    let list_mensa_command = ListMensaCommand::new(&api, open_mensa_client);
    let list_user_settings_command = ListUserSettingsCommand::new(&api);

    let commands: Vec<&dyn TelegramCommand> =
        vec![&list_mensa_command, &list_user_settings_command];

    let bot_commands = SetMyCommandsParamsBuilder::default()
        .commands(get_bot_commands(&commands))
        .build()
        .unwrap();

    api.set_my_commands(&bot_commands)
        .expect("Unable to set bot commands");

    let bot = TelegramBot::new(&api, commands);
    bot.execute();
}
