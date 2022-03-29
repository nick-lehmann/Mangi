use serde::{Deserialize, Serialize};
use telegram_bot::TelegramBotError;

use crate::internal::{mensa::models::Diet, users::models::UserType};

use super::errors::MangiTelegramError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TemporaryUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diet: Option<Diet>,
}

impl Into<String> for TemporaryUser {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl TryFrom<Option<String>> for TemporaryUser {
    type Error = MangiTelegramError;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        let data = value.ok_or(MangiTelegramError::Unrecoverable(
            "No callback data received".into(),
        ))?;

        serde_json::from_str(&data).map_err(|e| MangiTelegramError::Unrecoverable(e.to_string()))
    }
}
