use actix_web::web;

use crate::handlers::user::{handle_create_user, users, update};



pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/create", web::post().to(handle_create_user))
            .route("", web::get().to(users))
            .route("/update/{user_id}", web::put().to(update)) 
    );
}
