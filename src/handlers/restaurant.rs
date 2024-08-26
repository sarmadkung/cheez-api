use crate::db_conn::{establish_connection, DbPool};
use crate::models::restaurant::{CreateRestaurant, Restaurant, UpdateRestaurant};
use crate::schema;
use crate::services::restaurant::create_restaurant;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RestaurantIdRequestPath {
    id: Uuid,
}

pub async fn restaurants(pool: web::Data<DbPool>) -> impl Responder {
    let connection = &mut pool.get().expect("couldn't get db connection from pool");
    let restaurants = schema::restaurants::table
        .load::<Restaurant>(connection)
        .expect("Error loading restaurants");
    println!("{:?}", restaurants);
    HttpResponse::Ok().json(restaurants)
}

pub async fn my_restaurants() -> impl Responder {
    let connection = &mut establish_connection();
    let restaurants = schema::restaurants::table
        .filter(schema::restaurants::user_id.eq(Uuid::new_v4()))
        .load::<Restaurant>(connection)
        .expect("Error loading restaurants");
    HttpResponse::Ok().json(restaurants)
}

pub async fn restaurant(path: web::Path<RestaurantIdRequestPath>) -> impl Responder {
    let connection = &mut establish_connection();
    let restaurant = schema::restaurants::table
        .filter(schema::restaurants::id.eq(path.id))
        .load::<Restaurant>(connection)
        .expect("Error loading restaurants");
    HttpResponse::Ok().json(restaurant)
}

pub async fn handle_create_restaurant(
    pool: web::Data<DbPool>,
    restaurant_data: web::Json<CreateRestaurant>,
) -> impl Responder {
    let new_restaurant = CreateRestaurant {
        name: restaurant_data.name.clone(),
        location: restaurant_data.location.clone(),
    };
    match create_restaurant(pool, new_restaurant, Uuid::new_v4()).await {
        Ok(restaurant) => HttpResponse::Ok().json(restaurant),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error creating restaurant: {:?}", e))
        }
    }
}

pub async fn update(
    path: web::Path<RestaurantIdRequestPath>,
    restaurant_data: web::Json<UpdateRestaurant>,
) -> impl Responder {
    let connection = &mut establish_connection();
    let restaurant = schema::restaurants::table
        .filter(schema::restaurants::id.eq(path.id.clone()))
        .first::<Restaurant>(connection)
        .expect("Error loading restaurant");
    diesel::update(&restaurant)
        .set(schema::restaurants::name.eq(restaurant_data.name.clone()))
        .execute(connection)
        .expect("Error updating restaurant");
    diesel::update(&restaurant)
        .set(schema::restaurants::location.eq(restaurant_data.location.clone()))
        .execute(connection)
        .expect("Error updating restaurant");
    HttpResponse::Ok().body("Restaurant updated")
}

pub async fn delete(path: web::Path<RestaurantIdRequestPath>) -> impl Responder {
    let connection = &mut establish_connection();
    let restaurant = schema::restaurants::table
        .filter(schema::restaurants::id.eq(path.id.clone()))
        .first::<Restaurant>(connection)
        .expect("Error loading restaurant");
    diesel::delete(&restaurant)
        .execute(connection)
        .expect("Error deleting restaurant");
    HttpResponse::Ok().body("Restaurant deleted")
}
