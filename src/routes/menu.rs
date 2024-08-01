// create routes for menu

use actix_web::web;

use crate::handlers::menu::{create_menu, get_menus_by_restaurant_id};

pub fn menu_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/menu")
            .route(
                "/restaurant/{restaurant_id}",
                web::get().to(get_menus_by_restaurant_id),
            )
            .route("/create", web::post().to(create_menu),
    ));
}