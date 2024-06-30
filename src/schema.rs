// @generated automatically by Diesel CLI.

diesel::table! {
    restaurants (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        location -> Varchar,
        rating -> Float8,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    restaurants,
    users,
);
