pub mod canteen;
pub mod meal;
pub mod offer;

pub type Url = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    latitude: f32,
    longitude: f32,
}
