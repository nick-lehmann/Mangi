use super::{canteen::Canteen, meal::Meal};

#[derive(Debug, Clone, PartialEq)]
pub struct Offer {
    pub canteen: Canteen,
    pub meals: Vec<Meal>,
}
