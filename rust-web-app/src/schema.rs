// @generated automatically by Diesel CLI.

diesel::table! {
    properties (id) {
        id -> Integer,
        owner_id -> Integer,
        address -> Text,
        city -> Text,
        province -> Text,
        country -> Text,
        lat -> Float,
        lng -> Float,
        property_name -> Text,
        blurb -> Text,
        price -> Integer,
    }
}

diesel::table! {
    reservations (id) {
        id -> Integer,
        user_id -> Integer,
        property_id -> Integer,
        start_date -> Date,
        end_date -> Date,
    }
}

diesel::table! {
    reviews (id) {
        id -> Integer,
        user_id -> Integer,
        property_id -> Integer,
        rating -> Integer,
        comment -> Nullable<Text>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        username -> Text,
        email -> Text,
        hashed_password -> Text,
    }
}

diesel::joinable!(reservations -> properties (property_id));
diesel::joinable!(reservations -> users (user_id));
diesel::joinable!(reviews -> properties (property_id));
diesel::joinable!(reviews -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    properties,
    reservations,
    reviews,
    users,
);
