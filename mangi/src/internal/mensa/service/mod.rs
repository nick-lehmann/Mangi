pub mod mock;
mod service;

use chrono::NaiveDate;

use super::models::{self, CanteenID};
pub use service::DefaultMensaService;

pub enum ServiceError {}

pub type ServiceResult<T> = Result<T, ServiceError>;

pub trait MensaService {
    fn get_canteens(&self) -> ServiceResult<Vec<models::Canteen>>;
    fn get_meals(&self, canteen_id: CanteenID, date: NaiveDate)
        -> ServiceResult<Vec<models::Meal>>;
}
