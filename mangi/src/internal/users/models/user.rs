use {frankenstein::ChatId, open_mensa::CanteenID, telegram_bot::TelegramUserID};

use crate::internal::mensa::models::Diet;

use super::user_type::UserType;

pub type UserID = i32;

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub id: UserID,
    pub name: String,
    pub telegram_user_id: Option<TelegramUserID>,
    pub telegram_chat_id: i32,
    pub diet: Diet,
    pub user_type: UserType,
    pub favorite_canteens: Vec<CanteenID>,
}
