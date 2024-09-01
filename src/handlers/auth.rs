use crate::db_conn::establish_connection;
use crate::models::user::{LoginUser, User};
use crate::schema;
use crate::services::auth::{verify_password, JWTClaims};
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn login(user_data: web::Json<LoginUser>) -> impl Responder {
    // Log the received user data
    println!("Received user data: {:?}", user_data);

    let connection = &mut establish_connection();

    match schema::users::table
        .filter(schema::users::email.eq(&user_data.email))
        // .filter(schema::users::password.eq(&user_data.password))
        .first::<User>(connection)
    {
        Ok(user) => {
            println!("User found: {:?}", user);
            let password = user_data.password.clone();
            let user_password = user.password.clone();
            if verify_password(&user_password, &password) {
                match JWTClaims::generate_token(&user) {
                    Ok(token) => {
                        println!("Generated token: {}", token);
                        let tkn = serde_json::json!({
                            "token": token,
                        });
                        HttpResponse::Ok().json(tkn)
                    }
                    Err(e) => {
                        println!("Error generating token: {:?}", e);
                        HttpResponse::InternalServerError().body("Failed to generate token")
                    }
                }
            } else {
                HttpResponse::Unauthorized().body("Invalid password")
            }
        }
        Err(diesel::result::Error::NotFound) => {
            println!("User not found with email: {}", &user_data.email);
            HttpResponse::NotFound().body("Email or Password incorrect")
        }
        Err(err) => {
            println!("Error querying user: {:?}", err);
            HttpResponse::InternalServerError().body("Error loading user")
        }
    }
}
