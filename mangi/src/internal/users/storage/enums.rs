use crate::internal::mensa::models as mensa_models;
use crate::internal::users::models;
use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum, PartialEq)]
pub enum UserType {
    Student,
    Employee,
}

impl Into<models::UserType> for UserType {
    fn into(self) -> models::UserType {
        match self {
            UserType::Student => models::UserType::Student,
            UserType::Employee => models::UserType::Employee,
        }
    }
}

impl From<models::UserType> for UserType {
    fn from(model_user_type: models::UserType) -> Self {
        match model_user_type {
            models::UserType::Student => UserType::Student,
            models::UserType::Employee => UserType::Employee,
        }
    }
}

#[derive(Debug, DbEnum, PartialEq)]
pub enum Diet {
    Omnivore,
    Vegetarian,
    Vegan,
}

impl Into<mensa_models::Diet> for Diet {
    fn into(self) -> mensa_models::Diet {
        match self {
            Diet::Omnivore => mensa_models::Diet::Omnivore,
            Diet::Vegetarian => mensa_models::Diet::Vegetarian,
            Diet::Vegan => mensa_models::Diet::Vegan,
        }
    }
}
impl From<mensa_models::Diet> for Diet {
    fn from(model_diet: mensa_models::Diet) -> Self {
        match model_diet {
            mensa_models::Diet::Omnivore => Diet::Omnivore,
            mensa_models::Diet::Vegetarian => Diet::Vegetarian,
            mensa_models::Diet::Vegan => Diet::Vegan,
        }
    }
}
