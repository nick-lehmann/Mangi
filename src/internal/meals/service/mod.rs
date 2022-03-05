pub mod mock;

use chrono::NaiveDate;

use super::models::{canteen::Canteen, meal::Meal};

pub enum ServiceError {}

pub type ServiceResult<T> = Result<T, ServiceError>;

pub trait MealService {
    fn get_canteens(&self) -> ServiceResult<Vec<Canteen>>;
    fn get_meals(&self, date: NaiveDate) -> ServiceResult<Vec<Meal>>;
}
