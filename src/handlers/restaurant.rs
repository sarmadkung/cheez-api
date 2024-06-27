use diesel::prelude::*;
use crate::db::establish_connection;
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

pub fn restaurants() -> Vec<Restaurant> {
    let connection = &mut establish_connection();
    let restaurants = schema::restaurants::table
        .load::<Restaurant>(connection)
        .expect("Error loading restaurants");
    println!("{:?}", restaurants);
    return restaurants;
}

pub fn create() {
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
}
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
