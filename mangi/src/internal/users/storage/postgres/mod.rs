use diesel::BoolExpressionMethods;
use diesel::QueryDsl;

use crate::config::PgPool;
use crate::diesel::query_dsl::select_dsl::SelectDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::internal::mensa::models as mensa_models;
use crate::internal::users::models;
use crate::storage::schema;
use crate::storage::StorageResult;

use super::UserStorage;

pub mod favorite;
pub mod user;

pub struct PostgresUserStorage<'a> {
    pool: &'a PgPool,
}

impl<'a> PostgresUserStorage<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
}

impl<'a> UserStorage for PostgresUserStorage<'a> {
    fn list_users(&self) -> StorageResult<Vec<models::User>> {
        let result: Vec<user::User> = schema::users::dsl::users.load(&self.pool.get()?)?;

        let mut users: Vec<models::User> = Vec::new();
        for db_user in result {
            users.push(db_user.try_into()?)
        }

        Ok(users)
    }

    fn get_user(&self, id: models::UserID) -> StorageResult<models::User> {
        let user: user::User = schema::users::dsl::users
            .filter(schema::users::dsl::id.eq(id))
            .first(&self.pool.get()?)?;

        user.try_into()
    }

    fn get_telegram_user(&self, id: telegram_bot::TelegramUserID) -> StorageResult<models::User> {
        let user: user::User = schema::users::dsl::users
            .filter(schema::users::dsl::telegram_user_id.eq(id as i32))
            .first(&self.pool.get()?)?;

        user.try_into()
    }

    fn create_user(&self, user: models::User) -> StorageResult<models::User> {
        let user_input: user::User = user.try_into()?;

        let user: user::User = diesel::insert_into(schema::users::dsl::users)
            .values(user_input)
            .get_result(&self.pool.get()?)?;

        user.try_into()
    }

    fn update_user(&self, user: models::User) -> StorageResult<models::User> {
        let user_input: user::User = user.try_into()?;

        let updated_user: user::User =
            diesel::update(schema::users::dsl::users.find(user_input.id))
                .set((
                    schema::users::dsl::name.eq(user_input.name),
                    schema::users::dsl::telegram_user_id.eq(user_input.telegram_user_id),
                    schema::users::dsl::telegram_chat_id.eq(user_input.telegram_chat_id),
                    schema::users::dsl::diet.eq(user_input.diet),
                    schema::users::dsl::user_type.eq(user_input.user_type),
                ))
                .get_result(&self.pool.get()?)?;

        updated_user.try_into()
    }

    fn delete_user(&self, user_id: models::UserID) -> StorageResult<()> {
        diesel::delete(schema::users::dsl::users.filter(schema::users::dsl::id.eq(user_id)))
            .execute(&self.pool.get()?)?;
        Ok(())
    }

    fn get_favorites(
        &self,
        user_id: models::UserID,
    ) -> StorageResult<Vec<mensa_models::CanteenID>> {
        let result: Vec<mensa_models::CanteenID> = QueryDsl::select(
            schema::favorites::dsl::favorites,
            schema::favorites::dsl::canteen_id,
        )
        .filter(schema::favorites::dsl::user_id.eq(user_id))
        .load(&self.pool.get()?)?;

        Ok(result)
    }

    fn add_favorite(
        &self,
        user_id: models::UserID,
        canteen_id: mensa_models::CanteenID,
    ) -> StorageResult<()> {
        diesel::insert_into(schema::favorites::dsl::favorites)
            .values(favorite::Favorite {
                user_id,
                canteen_id,
            })
            .execute(&self.pool.get()?)?;

        Ok(())
    }

    fn remove_favorite(
        &self,
        user_id: models::UserID,
        canteen_id: mensa_models::CanteenID,
    ) -> StorageResult<()> {
        diesel::delete(
            schema::favorites::dsl::favorites.filter(schema::favorites::dsl::canteen_id
                .eq(canteen_id)
                .and(schema::favorites::dsl::user_id.eq(user_id),)),
        )
        .execute(&self.pool.get()?)?;

        Ok(())
    }
}
