use crate::internal::mensa::models::{self, CanteenID};

use super::MensaService;

pub struct MockMealService {
    canteens: Vec<models::Canteen>,
    meals: Vec<models::Meal>,
}

impl MensaService for MockMealService {
    fn get_canteens(&self) -> super::ServiceResult<Vec<models::Canteen>> {
        Ok(self.canteens.clone())
    }

    fn get_meals(
        &self,
        canteen_id: CanteenID,
        _date: chrono::NaiveDate,
    ) -> super::ServiceResult<Vec<models::Meal>> {
        Ok(self.meals.clone())
    }
}
