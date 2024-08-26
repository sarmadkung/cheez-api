use crate::db_conn::DbPool;
use crate::models::restaurant::{CreateRestaurant, Restaurant};
use crate::schema;
use actix_web::web;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub async fn create_restaurant(
    pool: web::Data<DbPool>,
    restaurant: CreateRestaurant,
    user_id: Uuid,
) -> Result<Restaurant, Error> {
    let connection = &mut pool.get().expect("couldn't get db connection from pool");
    let new_restaurant = Restaurant {
        id: Uuid::new_v4(),
        name: restaurant.name,
        location: restaurant.location,
        user_id,
        rating: None,
    };
    diesel::insert_into(schema::restaurants::table)
        .values(&new_restaurant)
        .execute(connection)?;

    let created_restaurant = schema::restaurants::table
        .filter(schema::restaurants::id.eq(new_restaurant.id))
        .first::<Restaurant>(connection)?;
    Ok(created_restaurant)
}

pub async fn delete_all_restaurants(pool: web::Data<DbPool>) -> Result<(), Error> {
    let connection = &mut pool.get().expect("couldn't get db connection from pool");
    println!("Deleting all restaurants from the database");
    let results = schema::restaurants::table.load::<Restaurant>(connection)?;
    for restaurant in results {
        println!(
            "ID: {}, Name: {}, Location: {}",
            restaurant.id, restaurant.name, restaurant.location
        );
    }

    // Delete all restaurants from the database
    diesel::delete(schema::restaurants::table).execute(connection)?;
    Ok(())
}
