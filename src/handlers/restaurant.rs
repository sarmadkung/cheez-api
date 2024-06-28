use actix_web::{body, HttpResponse, Responder};
use diesel::prelude::*;
use crate::db_conn::establish_connection;
use crate::models::restaurant::Restaurant;
use crate::schema;

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

// create should accept post request body and insert into database

pub async fn create() -> impl Responder {
    let connection = &mut establish_connection();
    let new_restaurant = Restaurant {
        id: 1,
        name: String::from("Cheez"),
        location: String::from("Kathmandu"),
        rating: 5,
    };
    diesel::insert_into(schema::restaurants::table)
        .values(&new_restaurant)
        .execute(connection)
        .expect("Error saving new restaurant");
    HttpResponse::Ok().body("New restaurant added")
}
    // let connection = &mut establish_connection();
    // let new_restaurant = Restaurant {
    //     id: 1,
    //     name: String::from("Cheez"),
    //     location: String::from("Kathmandu"),
    //     rating: 5,
    // };
    // diesel::insert_into(schema::restaurants::table)
    //     .values(&new_restaurant)
    //     .execute(connection)
    //     .expect("Error saving new restaurant");

    // HttpResponse::Ok().body("New restaurant created");
    // HttpResponse::Ok().body("New restaurant created");
// }
pub fn update() {
    let connection = &mut establish_connection();
    let restaurant = schema::restaurants::table
        .filter(schema::restaurants::id.eq(1))
        .first::<Restaurant>(connection)
        .expect("Error loading restaurant");
    diesel::update(&restaurant)
        .set(schema::restaurants::rating.eq(4))
        .execute(connection)
        .expect("Error updating restaurant");
}
