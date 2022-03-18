use chrono::Utc;
use frankenstein::{CallbackQuery, Message, Update};
use open_mensa::OpenMensaClient;
use telegram_bot::{CallbackDispatcher, CommandDispatcher, TelegramBot};

use self::commands::UserSettingsController;

pub mod commands;

pub fn start_telegram_bot(token: String, open_mensa_client: &OpenMensaClient) {
    let api = frankenstein::Api::new(&token);

    let mut bot = TelegramBot::new(&api);
    let dispatcher = &mut bot.dispatcher;

    // Food
    let food_controller = commands::FoodController::new(&api, &open_mensa_client);

    let canteen_list_handler = |message: Message| food_controller.list_food_by_canteen(message);
    dispatcher.register_command(
        "essen",
        "Heutige Menüs für beliebige Kantine",
        &canteen_list_handler,
    );

    let canteen_list_callback_handler =
        |callback_query: CallbackQuery| food_controller.canteen_chosen_callback(callback_query);
    dispatcher.register_callback("", &canteen_list_callback_handler);

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
    let user_settings_controller = UserSettingsController::new(&api);

    let user_settings_list_handler = |message: Message| user_settings_controller.list(message);
    dispatcher.register_command(
        "settings",
        "Deine Einstellungen",
        &user_settings_list_handler,
    );

    bot.start();
}
