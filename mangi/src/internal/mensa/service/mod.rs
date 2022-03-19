mod diet;
mod service;

use chrono::NaiveDate;
use open_mensa::CanteenID;

use super::models;
pub use service::DefaultMensaService;

#[derive(Debug)]
pub enum ServiceError {}

pub type ServiceResult<T> = Result<T, ServiceError>;

pub trait MensaService {
    fn get_canteens(&self) -> ServiceResult<Vec<models::Canteen>>;
    fn store_canteen(&self, canteen: models::Canteen) -> ServiceResult<()>;
    fn get_meals(&self, canteen_id: CanteenID, date: NaiveDate)
        -> ServiceResult<Vec<models::Meal>>;
    fn store_meal(&self, meal: models::Meal) -> ServiceResult<()>;
}
