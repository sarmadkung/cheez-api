
/// create restaurant all routes like create, read, update, delete

use actix_web::web;

use crate::handlers::restaurant::{create, restaurants,update,restaurant,delete,my_restaurants};

pub fn restaurant_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/restaurants")
        .route("", web::get().to(restaurants))
        .route("/create", web::post().to(create))
        .route("/update/{id}", web::put().to(update))
        .route("/{id}", web::get().to(restaurant))
        .route("delete/{id}", web::delete().to(delete))
        .route("/my", web::get().to(my_restaurants)),
    );
}
