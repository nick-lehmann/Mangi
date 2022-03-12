#![allow(unused_imports)]
#[macro_use]
extern crate diesel;
extern crate serde;
use std::env;

use api::telegram::start_telegram_bot;
use open_mensa::{OpenMensaClient, mensas::OpenMensa};

pub mod api;
pub mod internal;
extern crate select;

pub fn main() {
    env_logger::init();
    dotenv::dotenv().unwrap();

    let token = env::var("BOT_TOKEN").expect("Need to set 'BOT_TOKEN' environment variable");

    let open_mensa_client = OpenMensaClient::new(OpenMensa::TUDresden);

    start_telegram_bot(token, &open_mensa_client);
}
