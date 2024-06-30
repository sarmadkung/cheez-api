use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use cheez_api::{db_conn, handlers::restaurant::{create, restaurants}};
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


    let connection = &db_conn::establish_connection();
    // create app data
    let app_data = web::Data::new(AppState {
        app_name: String::from("Chez"),
    });

    let port: u16 = env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("Failed to parse PORT into u16");

    HttpServer::new( move || {
        App::new()
            .app_data(app_data.clone())
            .service(web::scope("/restaurants")
                .route("", web::get().to(restaurants))
                .route("/create", web::post().to(create))
            )
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
