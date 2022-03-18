use frankenstein::{CallbackQuery, Message, Update};
use log::info;

use crate::{Dispatcher, TelegramResult};

pub type CallbackHandler<'a> = &'a dyn Fn(CallbackQuery) -> TelegramResult<()>;

pub trait CallbackDispatcher<'a> {
    fn register_callback(&mut self, pattern: String, handler: CallbackHandler<'a>);
    fn handle_callback(&self, update: Update);
}

impl<'a> CallbackDispatcher<'a> for Dispatcher<'a> {
    fn register_callback(&mut self, pattern: String, handler: CallbackHandler<'a>) {
        info!("Register callback handler for pattern {}", &pattern);
        self.callbacks.insert(pattern, handler);
    }

    fn handle_callback(&self, update: Update) {
        let callback_query = update.callback_query.unwrap();
        let message = callback_query.message.as_ref().unwrap();
        let text = message.text.as_ref().unwrap().clone();

        for (pattern, callback) in &self.callbacks {
            if pattern == "" || &text == pattern {
                // TODO: Error handling
                (callback)(callback_query).expect("Failed to handle callback query");
                return;
            }
        }
    }
}
