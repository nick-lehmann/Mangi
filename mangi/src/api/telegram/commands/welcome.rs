use std::{collections::HashMap, sync::Mutex};

use frankenstein::{
    Api, CallbackQuery, InlineKeyboardButton, InlineKeyboardButtonBuilder, InlineKeyboardMarkup,
    Message, ReplyMarkup, SendMessageParamsBuilder, TelegramApi,
};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use telegram_bot::TelegramUserID;

use crate::{
    api::telegram::{
        errors::{MangiCommandResult, MangiTelegramError, MangiTelegramResult},
        helpers::TemporaryUser,
        messages::mangi_messages,
    },
    internal::{
        mensa::{models as mensa_models, models::Diet},
        users::{
            models as user_models, models::UserType, service::UserService, storage::UserStorage,
        },
    },
};

pub struct WelcomeController<'a, Service: UserService> {
    pub api: &'a Api,
    pub user_service: &'a Service,
}

impl<'a, Service: UserService> WelcomeController<'a, Service> {
    pub fn new(api: &'a Api, user_service: &'a Service) -> Self {
        Self { api, user_service }
    }

    /// Welcome a new user
    ///
    /// - Say hello to new user
    /// - Tell user what the bot can do
    pub fn start(&self, message: Message) -> MangiCommandResult {
        self.api.send_message(
            &SendMessageParamsBuilder::default()
                .chat_id(message.chat.id)
                .text(mangi_messages.welcome)
                .build()?,
        )?;

        self.show_user_type_select(message)
    }

    /// Returns a keyboard with all user types
    fn user_type_keyboard(&self) -> MangiTelegramResult<InlineKeyboardMarkup> {
        let mut buttons: Vec<InlineKeyboardButton> = vec![];

        for user_type in vec![UserType::Student, UserType::Employee] {
            let text: String = user_type.clone().into();
            let user: String = TemporaryUser {
                user_type: Some(user_type),
                diet: None,
            }
            .into();
            let button = InlineKeyboardButtonBuilder::default()
                .text(&text)
                .callback_data(user)
                .build()?;

            buttons.push(button);
        }

        Ok(InlineKeyboardMarkup {
            inline_keyboard: vec![buttons],
        })
    }

    /// Show user keyboard to input their type (student, employee, ...)
    fn show_user_type_select(&self, message: Message) -> MangiCommandResult {
        self.api.send_message(
            &SendMessageParamsBuilder::default()
                .chat_id(message.chat.id)
                .text(mangi_messages.user_type)
                .reply_markup(ReplyMarkup::InlineKeyboardMarkup(
                    self.user_type_keyboard()?,
                ))
                .build()?,
        )?;

        Ok(())
    }

    /// React when the user selects his or her type
    pub fn accept_user_type_select(&self, callback: CallbackQuery) -> MangiTelegramResult<()> {
        let temporary_user: TemporaryUser = callback.data.try_into()?;

        debug!("A user has selected his or her type: {:?}", &temporary_user);

        self.show_diet_keyboard(
            callback.message.ok_or(MangiTelegramError::Unrecoverable(
                "Callback has no message to it".into(),
            ))?,
            temporary_user,
        )
    }

    /// Returns an inline keyboard with all avaiable diets.
    fn diet_keyboard(&self, temporay_user: TemporaryUser) -> InlineKeyboardMarkup {
        let buttons: Vec<InlineKeyboardButton> =
            vec![Diet::Omnivore, Diet::Vegetarian, Diet::Vegan]
                .into_iter()
                .map(|diet| {
                    let diet_text: String = diet.clone().into();
                    let mut user = temporay_user.clone();
                    user.diet = Some(diet);
                    let payload: String = user.into();

                    debug!("Sending diet with payload {}", payload);

                    InlineKeyboardButtonBuilder::default()
                        .text(&diet_text)
                        .callback_data(&payload)
                        .build()
                        .unwrap()
                })
                .collect();

        InlineKeyboardMarkup {
            inline_keyboard: vec![buttons],
        }
    }

    /// Show user an inline keyboard with all diets
    fn show_diet_keyboard(
        &self,
        message: Message,
        temporary_user: TemporaryUser,
    ) -> MangiCommandResult {
        self.api.send_message(
            &SendMessageParamsBuilder::default()
                .chat_id(message.chat.id)
                .text(mangi_messages.diet)
                .reply_markup(ReplyMarkup::InlineKeyboardMarkup(
                    self.diet_keyboard(temporary_user),
                ))
                .build()?,
        )?;

        Ok(())
    }

    pub fn accept_diet_select(&self, callback_query: CallbackQuery) -> MangiTelegramResult<()> {
        let temporary_user: TemporaryUser = callback_query.data.try_into()?;

        let message = callback_query
            .message
            .ok_or(MangiTelegramError::Unrecoverable(
                "Callback has no message".into(),
            ))?;

        info!("Accepted diet select: {:?}", &message);

        self.finish_registration(message.chat.id, message.chat, temporary_user)
    }

    fn finish_registration(
        &self,
        chat_id: i64,
        telegram_chat: frankenstein::Chat,
        temporary_user: TemporaryUser,
    ) -> MangiTelegramResult<()> {
        info!("Finished registration {:?}", &telegram_chat);

        let user = user_models::User {
            id: 1,
            name: telegram_chat.first_name.unwrap(),
            telegram_user_id: Some(telegram_chat.id as u64),
            telegram_chat_id: chat_id as i32,
            diet: temporary_user.diet.ok_or(MangiTelegramError::InputError(
                "Missing diet in registration callback".into(),
            ))?,
            user_type: temporary_user
                .user_type
                .ok_or(MangiTelegramError::InputError(
                    "Missing user_type in registration callback".into(),
                ))?,
            favorite_canteens: vec![],
        };

        let user = self.user_service.create_user(user);

        let user_type_string: String = user.user_type.into();
        let diet_string: String = user.diet.into();
        self.api.send_message(
            &SendMessageParamsBuilder::default()
                .chat_id(chat_id)
                .text(format!(
                    "Danke für die Infos. Du bist {}, ein {} und isst {}. Viel Spaß mit Mangi",
                    &user.name, user_type_string, diet_string
                ))
                .build()?,
        )?;

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temporary_user_serialisation() {
        let user = TemporaryUser {
            user_type: Some(UserType::Student),
            diet: None,
        };

        let payload: String = user.clone().into();

        let callback_data: TemporaryUser = Some(payload).try_into().unwrap();

        assert_eq!(callback_data, user)
    }
}
