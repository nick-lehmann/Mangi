use serde::{Deserialize, Serialize};

pub type MensaID = i32;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mensa {
    pub id: MensaID,
    pub name: String,
}
