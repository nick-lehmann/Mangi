use std::fmt::Error;

use frankenstein::{Api, Message, SendMessageParamsBuilder, TelegramApi, Update};

use crate::api::telegram::errors::{MangiCommandResult, MangiTelegramError};

pub struct UserSettingsController<'a> {
    api: &'a Api,
}

impl<'a> UserSettingsController<'a> {
    pub fn new(api: &'a frankenstein::Api) -> Self {
        Self { api }
    }

    pub fn list(&self, message: Message) -> MangiCommandResult {
        let from = message.from.ok_or(MangiTelegramError::Unrecoverable(
            "Message has no user attached to it".into(),
        ))?;

        self.api.send_message(
            &SendMessageParamsBuilder::default()
                .chat_id(message.chat.id)
                .text(format!("Hello {}, your preferences are:", from.first_name))
                .build()?,
        )?;

        Ok(())
    }
}
