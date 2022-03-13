use std::fmt::Error;

use crate::api::telegram::TelegramError;
use crate::api::telegram::TelegramResult;

use super::TelegramCommand;
use frankenstein::Api;
use frankenstein::SendMessageParamsBuilder;
use frankenstein::TelegramApi;
use frankenstein::Update;

pub struct ListUserSettingsCommand<'a> {
    api: &'a Api,
}

impl<'a> ListUserSettingsCommand<'a> {
    pub fn new(api: &'a frankenstein::Api) -> Self {
        Self { api }
    }
}

impl<'a> TelegramCommand for ListUserSettingsCommand<'a> {
    fn name(&self) -> &'static str {
        "settings"
    }

    fn description(&self) -> &'static str {
        "Show user settings"
    }

    fn execute(&self, update: Update) -> TelegramResult<()> {
        let message = update.message.unwrap();

        let from = message.from.unwrap();

        let send_message_params = SendMessageParamsBuilder::default()
            .chat_id(message.chat.id)
            .text(format!("Hello {}, your preferences are:", from.first_name))
            .build()?;

        self.api.send_message(&send_message_params)?;

        Ok(())
    }

    fn handle_callback(&self, _callback_query: &Update) -> TelegramResult<()> {
        Err(TelegramError::NothingDone)
    }
}