use cheez_api::{
    models::{
        restaurant::CreateRestaurant,
        user::{CreateUser, User},
    },
    services::{auth::hash_password, restaurant::create_restaurant, user::create_user},
};
use tokio;
extern crate fakeit;
use fakeit::contact;
use fakeit::name;

const USERS_COUNT: i32 = 500;

pub async fn seed_users() {
    let password = hash_password("password123");
    let users: [CreateUser; 1] = [CreateUser {
        first_name: "Muhammad Sarmad".to_string(),
        last_name: "Sarmad".to_string(),
        email: Some("muhammadsarmad24@gmail.com").map(|s| s.to_string()),
        phone: Some("03421464075").map(|s| s.to_string()),
        role: "ADMIN".to_string(),
        password,
    }];

    for usr in users {
        match create_user(usr).await {
            Ok(user) => {
                println!("{:?} {:?} joined the app", user.first_name, user.last_name);

                let restaurant = CreateRestaurant {
                    name: "Cheez".to_string(),
                    location: "Lahore".to_string(),
                };
                match create_restaurant(restaurant, user.id).await {
                    Ok(restaurant) => println!(
                        "Restaurant: {:?}  created BY {:?}",
                        restaurant.name, user.first_name
                    ),
                    Err(e) => println!("Error creating restaurant: {:?}", e),
                }
            }
            Err(e) => println!("Error creating user: {:?}", e),
        }
    }

    for _ in 0..USERS_COUNT {
        let password = hash_password("password123");
        let user = CreateUser {
            first_name: name::first(),
            last_name: name::last(),
            email: Some(contact::email()).map(|s| s.to_string()),
            phone: Some(contact::phone()).map(|s| s.to_string()),
            role: "USER".to_string(),
            password,
        };
        match create_user(user).await {
            Ok(user) => println!("{:?} {:?} joined the app", user.first_name, user.last_name),
            Err(e) => println!("Error creating user: {:?}", e),
        }
    }
}

#[tokio::main]
async fn main() {
    seed_users().await;
}
