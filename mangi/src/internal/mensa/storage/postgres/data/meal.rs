use chrono::NaiveDate;
use open_mensa::{CanteenID, MealID};

use super::super::schema::meals;
use crate::internal::mensa::models;
use crate::internal::mensa::storage::StorageError;

type CommaSeparatedList = String;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "meals"]
pub struct Meal {
    pub id: MealID,
    pub name: String,
    pub category: Option<String>,
    pub price_student: f32,
    pub price_employee: f32,
    pub notes: Option<CommaSeparatedList>,
    pub image: Option<String>,
    pub url: Option<String>,
    pub day: NaiveDate,
    pub canteen: CanteenID,
}

impl TryFrom<Meal> for models::Meal {
    type Error = StorageError;

    fn try_from(meal: Meal) -> Result<Self, Self::Error> {
        let notes: Vec<String> = meal
            .notes
            .unwrap_or_default()
            .split(',')
            .map(|item| item.to_string())
            .collect();

        Ok(Self {
            id: meal.id,
            name: meal.name,
            prices: models::MealPrices {
                student: meal.price_student,
                employee: meal.price_employee,
            },
            notes,
            category: meal.category.unwrap_or_default(),
            image: meal.image.unwrap_or_default(),
            url: meal.url.unwrap_or_default(),
            day: meal.day,
            canteen: meal.canteen,
        })
    }
}

impl From<models::Meal> for Meal {
    fn from(meal: models::Meal) -> Self {
        Self {
            id: meal.id,
            name: meal.name,
            category: Some(meal.category),
            price_student: meal.prices.student,
            price_employee: meal.prices.employee,
            notes: Some(meal.notes.join(",")),
            image: Some(meal.image),
            url: Some(meal.url),
            day: meal.day,
            canteen: meal.canteen,
        }
    }
}
