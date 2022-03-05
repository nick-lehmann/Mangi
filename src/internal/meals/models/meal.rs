use serde::{self, Deserialize, Serialize};

use super::Url;
extern crate serde_derive;

type MealID = u32;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Meal {
    pub id: MealID,
    pub name: String,
    pub prices: MealPrices,
    pub notes: Vec<String>,
    pub category: String,
    pub image: Url,
    pub url: Url,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MealPrices {
    #[serde(rename = "Studierende")]
    pub student: f32,
    #[serde(rename = "Bedienstete")]
    pub employee: f32,
}
