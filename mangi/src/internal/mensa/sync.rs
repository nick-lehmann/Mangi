use std::collections::HashSet;

use crate::internal::mensa::models;
use chrono::{NaiveDate, Utc};
use log::{debug, info, log, Level};
use open_mensa;

use super::service::MensaService;

pub struct MealSync<'a, Service: MensaService> {
    pub mensa: &'a models::Mensa,
    pub open_mensa_client: &'a open_mensa::OpenMensaClient,
    pub meal_service: &'a Service,
}

impl<'a, Service: MensaService> MealSync<'a, Service> {
    fn fetch_canteens(&self) -> Vec<models::Canteen> {
        self.open_mensa_client
            .get_canteens()
            .unwrap()
            .into_iter()
            .map(|canteen| models::Canteen::from_open_mensa(canteen, self.mensa.id))
            .collect()
    }

    fn sync_canteens(&self) -> Vec<models::Canteen> {
        let canteens = self.fetch_canteens();
        let stored_canteens = self.meal_service.get_canteens().unwrap();

        let canteen_set: HashSet<&models::Canteen> = canteens.iter().collect();
        let stored_canteen_set: HashSet<&models::Canteen> = stored_canteens.iter().collect();

        let new_canteens: Vec<&&models::Canteen> =
            canteen_set.difference(&stored_canteen_set).collect();
        info!(
            "Adding {:?} new canteens not currently present",
            new_canteens
        );

        for new_canteen in new_canteens {
            self.meal_service
                .store_canteen(new_canteen.clone().clone())
                .unwrap();
        }

        canteens
    }

    fn sync_meals(&self, canteen: &models::Canteen, date: NaiveDate) {
        let meals = self.open_mensa_client.get_meals(canteen.id, date).unwrap();
        debug!("Found {} meals for canteen {}", meals.len(), &canteen.name);

        for meal in meals {
            self.meal_service
                .store_meal(models::Meal::from_open_mensa(meal, canteen.id, date))
                .unwrap();
        }
    }

    pub fn sync(&self, date: Option<NaiveDate>) {
        let date = date.unwrap_or(Utc::today().naive_local());
        let canteens = self.sync_canteens();
        for canteen in canteens.iter().take(5) {
            self.sync_meals(&canteen, date);
        }
    }
}
