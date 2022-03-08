pub mod mock;

use chrono::NaiveDate;

use super::models;

pub enum ServiceError {}

pub type ServiceResult<T> = Result<T, ServiceError>;

pub trait MealService {
    fn get_canteens(&self) -> ServiceResult<Vec<models::Canteen>>;
    fn get_meals(&self, date: NaiveDate) -> ServiceResult<Vec<models::Meal>>;
}
