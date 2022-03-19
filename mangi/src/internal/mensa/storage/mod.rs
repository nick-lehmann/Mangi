use chrono::NaiveDate;

use crate::storage::StorageResult;

use super::models;

pub mod postgres;

pub trait MensaStorage {
    // fn create_mensa() -> StorageResult<Mensa>;

    fn list_canteens(&self) -> StorageResult<Vec<models::Canteen>>;
    fn create_canteen(&self, canteen: models::Canteen) -> StorageResult<models::Canteen>;

    fn list_meals(
        &self,
        canteen: open_mensa::CanteenID,
        day: NaiveDate,
    ) -> StorageResult<Vec<models::Meal>>;
    fn create_meal(&self, meal: models::Meal) -> StorageResult<models::Meal>;
}
