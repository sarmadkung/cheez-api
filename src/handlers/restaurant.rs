use actix_web::{body, HttpResponse, Responder, web};
use diesel::prelude::*;
use crate::db_conn::establish_connection;
use crate::models::restaurant::Restaurant;
use crate::schema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// pub fn restaurants() -> Vec<Restaurant> {
//     let connection = &mut establish_connection();
//     let restaurants = schema::restaurants::table
//         .load::<Restaurant>(connection)
//         .expect("Error loading restaurants");
//     println!("{:?}", restaurants);
//     return restaurants;  
// }

pub async fn restaurants() -> impl Responder {
    let connection = &mut establish_connection();
    let restaurants = schema::restaurants::table
        .load::<Restaurant>(connection)
        .expect("Error loading restaurants");
    println!("{:?}", restaurants);
    HttpResponse::Ok().json(restaurants)
}


#[derive(Debug, Deserialize)]
pub struct ParseData {
    name: String,
    location: String,
    rating: f64
}

pub async fn create(restaurant_data: web::Json<ParseData>) -> impl Responder {

    let connection = &mut establish_connection();
    // let new_restaurant: Restaurant = serde_json::from_str(restaurant_data).unwrap();
    let new_restaurant = Restaurant {
        id: Uuid::new_v4(),
        name: restaurant_data.name.clone(),
        location: restaurant_data.location.clone(),
        rating: restaurant_data.rating.clone(),
    };
    diesel::insert_into(schema::restaurants::table)
        .values(&new_restaurant)
        .execute(connection)
        .expect("Error saving new restaurant");
    HttpResponse::Ok().body("New restaurant added")
}
    // diesel::insert_into(schema::restaurants::table)
    //     .values(&new_restaurant)
    //     .execute(connection)
    //     .expect("Error saving new restaurant");

    // HttpResponse::Ok().body("New restaurant created");
// }
pub fn update() {
    let connection = &mut establish_connection();
    let restaurant = schema::restaurants::table
        .filter(schema::restaurants::id.eq(Uuid::new_v4()))
        .first::<Restaurant>(connection)
        .expect("Error loading restaurant");
    diesel::update(&restaurant)
        .set(schema::restaurants::rating.eq(4.0))
        .execute(connection)
        .expect("Error updating restaurant");
}
