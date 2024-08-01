use actix_web::web;
use crate::handlers::auth::login;



pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
    );
}
