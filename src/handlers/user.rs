use crate::db_conn::{establish_connection, DbPool};
use crate::models::user::{CreateUser, UpdateUser, User};
use crate::schema;
use crate::services::user::{create_user, update_user};
use actix_web::{web, HttpResponse, Responder};
use diesel::RunQueryDsl;
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

pub async fn handle_create_user(
    pool: web::Data<DbPool>,
    user_data: web::Json<CreateUser>,
) -> impl Responder {
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

    let response = match create_user(pool, new_user).await {
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

pub async fn handle_update(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
    user_data: web::Json<UpdateUser>,
) -> impl Responder {
    let updated_user = match update_user(pool, user_data, user_id).await {
        Ok(user) => {
            // Log the updated user
            println!("User updated successfully: {:?}", user);
            HttpResponse::Ok().json(user)
        }
        Err(e) => {
            // Log the error if user update fails
            println!("Error updating user: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error updating user: {:?}", e))
        }
    };

    updated_user
}
