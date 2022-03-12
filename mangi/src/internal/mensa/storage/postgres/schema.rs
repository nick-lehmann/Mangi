table! {
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
    mensa (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

joinable!(canteens -> mensa (mensa));
joinable!(meals -> canteens (canteen));

allow_tables_to_appear_in_same_query!(
    canteens,
    meals,
    mensa,
);
