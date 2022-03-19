use frankenstein::ChatId;
use open_mensa::CanteenID;
use telegram_bot::TelegramUserID;

use crate::internal::mensa::models::Diet;

use super::user_type::UserType;

pub type UserID = i32;

pub struct User {
    pub id: UserID,
    pub name: String,
    pub telegram_user_id: Option<TelegramUserID>,
    pub telegram_chat_id: i64,
    pub user_type: UserType,
    pub favorite_canteens: Vec<CanteenID>,
    pub diet: Diet,
}
