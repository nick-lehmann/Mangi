use crate::internal::mensa::models;

use super::MealService;

pub struct MockMealService {
    canteens: Vec<models::Canteen>,
    meals: Vec<models::Meal>,
}

impl MealService for MockMealService {
    fn get_canteens(&self) -> super::ServiceResult<Vec<models::Canteen>> {
        return Ok(self.canteens.clone());
    }

    fn get_meals(&self, _date: chrono::NaiveDate) -> super::ServiceResult<Vec<models::Meal>> {
        return Ok(self.meals.clone());
    }
}
