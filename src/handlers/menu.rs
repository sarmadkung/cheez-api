use crate::db_conn::establish_connection;
use crate::models::menu::{CreateMenu, Menu};
use crate::schema;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;

// Get menus by restaurant id in params
pub async fn get_menus_by_restaurant_id(restaurant_id: web::Path<Uuid>) -> impl Responder {
    let connection = &mut establish_connection();
    let res = schema::menu::table
    .filter(schema::menu::restaurant_id.eq(restaurant_id.into_inner()))
    .load::<Menu>(connection);
    match res {
        Ok(menu) => HttpResponse::Ok().json(menu),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error getting menus: {:?}", e)),
    }
}

// Create menu for restaurant
pub async fn create_menu(menu: web::Json<CreateMenu>) -> impl Responder {
    let connection = &mut establish_connection();
    let menu = menu.into_inner();
    let new_menu = Menu {
        id: Uuid::new_v4(),
        name: menu.name,
        user_id: Uuid::new_v4(),
        restaurant_id: menu.restaurant_id,
    };
    let res = diesel::insert_into(schema::menu::table)
        .values(&new_menu)
        .execute(connection);
    match res {
        Ok(_) => HttpResponse::Ok().json(new_menu),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating menu: {:?}", e)),
    }
}
