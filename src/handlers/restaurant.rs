use crate::db_conn::DbPool;
use crate::models::restaurant::{CreateRestaurant, UpdateRestaurant};
use crate::services::restaurant::{
    create_restaurant, delete_restaurant, get_restaurant, get_restaurants, update_restaurant,
};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RestaurantIdRequestPath {
    id: Uuid,
}

pub async fn restaurants(pool: web::Data<DbPool>) -> impl Responder {
    match get_restaurants(pool).await {
        Ok(restaurants) => HttpResponse::Ok().json(restaurants),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error getting restaurants: {:?}", e))
        }
    }
}

pub async fn my_restaurants(pool: web::Data<DbPool>) -> impl Responder {
    match get_restaurants(pool).await {
        Ok(restaurants) => HttpResponse::Ok().json(restaurants),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error getting restaurants: {:?}", e))
        }
    }
}

pub async fn restaurant(
    pool: web::Data<DbPool>,
    path: web::Path<RestaurantIdRequestPath>,
) -> impl Responder {
    match get_restaurant(pool, path.id).await {
        Ok(restaurant) => HttpResponse::Ok().json(restaurant),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error getting restaurant: {:?}", e))
        }
    }
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

pub async fn handle_update_restaurant(
    pool: web::Data<DbPool>,
    path: web::Path<RestaurantIdRequestPath>,
    restaurant_data: web::Json<UpdateRestaurant>,
) -> impl Responder {
    match update_restaurant(pool, path.id, restaurant_data).await {
        Ok(restaurant) => HttpResponse::Ok().json(restaurant),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error updating restaurant: {:?}", e))
        }
    }
}

pub async fn handle_delete(
    pool: web::Data<DbPool>,
    path: web::Path<RestaurantIdRequestPath>,
) -> impl Responder {
    match delete_restaurant(pool, path.id).await {
        Ok(restaurant) => HttpResponse::Ok().json(restaurant),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error deleting restaurant: {:?}", e))
        }
    }
}
