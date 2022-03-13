mod canteen;
pub use canteen::{Canteen, CanteenDay};
mod meal;
pub use meal::{Meal, MealPrices};
mod mensa;
pub use mensa::{Mensa, MensaID};

pub type Url = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    latitude: f32,
    longitude: f32,
}
