use {
    chrono::NaiveDate,
    open_mensa,
    serde::{self, Deserialize, Serialize},
};

use super::Url;
extern crate serde_derive;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Serialize, Deserialize)]
pub struct Meal {
    pub id: open_mensa::MealID,
    pub name: String,
    pub prices: MealPrices,
    pub notes: Vec<String>,
    pub category: String,
    pub image: Url,
    pub url: Url,
    pub day: NaiveDate,
    pub canteen: open_mensa::CanteenID,
}

impl Meal {
    pub fn from_open_mensa(
        meal: open_mensa::Meal,
        canteen_id: open_mensa::CanteenID,
        day: NaiveDate,
    ) -> Self {
        Self {
            id: meal.id,
            name: meal.name,
            prices: meal.prices.into(),
            notes: meal.notes,
            category: meal.category,
            image: meal.image,
            url: meal.url,
            day: day,
            canteen: canteen_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MealPrices {
    #[serde(rename = "Studierende")]
    pub student: f32,
    #[serde(rename = "Bedienstete")]
    pub employee: f32,
}

impl From<open_mensa::MealPrices> for MealPrices {
    fn from(meal_prices: open_mensa::MealPrices) -> Self {
        Self {
            student: meal_prices.student,
            employee: meal_prices.employee,
        }
    }
}

impl Eq for MealPrices {}
