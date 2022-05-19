use crate::storage::StorageError;

use {
    super::{models, storage::UserStorage},
    diesel::result::Error as DieselError,
    log::debug,
};

pub trait UserService {
    fn get_user(&self, user_id: models::UserID) -> Option<models::User>;
    fn create_user(&self, user: models::User) -> models::User;
}

pub struct MangiUserService<'a, Storage: UserStorage> {
    storage: &'a Storage,
}

impl<'a, Storage: UserStorage> MangiUserService<'a, Storage> {
    pub fn new(storage: &'a Storage) -> Self {
        Self { storage }
    }
}

impl<'a, Storage: UserStorage> UserService for MangiUserService<'a, Storage> {
    fn get_user(&self, user_id: models::UserID) -> Option<models::User> {
        match self.storage.get_user(user_id) {
            Ok(potential_user) => potential_user,
            Err(_) => todo!(),
        }
    }

    fn create_user(&self, user: models::User) -> models::User {
        debug!("Creating new user {}", &user.name);

        match self.storage.get_user(user.id) {
            Ok(option) => match option {
                Some(user) => self.storage.update_user(user).unwrap(),
                None => self.storage.create_user(user).unwrap(),
            },
            Err(_) => todo!(),
        }
    }
}
