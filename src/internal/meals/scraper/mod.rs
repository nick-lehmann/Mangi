use chrono::NaiveDate;
use ureq::Error;

use super::models::{
    canteen::{Canteen, CanteenDay, CanteenID},
    meal::Meal,
};

pub enum OpenMensa {
    TUDresden,
}

impl OpenMensa {
    fn as_str(&self) -> &'static str {
        match self {
            OpenMensa::TUDresden => "https://api.studentenwerk-dresden.de/openmensa/v2",
        }
    }
}

pub type ClientResult<T> = Result<T, Error>;

pub struct OpenMensaClient {
    base_url: String,
}

impl OpenMensaClient {
    pub fn new(mensa: OpenMensa) -> Self {
        OpenMensaClient {
            base_url: mensa.as_str().to_owned(),
        }
    }

    pub fn get_canteens(&self) -> ClientResult<Vec<Canteen>> {
        let endpoint = format!("{}/canteens", self.base_url);
        let body = ureq::get(&endpoint).call()?.into_string().unwrap();

        Ok(serde_json::from_str(&body).unwrap())
    }

    pub fn get_canteen(&self, canteen_id: CanteenID) -> ClientResult<Canteen> {
        let endpoint = format!("{}/canteens/{}", self.base_url, canteen_id);
        let body = ureq::get(&endpoint).call()?.into_string().unwrap();

        Ok(serde_json::from_str(&body).unwrap())
    }

    pub fn get_days(&self, canteen_id: CanteenID) -> ClientResult<Vec<CanteenDay>> {
        let endpoint = format!("{}/canteens/{}/days", self.base_url, canteen_id);
        let body = ureq::get(&endpoint).call()?.into_string().unwrap();

        Ok(serde_json::from_str(&body).unwrap())
    }

    pub fn get_meals(self, canteen_id: CanteenID, day: NaiveDate) -> ClientResult<Vec<Meal>> {
        let canteen_day = day.format("%Y-%m-%d");

        let endpoint = format!(
            "{}/canteens/{}/days/{}/meals",
            self.base_url, canteen_id, canteen_day
        );
        let body = ureq::get(&endpoint).call()?.into_string().unwrap();
        let jd = &mut serde_json::Deserializer::from_str(&body);

        let result: Result<Vec<Meal>, _> = serde_path_to_error::deserialize(jd);

        match result {
            Ok(x) => Ok(x),
            Err(err) => {
                let path = err.path().to_string();
                println!("Path of error: {}", path);
                println!("{}", err);
                Ok(vec![])
            }
        }
    }
}
