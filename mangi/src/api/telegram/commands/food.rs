use anyhow::Context;
use chrono::NaiveDate;
use frankenstein::{
    Api, CallbackQuery, DeleteMessageParams, DeleteMessageParamsBuilder, EditMessageTextParams,
    EditMessageTextParamsBuilder, InlineKeyboardButton, InlineKeyboardButtonBuilder,
    InlineKeyboardMarkup, Message, ReplyMarkup, SendMessageParamsBuilder, TelegramApi, Update,
};
use log::error;
use open_mensa::{Meal, OpenMensaClient};

use crate::{
    api::telegram::errors::{MangiCommandResult, MangiTelegramError, MangiTelegramResult},
    internal::mensa::models,
};

pub struct FoodController<'a> {
    api: &'a Api,
    open_mensa_client: &'a OpenMensaClient,
}

impl<'a> FoodController<'a> {
    pub fn new(api: &'a frankenstein::Api, open_mensa_client: &'a OpenMensaClient) -> Self {
        Self {
            api,
            open_mensa_client,
        }
    }

    /// Returns the buttons for an inline keyboard where the user chooses a canteen.
    fn get_canteen_buttons(&self) -> MangiTelegramResult<Vec<Vec<InlineKeyboardButton>>> {
        let canteens = self.open_mensa_client.get_canteens()?;
        Ok(canteens
            .iter()
            .map(|canteen| {
                vec![InlineKeyboardButtonBuilder::default()
                    .text(&canteen.name)
                    .callback_data(canteen.id.to_string())
                    .build()
                    .unwrap()]
            })
            .collect())
    }

    /// Sends a list of meals in the specified chat.
    ///
    /// Provides a nice message if the list of meals is empty.
    fn send_meals(&self, meals: &Vec<Meal>, chat_id: i64) -> MangiCommandResult {
        let text = if meals.len() == 0 {
            "Leider gibt es keine Gerichte in der Kantine"
        } else {
            "Hier sind deine Gerichte"
        };

        self.api.send_message(
            &SendMessageParamsBuilder::default()
                .chat_id(chat_id)
                .text(text)
                .build()?,
        )?;

        for meal in meals {
            let send_message_params = SendMessageParamsBuilder::default()
                .chat_id(chat_id)
                .text(&meal.name)
                .build()?;

            self.api.send_message(&send_message_params)?;
        }
        Ok(())
    }

    /// User can choose a canteen to see the food
    pub fn list_food_by_canteen(&self, message: Message) -> MangiCommandResult {
        let inline_keyboard = ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup {
            inline_keyboard: self.get_canteen_buttons()?,
        });

        let send_message_params = SendMessageParamsBuilder::default()
            .chat_id(message.chat.id)
            .text("Wähle eine Mensa ⬇")
            .reply_markup(inline_keyboard)
            .build()?;

        self.api.send_message(&send_message_params)?;

        Ok(())
    }

    pub fn list_food_for_favorite(&self, message: Message, date: NaiveDate) -> MangiCommandResult {
        let favorite_canteen = 4;
        let meals = self.open_mensa_client.get_meals(favorite_canteen, date)?;

        self.send_meals(&meals, message.chat.id)?;

        Ok(())
    }

    /// Show the meals for a user-chosen canteen.
    pub fn canteen_chosen_callback(&self, callback_query: CallbackQuery) -> MangiCommandResult {
        let canteen_id: open_mensa::CanteenID = callback_query
            .data
            .as_ref()
            .ok_or(MangiTelegramError::InputError(
                "No callback data received".into(),
            ))?
            .parse::<open_mensa::CanteenID>()
            .map_err(|e| MangiTelegramError::Unrecoverable(e.to_string()))?;

        let meals = self
            .open_mensa_client
            .get_meals(canteen_id, NaiveDate::from_ymd(2022, 3, 17))?;

        let message = callback_query
            .message
            .as_ref()
            .ok_or(MangiTelegramError::InputError("No message given".into()))?;

        self.api.delete_message(
            &DeleteMessageParamsBuilder::default()
                .chat_id(message.chat.id)
                .message_id(message.message_id)
                .build()?,
        )?;

        self.send_meals(&meals, message.chat.id)?;

        Ok(())
    }
}
