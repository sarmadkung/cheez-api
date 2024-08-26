use actix_web::web;
use cheez_api::db_conn::{establish_connection_pool, DbPool};
use cheez_api::services::restaurant::delete_all_restaurants;
use cheez_api::{
    models::{restaurant::CreateRestaurant, user::CreateUser},
    services::user::{create_user, delete_all_users},
    services::{auth::hash_password, restaurant::create_restaurant},
};
use std::time::Instant;
use tokio;
extern crate fakeit;
use fakeit::{contact, name};

const USERS_COUNT: i32 = 0;

pub async fn clean_restaurants(pool: web::Data<DbPool>) {
    let start_time = Instant::now();
    println!("Started purging restaurants");

    if let Err(e) = delete_all_restaurants(pool).await {
        println!("Error purging restaurants: {:?}", e);
    }

    let duration = start_time.elapsed();
    println!("Finished purging restaurants. Time taken: {:?}", duration);
}

pub async fn clean_users(pool: web::Data<DbPool>) {
    let start_time = Instant::now();
    println!("Started purging users");

    if let Err(e) = delete_all_users(pool).await {
        println!("Error purging users: {:?}", e);
    }

    let duration = start_time.elapsed();
    println!("Finished purging users. Time taken: {:?}", duration);
}

pub async fn seed_users(pool: web::Data<DbPool>) {
    let password = hash_password("password123");
    let password2 = hash_password("hamza123");
    let password3 = hash_password("tiyyab123");

    let start_time = Instant::now();
    println!("Started seeding users");

    let users: [CreateUser; 4] = [
        CreateUser {
            first_name: "Muhammad Sarmad".to_string(),
            last_name: "Sarmad".to_string(),
            email: Some("muhammadsarmad24@gmail.com".to_string()),
            phone: Some("03421464075".to_string()),
            role: "OWNER".to_string(),
            password,
        },
        CreateUser {
            first_name: "hamza".to_string(),
            last_name: "mukhtar".to_string(),
            email: Some("hamzamukhtar292@gmail.com".to_string()),
            phone: Some("03455334292".to_string()),
            role: "USER".to_string(),
            password: password2.clone(),
        },
        CreateUser {
            first_name: "tiyyab".to_string(),
            last_name: "ali".to_string(),
            email: Some("tiyyabali@gmail.com".to_string()),
            phone: Some("1234567890".to_string()),
            role: "ADMIN".to_string(),
            password: password3,
        },
        CreateUser {
            first_name: "Muhammad Sarmad".to_string(),
            last_name: "Sarmad".to_string(),
            email: Some("ansi@gmail.com".to_string()),
            phone: Some("03421464071".to_string()),
            role: "OWNER".to_string(),
            password: password2.clone(),
        },
    ];

    for usr in users {
        let user_start_time = Instant::now();
        match create_user(pool.clone(), usr).await {
            Ok(user) => {
                let user_duration = user_start_time.elapsed();
                println!(
                    "{:?} {:?} joined the app. Time taken: {:?}",
                    user.first_name, user.last_name, user_duration
                );

                let restaurant_start_time = Instant::now();
                let restaurant = CreateRestaurant {
                    name: "Cheez".to_string(),
                    location: "Lahore".to_string(),
                };
                match create_restaurant(pool.clone(), restaurant, user.id).await {
                    Ok(restaurant) => {
                        let restaurant_duration = restaurant_start_time.elapsed();
                        println!(
                            "Restaurant: {:?} created BY {:?}. Time taken: {:?}",
                            restaurant.name, user.first_name, restaurant_duration
                        );
                    }
                    Err(e) => println!("Error creating restaurant: {:?}", e),
                }
            }
            Err(e) => println!("Error creating user: {:?}", e),
        }
    }

    for _ in 0..USERS_COUNT {
        let user_start_time = Instant::now();
        let password = hash_password("password123");
        let user = CreateUser {
            first_name: name::first(),
            last_name: name::last(),
            email: Some(contact::email().to_string()),
            phone: Some(contact::phone().to_string()),
            role: "USER".to_string(),
            password,
        };
        match create_user(pool.clone(), user).await {
            Ok(user) => {
                let user_duration = user_start_time.elapsed();
                println!(
                    "{:?} {:?} joined the app. Time taken: {:?}",
                    user.first_name, user.last_name, user_duration
                );
            }
            Err(e) => println!("Error creating user: {:?}", e),
        }
    }

    let total_duration = start_time.elapsed();
    println!(
        "Finished seeding users. Total time taken: {:?}",
        total_duration
    );
}

#[tokio::main]
async fn main() {
    let pool = web::Data::new(establish_connection_pool());

    clean_restaurants(pool.clone()).await;
    clean_users(pool.clone()).await;
    seed_users(pool.clone()).await;
}
