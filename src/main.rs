use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use cheez_api::{db_conn};
use cheez_api::handlers::{restaurant::create, restaurant::restaurants};
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

// async fn restaurants() -> impl Responder {
    
//     let results = handlers::restaurant::restaurants();
//     let mut response = String::from("Restaurants: \n");
//     for restaurant in results {
//         response.push_str(&format!("{} - {}\n", restaurant.id, restaurant.name));
//     }
//     HttpResponse::Ok().body(response)
// }




#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let connection = &db_conn::establish_connection();
    // create app data
    let app_data = web::Data::new(AppState {
        app_name: String::from("Cheez"),
    });

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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
