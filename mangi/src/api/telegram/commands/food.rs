use chrono::NaiveDate;
use frankenstein::Api;
use frankenstein::CallbackQuery;
use frankenstein::EditMessageTextParams;
use frankenstein::EditMessageTextParamsBuilder;
use frankenstein::InlineKeyboardButton;
use frankenstein::InlineKeyboardButtonBuilder;
use frankenstein::InlineKeyboardMarkup;
use frankenstein::Message;
use frankenstein::ReplyMarkup;
use frankenstein::SendMessageParamsBuilder;
use frankenstein::TelegramApi;
use frankenstein::Update;
use log::error;
use open_mensa::OpenMensaClient;
use telegram_bot::TelegramError;
use telegram_bot::TelegramResult;

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

    pub fn list_food_by_canteen(&self, message: Message) -> TelegramResult<()> {
        let inline_keyboard = ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup {
            inline_keyboard: self.get_canteen_buttons(),
        });

        let send_message_params = SendMessageParamsBuilder::default()
            .chat_id(message.chat.id)
            .text("Hier sind deine Mensen")
            .reply_markup(inline_keyboard)
            .build()?;

        self.api.send_message(&send_message_params)?;

        Ok(())
    }

    pub fn handle_callback(&self, callback_query: CallbackQuery) -> TelegramResult<()> {
        let data = callback_query.data.as_ref().unwrap();

        let meals = self
            .open_mensa_client
            .get_meals(data.parse().unwrap(), NaiveDate::from_ymd(2022, 3, 17))
            .unwrap();

        let message = callback_query
            .message
            .as_ref()
            .ok_or(TelegramError::ValueError("No callback given".into()))?;

        let edit_message_params = EditMessageTextParamsBuilder::default()
            .chat_id(message.chat.id)
            .message_id(message.message_id)
            .text("Hier sind deine Gerichte")
            .build()
            .unwrap();

        self.api.edit_message_text(&edit_message_params)?;

        for meal in meals {
            let send_message_params = SendMessageParamsBuilder::default()
                .chat_id(message.chat.id)
                .text(meal.name)
                .build()?;

            self.api.send_message(&send_message_params)?;
        }

        Ok(())
    }
}
