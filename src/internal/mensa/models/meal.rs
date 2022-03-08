use chrono::NaiveDate;
use serde::{self, Deserialize, Serialize};

use super::{CanteenID, Url};
extern crate serde_derive;

pub type MealID = i32;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Meal {
    pub id: MealID,
    pub name: String,
    pub prices: MealPrices,
    pub notes: Vec<String>,
    pub category: String,
    pub image: Url,
    pub url: Url,
    pub day: NaiveDate,
    pub canteen: CanteenID,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MealPrices {
    #[serde(rename = "Studierende")]
    pub student: f32,
    #[serde(rename = "Bedienstete")]
    pub employee: f32,
}
