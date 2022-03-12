use serde::{Deserialize, Serialize};

use super::Url;

pub type CanteenID = i32;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Canteen {
    pub id: CanteenID,
    pub name: String,
    pub city: String,
    pub address: String,
    // pub coordinates: Coordinates,
    pub url: Option<Url>,
    pub menu: Option<Url>,
    // pub mensa: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CanteenDay {
    pub date: String,
    pub closed: bool,
}
