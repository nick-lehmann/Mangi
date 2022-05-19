use open_mensa::CanteenID;

use crate::internal::mensa::{models, storage::MensaStorage};

use super::MensaService;

pub struct DefaultMensaService<Storage: MensaStorage> {
    pub storage: Storage,
}

impl<Storage: MensaStorage> MensaService for DefaultMensaService<Storage> {
    fn get_canteen(&self, id: CanteenID) -> super::ServiceResult<Option<models::Canteen>> {
        Ok(self.storage.get_canteen(id).unwrap())
    }

    fn get_canteens(&self) -> super::ServiceResult<Vec<models::Canteen>> {
        Ok(self.storage.list_canteens().unwrap())
    }

    fn store_canteen(&self, canteen: models::Canteen) -> super::ServiceResult<()> {
        self.storage.create_canteen(canteen).unwrap();
        Ok(())
    }

    fn get_meals(
        &self,
        canteen_id: CanteenID,
        day: chrono::NaiveDate,
    ) -> super::ServiceResult<Vec<models::Meal>> {
        Ok(self.storage.list_meals(canteen_id, day).unwrap())
    }

    fn store_meal(&self, meal: models::Meal) -> super::ServiceResult<()> {
        self.storage.create_meal(meal).unwrap();
        Ok(())
    }
}
