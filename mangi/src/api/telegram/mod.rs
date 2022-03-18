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
        "list".to_string(),
        "List all canteens 2".to_string(),
        &canteen_list_handler,
    );

    let canteen_list_callback_handler =
        |callback_query: CallbackQuery| food_controller.handle_callback(callback_query);
    dispatcher.register_callback("".to_string(), &canteen_list_callback_handler);

    // User settings
    let user_settings_controller = UserSettingsController::new(&api);

    let user_settings_list_handler = |message: Message| user_settings_controller.list(message);
    dispatcher.register_command(
        "settings".to_string(),
        "List user settings".to_string(),
        &user_settings_list_handler,
    );

    bot.start();
}
