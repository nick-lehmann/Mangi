use diesel::r2d2::ConnectionManager;
use diesel::QueryDsl;
use diesel::{PgConnection, RunQueryDsl};
use r2d2::Pool;

use super::data;
use super::schema::canteens::{all_columns as all_canteen_columns, dsl as canteens};
use super::schema::meals::{all_columns as all_meal_columns, dsl as meals};
use crate::internal::mensa::models;
use crate::internal::mensa::storage::{MensaStorage, StorageResult};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct MensaPostgresStorage {
    pool: PgPool,
}

impl MensaPostgresStorage {
    pub fn new(database_url: String) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder().max_size(5).build(manager).unwrap();

        MensaPostgresStorage { pool }
    }
}

impl MensaStorage for MensaPostgresStorage {
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

        let canteen: data::Canteen = diesel::insert_into(canteens::canteens)
            .values(&input)
            .get_result(&connection)?;

        return Ok(canteen.into());
    }

    fn list_meals(
        &self,
        _cantenn: models::CanteenID,
        _day: chrono::NaiveDate,
    ) -> StorageResult<Vec<models::Meal>> {
        let connection = self.pool.get()?;

        let items: Vec<data::Meal> = meals::meals.select(all_meal_columns).load(&connection)?;

        items.into_iter().map(|item| item.try_into()).collect()
    }

    fn create_meal(&self, meal: models::Meal) -> StorageResult<models::Meal> {
        let connection = self.pool.get()?;
        let input: data::Meal = meal.into();

        let canteen: data::Meal = diesel::insert_into(meals::meals)
            .values(&input)
            .get_result(&connection)?;

        return Ok(canteen.try_into()?);
    }
}
