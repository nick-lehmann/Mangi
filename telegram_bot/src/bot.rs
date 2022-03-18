use frankenstein::Api;
use frankenstein::GetUpdatesParams;
use frankenstein::GetUpdatesParamsBuilder;
use frankenstein::SetMyCommandsParamsBuilder;
use frankenstein::TelegramApi;

use log::error;
use log::info;

use crate::Dispatcher;

pub struct TelegramBot<'a> {
    api: &'a Api,
    pub dispatcher: Dispatcher<'a>,
}

impl<'a> TelegramBot<'a> {
    pub fn new(api: &'a Api) -> Self {
        Self {
            api,
            dispatcher: Dispatcher::new(),
        }
    }

    /// Informs telegram about the commands available for the bot
    ///
    /// This allows the user to have autocompletion for the commands. While typing a command,
    /// the description for the command is also shown to the user.
    fn set_bot_commands(&self) {
        let commands: Vec<frankenstein::BotCommand> = self
            .dispatcher
            .get_commands()
            .into_iter()
            .map(|command| command.clone().into())
            .collect();

        let params = SetMyCommandsParamsBuilder::default()
            .commands(commands)
            .build()
            .unwrap();

        self.api
            .set_my_commands(&params)
            .expect("Unable to set bot commands");
    }

    fn build_update_params(&self, offset: Option<u32>) -> GetUpdatesParams {
        let mut update_params_builder = GetUpdatesParamsBuilder::default();
        // TODO: Replace magic strings by enum
        update_params_builder
            .allowed_updates(vec!["message".to_string(), "callback_query".to_string()]);

        if let Some(offset_value) = offset {
            update_params_builder.offset(offset_value);
        }

        update_params_builder.build().unwrap()
    }

    pub fn start(&self) {
        info!("Starting bot");

        info!("Setting bot commands");
        self.set_bot_commands();

        let mut offset = None;

        loop {
            let update_params = self.build_update_params(offset);
            let result = self.api.get_updates(&update_params);

            if result.is_err() {
                error!("Failed to get updates: {:?}", result);
                continue;
            }

            let response = result.unwrap();

            for update in response.result {
                let update_id = update.update_id;

                self.dispatcher
                    .dispatch(update)
                    .expect("Unable to dispatch event");

                offset = Some(update_id + 1);
            }
        }
    }
}
