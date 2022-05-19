table! {
    use diesel::sql_types::*;

    canteens (id) {
        id -> Int4,
        name -> Varchar,
        city -> Varchar,
        address -> Varchar,
        url -> Nullable<Varchar>,
        menu -> Nullable<Varchar>,
        mensa -> Int4,
    }
}

table! {
    use diesel::sql_types::*;

    favorites (user_id, canteen_id) {
        user_id -> Int4,
        canteen_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;

    meals (id) {
        id -> Int4,
        name -> Varchar,
        category -> Nullable<Varchar>,
        price_student -> Float4,
        price_employee -> Float4,
        notes -> Nullable<Text>,
        image -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        day -> Date,
        canteen -> Int4,
    }
}

table! {
    use diesel::sql_types::*;

    mensa (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Int4,
        name -> Varchar,
        telegram_user_id -> Int4,
        telegram_chat_id -> Int4,
        user_type -> Varchar,
        diet -> Varchar,
    }
}

joinable!(canteens -> mensa (mensa));
joinable!(favorites -> canteens (canteen_id));
joinable!(favorites -> users (user_id));
joinable!(meals -> canteens (canteen));

allow_tables_to_appear_in_same_query!(canteens, favorites, meals, mensa, users,);
