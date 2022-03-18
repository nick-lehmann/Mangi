#![allow(unused_imports)]
#[macro_use]
extern crate diesel;
extern crate serde;
use std::env;

use api::telegram::start_telegram_bot;
use chrono::NaiveDate;
use internal::mensa::{
    models,
    service::DefaultMensaService,
    storage::postgres::storage::{DbConfig, MensaPostgresStorage},
    sync::MealSync,
};
use open_mensa::{OpenMensaClient, OpenMensaEndpoint};

pub mod api;
pub mod internal;
extern crate select;

pub fn main() {
    env_logger::init();
    dotenv::dotenv().unwrap();

    let token = env::var("BOT_TOKEN").expect("Need to set 'BOT_TOKEN' environment variable");

    let open_mensa_client = OpenMensaClient::new(OpenMensaEndpoint::TUDresden);

    // let mensa = models::Mensa {
    //     id: 1,
    //     name: "TU Dresden".into(),
    // };

    // let storage = MensaPostgresStorage::new(DbConfig {
    //     host: "localhost".into(),
    //     port: 5433,
    //     user: "mangi".into(),
    //     password: "mangi".into(),
    //     database: "mangi".into(),
    // });

    // let service = DefaultMensaService { storage: storage };

    // let meal_syncer = MealSync {
    //     mensa: &mensa,
    //     open_mensa_client: &open_mensa_client,
    //     meal_service: &service,
    // };

    // meal_syncer.sync(Some(NaiveDate::from_ymd(2022, 3, 9)));

    start_telegram_bot(token, &open_mensa_client);
}
