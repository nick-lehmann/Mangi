pub mod data;

use {
    diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl},
    log::debug,
    r2d2::Pool,
};

use crate::{
    config::PgPool,
    internal::mensa::{
        models,
        storage::{MensaStorage, StorageResult},
    },
    storage::schema::{
        canteens::{all_columns as all_canteen_columns, dsl as canteens},
        meals::{all_columns as all_meal_columns, dsl as meals},
    },
};

pub struct PostgresMensaStorage<'a> {
    pool: &'a PgPool,
}

impl<'a> PostgresMensaStorage<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        PostgresMensaStorage { pool }
    }
}

impl<'a> MensaStorage for PostgresMensaStorage<'a> {
    fn list_canteens(&self) -> StorageResult<Vec<models::Canteen>> {
        let connection = self.pool.get()?;

        let result: Vec<data::Canteen> = canteens::canteens
            .select(all_canteen_columns)
            .load(&connection)?;

        Ok(result.into_iter().map(|item| item.into()).collect())
    }

    fn create_canteen(&self, canteen: models::Canteen) -> StorageResult<models::Canteen> {
        let connection = self.pool.get().unwrap();
        let input: data::Canteen = canteen.into();

        debug!("Inserting: {:#?}", &input);

        let canteen: data::Canteen = diesel::insert_into(canteens::canteens)
            .values(&input)
            .get_result(&connection)?;

        Ok(canteen.into())
    }

    fn list_meals(
        &self,
        _cantenn: open_mensa::CanteenID,
        _day: chrono::NaiveDate,
    ) -> StorageResult<Vec<models::Meal>> {
        let connection = self.pool.get()?;

        let items: Vec<data::Meal> = meals::meals.select(all_meal_columns).load(&connection)?;

        items.into_iter().map(|item| item.try_into()).collect()
    }

    fn create_meal(&self, meal: models::Meal) -> StorageResult<models::Meal> {
        let connection = self.pool.get()?;
        let input: data::Meal = meal.into();

        debug!("Insert: {:#?}", &input);

        let meal: data::Meal = diesel::insert_into(meals::meals)
            .values(&input)
            .get_result(&connection)?;

        debug!("Insert successful: {:#?}", meal);

        meal.try_into()
    }
}
