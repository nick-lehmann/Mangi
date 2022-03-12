use crate::internal::mensa::models;
use crate::internal::mensa::{models::CanteenID, storage::MensaStorage};

use super::MensaService;

pub struct DefaultMensaService<Storage: MensaStorage> {
    storage: Storage,
}

impl<Storage: MensaStorage> MensaService for DefaultMensaService<Storage> {
    fn get_canteens(&self) -> super::ServiceResult<Vec<models::Canteen>> {
        Ok(self.storage.list_canteens().unwrap())
    }

    fn get_meals(
        &self,
        canteen_id: CanteenID,
        day: chrono::NaiveDate,
    ) -> super::ServiceResult<Vec<models::Meal>> {
        Ok(self.storage.list_meals(canteen_id, day).unwrap())
    }
}
