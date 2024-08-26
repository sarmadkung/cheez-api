use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use cheez_api::db_conn::{establish_connection_pool, DbPool};
use cheez_api::routes::auth::auth_routes;
use cheez_api::routes::menu::menu_routes;
use cheez_api::routes::restaurant::restaurant_routes;
use cheez_api::routes::user::user_routes;
use dotenvy::dotenv;
use std::env;

// app state
struct AppState {
    app_name: String,
    pool: DbPool,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = establish_connection_pool();

    let app_data = web::Data::new(AppState {
        app_name: String::from("Chez"),
        pool: pool,
    });

    let port: u16 = env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("Failed to parse PORT into u16");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Allow any origin to access the resources
            .allow_any_method() // Allow any HTTP method (GET, POST, etc.)
            .allow_any_header(); // Allow any header in the requests

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
