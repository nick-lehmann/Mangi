use std::fmt::Error;

use frankenstein::Api;
use frankenstein::Message;
use frankenstein::SendMessageParamsBuilder;
use frankenstein::TelegramApi;
use frankenstein::Update;
use telegram_bot::TelegramError;
use telegram_bot::TelegramResult;

pub struct UserSettingsController<'a> {
    api: &'a Api,
}

impl<'a> UserSettingsController<'a> {
    pub fn new(api: &'a frankenstein::Api) -> Self {
        Self { api }
    }

    pub fn list(&self, message: Message) -> TelegramResult<()> {
        let from = message.from.unwrap();

        let send_message_params = SendMessageParamsBuilder::default()
            .chat_id(message.chat.id)
            .text(format!("Hello {}, your preferences are:", from.first_name))
            .build()?;

        self.api.send_message(&send_message_params)?;

        Ok(())
    }
}
