use crate::handlers::menu::get_menus_by_restaurant_id;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use cheez_api::handlers::auth::login;
use cheez_api::handlers::user::{create_user, users};
use cheez_api::{
    db_conn,
    handlers::restaurant::{create, restaurants},
};
use std::env;

// app state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // create app data
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
            .service(
                web::scope("/restaurants")
                    .route("", web::get().to(restaurants))
                    .route("/create", web::post().to(create)),
            )
            .service(
                web::scope("/users")
                    .route("/create", web::post().to(create_user))
                    .route("", web::get().to(users)),
            )
            .service(
                web::space("/menu")
                    .route(
                        "/restaurant/{restaurant_id}",
                        web::get().to(get_menus_by_restaurant_id),
                    )
                    .route("/create", web::post().to(create_menu)),
            )
            .service(web::scope("/auth").route("/login", web::post().to(login)))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
