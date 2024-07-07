// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        price -> Float8,
        about -> Nullable<Text>,
        time_to_make -> Nullable<Int4>,
        rating -> Nullable<Float8>,
        menu_id -> Uuid,
        restaurant_id -> Uuid,
    }
}

diesel::table! {
    menu (id) {
        id -> Uuid,
        name -> Text,
        user_id -> Uuid,
        restaurant_id -> Uuid,
    }
}

diesel::table! {
    restaurants (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        location -> Varchar,
        rating -> Nullable<Float8>,
        user_id -> Uuid,
    }
}

diesel::table! {
    reviews (id) {
        id -> Uuid,
        body -> Text,
        rating -> Float8,
        item_id -> Nullable<Uuid>,
        user_id -> Nullable<Uuid>,
        restaurant_id -> Nullable<Uuid>,
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

diesel::joinable!(items -> menu (menu_id));
diesel::joinable!(items -> restaurants (restaurant_id));
diesel::joinable!(menu -> restaurants (restaurant_id));
diesel::joinable!(menu -> users (user_id));
diesel::joinable!(restaurants -> users (user_id));
diesel::joinable!(reviews -> items (item_id));
diesel::joinable!(reviews -> restaurants (restaurant_id));
diesel::joinable!(reviews -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    items,
    menu,
    restaurants,
    reviews,
    users,
);
