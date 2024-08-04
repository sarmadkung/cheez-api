use crate::db_conn::establish_connection;
use crate::models::user::{CreateUser, User};
use crate::schema;
use crate::services::user::create_user;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};
use uuid::Uuid;

pub async fn users() -> impl Responder {
    let connection = &mut establish_connection();
    let users = schema::users::table
        .load::<User>(connection)
        .expect("Error loading users");
    println!("{:?}", users);
    HttpResponse::Ok().json(users)
}

pub async fn handle_create_user(user_data: web::Json<CreateUser>) -> impl Responder {
    let new_user = CreateUser {
        first_name: user_data.first_name.clone(),
        last_name: user_data.last_name.clone(),
        email: user_data.email.clone(),
        phone: user_data.phone.clone(),
        role: "Admin".to_string(),
        password: user_data.password.clone(),
    };

    let response = match create_user(new_user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating user: {:?}", e)),
    };

    response
}

pub async fn update(user_id: Uuid) -> impl Responder {
    let connection = &mut establish_connection();
    let user = schema::users::table
        .filter(schema::users::id.eq(user_id.clone()))
        .first::<User>(connection)
        .expect("Error loading user");
    diesel::update(&user)
        .set(schema::users::last_name.eq("Ahmad"))
        .execute(connection)
        .expect("Error updating user");
    HttpResponse::Ok().body("User updated")
}
