use frankenstein::CallbackQuery;
use log::{debug, info};

use crate::{errors::AsTelegramBotError, Dispatcher};

pub type CallbackHandler<'a, Error> = &'a dyn Fn(CallbackQuery) -> Result<(), Error>;

pub trait CallbackDispatcher<'a, Error: AsTelegramBotError> {
    fn register_callback(&mut self, pattern: &'a str, handler: CallbackHandler<'a, Error>);
    fn handle_callback(&self, callback_query: CallbackQuery) -> Result<(), Error>;
}

impl<'a, Error: std::error::Error + AsTelegramBotError + Clone + Send + Sync>
    CallbackDispatcher<'a, Error> for Dispatcher<'a, Error>
{
    fn register_callback(&mut self, pattern: &'a str, handler: CallbackHandler<'a, Error>) {
        info!("Register callback handler for pattern {}", &pattern);
        self.callbacks.insert(pattern.to_string(), handler);
    }

    fn handle_callback(&self, callback_query: CallbackQuery) -> Result<(), Error> {
        let message = callback_query.message.as_ref().unwrap();
        let text = message.text.as_ref().unwrap().clone();

        debug!("Received a callback for messsage: {}", &text);

        for (pattern, callback) in &self.callbacks {
            if pattern == "" || &text == pattern {
                // TODO: Error handling
                (callback)(callback_query)?;
                break;
            }
        }
        Ok(())
    }
}
