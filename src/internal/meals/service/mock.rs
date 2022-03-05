use crate::internal::meals::models::{canteen::Canteen, meal::Meal};

use super::MealService;

pub struct MockMealService {
    canteens: Vec<Canteen>,
    meals: Vec<Meal>,
}

impl MealService for MockMealService {
    fn get_canteens(&self) -> super::ServiceResult<Vec<Canteen>> {
        return Ok(self.canteens.clone());
    }

    fn get_meals(&self, _date: chrono::NaiveDate) -> super::ServiceResult<Vec<Meal>> {
        return Ok(self.meals.clone());
    }
}
