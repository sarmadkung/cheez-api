use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use cheez_api::routes::auth::auth_routes;
use cheez_api::routes::menu::menu_routes;
use cheez_api::routes::restaurant::restaurant_routes;
use cheez_api::routes::user::user_routes;
use dotenvy::dotenv;
use std::env;

// app state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_data = web::Data::new(AppState {
        app_name: String::from("Chez"),
    });

    let port: u16 = env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("Failed to parse PORT into u16");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("*")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .configure(restaurant_routes)
            .configure(user_routes)
            .configure(auth_routes)
            .configure(menu_routes)
            .service(hello)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
