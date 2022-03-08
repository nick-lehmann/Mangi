use std::num::ParseFloatError;

use chrono::NaiveDate;
use diesel::result::Error as DieselError;

use super::models;

mod errors;
pub use errors::StorageError;
pub mod postgres;

pub type StorageResult<T> = Result<T, StorageError>;

pub trait MensaStorage {
    // fn create_mensa() -> StorageResult<Mensa>;

    fn list_canteens(&self) -> StorageResult<Vec<models::Canteen>>;
    fn create_canteen(&self, canteen: models::Canteen) -> StorageResult<models::Canteen>;

    fn list_meals(
        &self,
        canteen: models::CanteenID,
        day: NaiveDate,
    ) -> StorageResult<Vec<models::Meal>>;
    fn create_meal(&self, meal: models::Meal) -> StorageResult<models::Meal>;
}
