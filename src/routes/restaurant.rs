
/// create restaurant all routes like create, read, update, delete

use actix_web::{web, HttpResponse, Responder};
use crate::handlers::restaurant;

pub fn restaurant_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/restaurant")
            // .route("/get", web::get().to(|| HttpResponse::Ok().json(restaurant::restaurants())))
            // .route("/create", web::post().to(restaurant::create))
            // .route("/update", web::put().to(restaurant::update))
            // .route("/delete", web::delete().to())
    );
}
