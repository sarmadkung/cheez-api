/// create restaurant all routes like create, read, update, delete
use actix_web::web;

use crate::handlers::restaurant::{
    handle_create_restaurant, handle_delete, handle_update_restaurant, my_restaurants, restaurant,
    restaurants,
};

pub fn restaurant_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/restaurants")
            .route("", web::get().to(restaurants))
            .route("/create", web::post().to(handle_create_restaurant))
            .route("/update/{id}", web::put().to(handle_update_restaurant))
            .route("/{id}", web::get().to(restaurant))
            .route("delete/{id}", web::delete().to(handle_delete))
            .route("/my", web::get().to(my_restaurants)),
    );
}
