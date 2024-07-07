use crate::db_conn::establish_connection;
use crate::models::item::{CreateItem, Item};
use crate::schema;
use actix_web::{web, HttpResponse, Responder};
use diesel::RunQueryDsl;
use uuid::Uuid;

// Create item
pub async fn create_item(item: web::Json<CreateItem>) -> impl Responder {
    let connection = &mut establish_connection();
    let item = item.into_inner();
    let new_item = Item {
        id: Uuid::new_v4(),
        name: item.name,
        price: item.price,
        menu_id: item.menu_id,
        rating: 0.0,
        about: item.about,
        restaurant_id: item.restaurant_id,
    };
    let res = diesel::insert_into(schema::items::table)
        .values(&new_item)
        .execute(connection);
    match res {
        Ok(_) => HttpResponse::Ok().json(new_item),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating item: {:?}", e)),
    }
}
