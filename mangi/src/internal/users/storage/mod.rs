use telegram_bot::TelegramUserID;

use crate::storage::StorageResult;

use super::models;
use crate::internal::mensa::models as mensa_models;

pub mod enums;
pub mod postgres;

pub trait UserStorage {
    fn list_users(&self) -> StorageResult<Vec<models::User>>;
    fn get_user(&self, id: models::UserID) -> StorageResult<models::User>;
    fn get_telegram_user(&self, id: TelegramUserID) -> StorageResult<models::User>;
    fn create_user(&self, user: models::User) -> StorageResult<models::User>;
    fn update_user(&self, user: models::User) -> StorageResult<models::User>;
    fn delete_user(&self, user_id: models::UserID) -> StorageResult<()>;

    fn get_favorites(&self, user_id: models::UserID)
        -> StorageResult<Vec<mensa_models::CanteenID>>;
    fn add_favorite(
        &self,
        user_id: models::UserID,
        canteen_id: mensa_models::CanteenID,
    ) -> StorageResult<()>;
    fn remove_favorite(
        &self,
        user_id: models::UserID,
        canteen_id: mensa_models::CanteenID,
    ) -> StorageResult<()>;
}
