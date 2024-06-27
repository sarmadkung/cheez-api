use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use cheez_api::handlers::restaurant::get_all_restaurants;
use diesel::connection;


// app state
struct AppState {
    app_name: String,
    connection: connection::Connection,

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
    let connection = connection::establish_connection();
    // create app data
    let app_data = web::Data::new(AppState {
        app_name: String::from("Cheez"),
        connection: connection,
    });

    HttpServer::new(|| {
        App::new()
            .app_data(app_data.clone())
            .service(web::scope("/restaurant")
                .route("/get", web::get().to(get_all_restaurants))
            )
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
