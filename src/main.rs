use std::env;
use dotenvy::dotenv;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use cheez_api::routes::auth::auth_routes;
use cheez_api::routes::menu::menu_routes;
use cheez_api::routes::restaurant::restaurant_routes;
use cheez_api::routes::user::user_routes;


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
        App::new()
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
