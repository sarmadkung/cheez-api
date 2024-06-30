use actix_web::{ HttpResponse, Responder, web};
use diesel::prelude::*;
use crate::db_conn::establish_connection;
use crate::models::restaurant::{CreateRestaurant, Restaurant};
use crate::schema;
use uuid::Uuid;


pub async fn restaurants() -> impl Responder {
    let connection = &mut establish_connection();
    let restaurants = schema::restaurants::table
        .load::<Restaurant>(connection)
        .expect("Error loading restaurants");
    println!("{:?}", restaurants);
    HttpResponse::Ok().json(restaurants)
}


pub async fn create(restaurant_data: web::Json<CreateRestaurant>) -> impl Responder {

    let connection = &mut establish_connection();
    let new_restaurant = Restaurant {
        id: Uuid::new_v4(),
        name: restaurant_data.name.clone(),
        location: restaurant_data.location.clone(),
        rating: 5.0,
    };
    diesel::insert_into(schema::restaurants::table)
        .values(&new_restaurant)
        .execute(connection)
        .expect("Error saving new restaurant");
    HttpResponse::Ok().body("New restaurant added")
}

pub fn update(restaurant_id:Uuid) {
    let connection = &mut establish_connection();
    let restaurant = schema::restaurants::table
        .filter(schema::restaurants::id.eq(restaurant_id.clone()))
        .first::<Restaurant>(connection)
        .expect("Error loading restaurant");
    diesel::update(&restaurant)
        .set(schema::restaurants::rating.eq(4.0))
        .execute(connection)
        .expect("Error updating restaurant");
}
