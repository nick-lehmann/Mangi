#![allow(unused_imports)]
#[macro_use]
extern crate diesel;
extern crate diesel_derive_enum;
extern crate select;
extern crate serde;

pub mod api;
pub mod config;
pub mod internal;
pub mod storage;

use std::env;

use api::telegram::start_telegram_bot;
use chrono::{NaiveDate, Utc};
use internal::users::storage::UserStorage;
use internal::{
    mensa::{
        models as mensa_models, service::DefaultMensaService,
        storage::postgres::PostgresMensaStorage, sync::MensaSync,
    },
    users::models as user_models,
    users::storage::postgres::PostgresUserStorage,
};
use open_mensa::{OpenMensaClient, OpenMensaEndpoint};

pub fn main() {
    env_logger::init();
    dotenv::dotenv().unwrap();

    let database_pool = crate::config::get_pool().unwrap();
    let mensa_storage = PostgresMensaStorage::new(&database_pool);
    let user_storage = PostgresUserStorage::new(&database_pool);

    let open_mensa_client = OpenMensaClient::new(OpenMensaEndpoint::TUDresden);

    let mensa = mensa_models::Mensa {
        id: 1,
        name: "TU Dresden".into(),
    };

    let mensa_service = DefaultMensaService {
        storage: mensa_storage,
    };

    let meal_syncer = MensaSync {
        mensa: &mensa,
        open_mensa_client: &open_mensa_client,
        mensa_service: &mensa_service,
    };
    meal_syncer.sync(Some(Utc::now().naive_local().date()));

    let token = env::var("BOT_TOKEN").expect("Need to set 'BOT_TOKEN' environment variable");
    start_telegram_bot(token, &open_mensa_client);
}
