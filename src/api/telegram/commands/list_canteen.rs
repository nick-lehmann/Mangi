use crate::internal::mensa::scraper::OpenMensaClient;

use super::TelegramCommand;
use chrono::NaiveDate;
use frankenstein::Api;
use frankenstein::InlineKeyboardButton;
use frankenstein::InlineKeyboardButtonBuilder;
use frankenstein::InlineKeyboardMarkup;
use frankenstein::Message;
use frankenstein::ReplyMarkup;
use frankenstein::SendMessageParamsBuilder;
use frankenstein::TelegramApi;
use frankenstein::Update;
use log::error;

pub struct ListMensaCommand<'a> {
    api: &'a Api,
    open_mensa_client: &'a OpenMensaClient,
}

impl<'a> ListMensaCommand<'a> {
    pub fn new(api: &'a frankenstein::Api, open_mensa_client: &'a OpenMensaClient) -> Self {
        Self {
            api,
            open_mensa_client,
        }
    }

    fn get_canteen_buttons(&self) -> Vec<Vec<InlineKeyboardButton>> {
        let canteens = self.open_mensa_client.get_canteens().unwrap();
        canteens
            .iter()
            .map(|canteen| {
                vec![InlineKeyboardButtonBuilder::default()
                    .text(&canteen.name)
                    .callback_data(canteen.id.to_string())
                    .build()
                    .unwrap()]
            })
            .collect()
    }
}

impl<'a> TelegramCommand for ListMensaCommand<'a> {
    fn name(&self) -> &'static str {
        "list"
    }

    fn description(&self) -> &'static str {
        "List all canteens"
    }

    fn execute(&self, update: Update) {
        let message = update.message.unwrap();

        let inline_keyboard = ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup {
            inline_keyboard: self.get_canteen_buttons(),
        });

        let send_message_params = SendMessageParamsBuilder::default()
            .chat_id(message.chat.id)
            .text("Hier sind deine Mensen")
            .reply_markup(inline_keyboard)
            .build()
            .unwrap();

        if let Err(err) = self.api.send_message(&send_message_params) {
            error!("Failed to send message: {:?}", err);
        }
    }

    fn handle_callback(&self, update: &Update) -> Option<()> {
        let callback_query = update.callback_query.as_ref().unwrap();
        let data = callback_query.data.as_ref().unwrap();

        let meals = self
            .open_mensa_client
            .get_meals(data.parse().unwrap(), NaiveDate::from_ymd(2022, 3, 10))
            .unwrap();

        let message: &Message = match callback_query.message.as_ref() {
            Some(text) => text,
            None => {
                println!("Fail. Update is {:#?}", update);
                return None;
            }
        };

        for meal in meals {
            let send_message_params = SendMessageParamsBuilder::default()
                .chat_id(message.chat.id)
                .text(meal.name)
                .build()
                .unwrap();

            if let Err(err) = self.api.send_message(&send_message_params) {
                println!("Failed to send message: {:?}", err);
            }
        }

        Some(())
    }
}
