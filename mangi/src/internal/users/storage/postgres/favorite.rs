use crate::{internal::mensa::models::CanteenID, storage::schema::favorites};

use super::user::UserID;

#[derive(Insertable, Queryable, Debug, PartialEq)]
#[table_name = "favorites"]
pub struct Favorite {
    pub user_id: UserID,
    pub canteen_id: CanteenID,
}
