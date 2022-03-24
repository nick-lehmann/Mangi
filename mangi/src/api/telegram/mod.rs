use std::{fs, io};

use {
    chrono::Utc,
    frankenstein::{Api, CallbackQuery, Message, Update},
    log::info,
    open_mensa::OpenMensaClient,
    telegram_bot::{AsTelegramBotError, CallbackDispatcher, CommandDispatcher, TelegramBot},
};

use crate::internal::users::service::UserService;

use self::commands::UserSettingsController;

pub mod commands;
mod errors;
mod helpers;
mod messages;
use messages::mangi_messages;

fn error_handler(_api: &Api, _error: errors::MangiTelegramError) {
    todo!()
}

pub fn start_telegram_bot<UserServiceImpl>(
    token: String,
    open_mensa_client: &OpenMensaClient,
    user_service: &UserServiceImpl,
) where
    UserServiceImpl: UserService,
{
    let api = frankenstein::Api::new(&token);

    let mut bot = TelegramBot::new(&api, error_handler);
    let dispatcher = &mut bot.dispatcher;

    // Welcome
    info!("Register welcome methods");
    let welcome_controller = commands::WelcomeController::new(&api, user_service);
    let start_handler = |message: Message| welcome_controller.start(message);
    dispatcher.register_command("start", "Sag hallo zu Mangi", &start_handler);

    let user_type_select_handler =
        |callback: CallbackQuery| welcome_controller.accept_user_type_select(callback);
    dispatcher.register_callback(mangi_messages.user_type, &user_type_select_handler);

    let diet_select_handler =
        |callback: CallbackQuery| welcome_controller.accept_diet_select(callback);
    dispatcher.register_callback(mangi_messages.diet, &diet_select_handler);

    // Food
    info!("Register food methods");
    let food_controller = commands::FoodController::new(&api, &open_mensa_client);

    let canteen_list_handler = |message: Message| food_controller.list_food_by_canteen(message);
    dispatcher.register_command(
        "essen",
        "Heutige Menüs für beliebige Kantine",
        &canteen_list_handler,
    );

    let canteen_list_callback_handler =
        |callback_query: CallbackQuery| food_controller.canteen_chosen_callback(callback_query);
    dispatcher.register_callback("Wähle eine Mensa ⬇", &canteen_list_callback_handler);

    let canteen_favorite_today_handler = |message: Message| {
        food_controller.list_food_for_favorite(message, Utc::now().naive_local().date())
    };
    dispatcher.register_command(
        "heute",
        "Heutige Speisen deiner Lieblingskantine",
        &canteen_favorite_today_handler,
    );

    let canteen_favorite_tomorrow_handler = |message: Message| {
        food_controller.list_food_for_favorite(message, Utc::now().naive_local().date().succ())
    };
    dispatcher.register_command(
        "morgen",
        "Morgige Speisen deiner Lieblingskantine",
        &canteen_favorite_tomorrow_handler,
    );

    // User settings
    info!("Register user methods");
    let user_settings_controller = UserSettingsController::new(&api);

    let user_settings_list_handler = |message: Message| user_settings_controller.list(message);
    dispatcher.register_command(
        "settings",
        "Deine Einstellungen",
        &user_settings_list_handler,
    );

    info!("Starting telegram bot");
    bot.start();
}
