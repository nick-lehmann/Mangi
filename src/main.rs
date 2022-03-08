#[macro_use]
extern crate diesel;
extern crate serde;
use chrono::NaiveDate;
use internal::mensa::scraper::{OpenMensa, OpenMensaClient};

pub mod api;
pub mod internal;
extern crate select;

pub fn main() {
    env_logger::init();
    dotenv::dotenv().unwrap();

    let open_mensa_client = OpenMensaClient::new(OpenMensa::TUDresden);
    let canteens = open_mensa_client.get_canteens().unwrap();
    println!("{:#?}", canteens);

    let meals = open_mensa_client.get_meals(4, NaiveDate::from_ymd(2022, 3, 4));
    println!("{:#?}", meals);
}
