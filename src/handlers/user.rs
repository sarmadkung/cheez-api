use actix_web::{ web, HttpResponse, Responder};
use diesel::{ExpressionMethods, RunQueryDsl};
use uuid::Uuid;
use diesel::prelude::*;
use crate::db_conn::establish_connection;
use crate::models::user::{CreateUser, User};
use crate::schema;

pub async fn users() -> impl Responder {
    let connection = &mut establish_connection();
    let users = schema::users::table
        .load::<User>(connection)
        .expect("Error loading users");
    println!("{:?}", users);
    HttpResponse::Ok().json(users)
}

pub async fn create_user(user_data: web::Json<CreateUser>) -> impl Responder {
    let connection = &mut establish_connection();
    let new_user = User {
        id: Uuid::new_v4(),
        first_name: user_data.first_name.clone(),
        last_name: user_data.last_name.clone(),
        email: user_data.email.clone(),
        password: user_data.password.clone()
    };
    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user");
    HttpResponse::Ok().body("New user added")
}

pub async fn update(user_id:Uuid) -> impl Responder {
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
