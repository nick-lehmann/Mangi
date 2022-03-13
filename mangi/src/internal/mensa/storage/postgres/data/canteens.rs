use open_mensa::CanteenID;

use super::super::schema::canteens;
use crate::internal::mensa::models::{self, Url};

#[derive(Queryable, Insertable, Debug)]
#[table_name = "canteens"]
pub struct Canteen {
    pub id: CanteenID,
    pub name: String,
    pub city: String,
    pub address: String,
    pub url: Option<Url>,
    pub menu: Option<Url>,
    pub mensa: i32,
}

impl From<Canteen> for models::Canteen {
    fn from(canteen: Canteen) -> Self {
        Self {
            id: canteen.id,
            name: canteen.name,
            city: canteen.city,
            address: canteen.address,
            url: canteen.url,
            menu: canteen.menu,
            mensa: canteen.mensa,
        }
    }
}

impl From<models::Canteen> for Canteen {
    fn from(canteen: models::Canteen) -> Self {
        Canteen {
            id: canteen.id,
            name: canteen.name,
            city: canteen.city,
            address: canteen.address,
            url: canteen.url,
            menu: canteen.menu,
            mensa: canteen.mensa,
        }
    }
}
