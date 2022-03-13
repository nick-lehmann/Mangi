use open_mensa::CanteenID;
use serde::{Deserialize, Serialize};

use super::{MensaID, Url};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Canteen {
    pub id: open_mensa::CanteenID,
    pub name: String,
    pub city: String,
    pub address: String,
    // pub coordinates: Coordinates,
    pub url: Option<Url>,
    pub menu: Option<Url>,
    pub mensa: MensaID,
}

impl Canteen {
    pub fn from_open_mensa(canteen: open_mensa::Canteen, mensa: MensaID) -> Self {
        Self {
            id: canteen.id,
            name: canteen.name,
            city: canteen.city,
            address: canteen.address,
            url: canteen.url,
            menu: canteen.menu,
            mensa: mensa,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CanteenDay {
    pub date: String,
    pub closed: bool,
}
