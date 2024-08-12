use crate::db_conn::establish_connection;
use crate::models::user::{CreateUser, User, UpdateUser};
use crate::schema;
use crate::services::user::create_user;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};
use uuid::Uuid;

pub async fn users() -> impl Responder {
    let connection = &mut establish_connection();

    // Log before loading users
    println!("Attempting to load users from the database...");

    let users = schema::users::table
        .load::<User>(connection)
        .expect("Error loading users");

    // Log the loaded users
    println!("Loaded users: {:?}", users);

    HttpResponse::Ok().json(users)
}

pub async fn handle_create_user(user_data: web::Json<CreateUser>) -> impl Responder {
    // Log the received user data
    println!("Received user data for creation: {:?}", user_data);

    let new_user = CreateUser {
        first_name: user_data.first_name.clone(),
        last_name: user_data.last_name.clone(),
        email: user_data.email.clone(),
        phone: user_data.phone.clone(),
        role: user_data.role.to_string(),
        password: user_data.password.clone(),
    };

    // Log the new user data before creation
    println!("Creating new user: {:?}", new_user);

    let response = match create_user(new_user).await {
        Ok(user) => {
            // Log the created user
            println!("User created successfully: {:?}", user);
            HttpResponse::Ok().json(user)
        }
        Err(e) => {
            // Log the error if user creation fails
            println!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error creating user: {:?}", e))
        }
    };

    response
}

pub async fn update(
    user_id: web::Path<Uuid>,
    user_data: web::Json<UpdateUser>,
) -> impl Responder {
    let connection = &mut establish_connection();

    // Log the user ID for the update
    println!("Attempting to update user with ID: {:?}", user_id);

    let user = schema::users::table
        .filter(schema::users::id.eq(user_id.into_inner())) // Convert Path to Uuid
        .first::<User>(connection)
        .expect("Error loading user");

    // Log the user before updating
    println!("User before update: {:?}", user);

    // Update the user with the data from the request
    diesel::update(&user)
        .set((
            schema::users::first_name.eq(&user_data.first_name),
            schema::users::last_name.eq(&user_data.last_name),
            schema::users::email.eq(&user_data.email),
            schema::users::phone.eq(&user_data.phone),
        ))
        .execute(connection)
        .expect("Error updating user");

    // Log the success of the update
    println!("User updated successfully.");

    HttpResponse::Ok().body("User updated")
}