use {
    super::super::enums::{Diet, UserType},
    crate::{
        internal::users::models,
        storage::{schema::users, StorageError},
    },
    diesel::{Insertable, Queryable},
    telegram_bot::TelegramUserID,
};

pub type UserID = i32;

#[derive(Insertable, Queryable, Identifiable, Debug, PartialEq)]
#[table_name = "users"]
pub struct User {
    pub id: UserID,
    pub name: String,
    pub telegram_user_id: UserID,
    pub telegram_chat_id: i32,
    pub user_type: String,
    pub diet: String,
}

impl TryFrom<models::User> for User {
    type Error = StorageError;

    fn try_from(user: models::User) -> Result<User, Self::Error> {
        Ok(User {
            id: user.id,
            name: user.name,
            // TODO: Remove unwrap()
            telegram_user_id: user.telegram_user_id.unwrap() as i32,
            telegram_chat_id: user.telegram_chat_id,
            user_type: user.user_type.into(),
            diet: user.diet.into(),
        })
    }
}

impl TryInto<models::User> for User {
    type Error = StorageError;

    fn try_into(self) -> Result<models::User, Self::Error> {
        Ok(models::User {
            id: self.id,
            name: self.name,
            telegram_user_id: Some(self.telegram_user_id as u64),
            telegram_chat_id: self.telegram_chat_id,
            user_type: self
                .user_type
                .try_into()
                .map_err(|err| StorageError::ValidationError(err))?,
            favorite_canteens: vec![],
            diet: self
                .diet
                .try_into()
                .map_err(|err| StorageError::ValidationError(err))?,
        })
    }
}
