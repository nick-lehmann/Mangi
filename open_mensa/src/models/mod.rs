mod canteen;
pub use canteen::{Canteen, CanteenDay, CanteenID};
mod meal;
pub use meal::{Meal, MealID, MealPrices};

pub type Url = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    latitude: f32,
    longitude: f32,
}
