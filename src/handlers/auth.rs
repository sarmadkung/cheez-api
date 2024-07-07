use actix_web::{ HttpResponse, Responder, web};
use diesel::prelude::*;
use crate::db_conn::establish_connection;
use crate::models::user::{LoginUser, User};
use crate::schema;


pub async fn login(user_data: web::Json<LoginUser>) -> impl Responder {
    let connection = &mut establish_connection();
    match schema::users::table
        .filter(schema::users::email.eq(&user_data.email))
        .filter(schema::users::password.eq(&user_data.password))
        .first::<User>(connection)
    {
        Ok(user) => HttpResponse::Ok().json("Login Successfully"),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().body("Email or Password incorrect")
        },
        Err(err) => {
            HttpResponse::InternalServerError().body("Error loading user")
        }
    }
}