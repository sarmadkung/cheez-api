use actix_web::{ HttpResponse, Responder, web};
use diesel::prelude::*;
use crate::db_conn::establish_connection;
use crate::models::user::{LoginUser, User};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use crate::schema;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn generate_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {

        let expiration = 1720477487;

        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration,
        };

        // Example secret key (replace with your secure secret)
        let secret = "CheezAPI-$&)93!G9879";

        // Encode the JWT token
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;

        Ok(token)
    }
}
pub async fn login(user_data: web::Json<LoginUser>) -> impl Responder {
    let connection = &mut establish_connection();
    match schema::users::table
        .filter(schema::users::email.eq(&user_data.email))
        .filter(schema::users::password.eq(&user_data.password))
        .first::<User>(connection)
    {
        Ok(user) => match Claims::generate_token(&user) {
            Ok(token) => {
                let tkn = serde_json::json!({
                    "token": token,
                });
                HttpResponse::Ok().json(tkn)
            },
            Err(_) => HttpResponse::InternalServerError().body("Failed to generate token"),
        },
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().body("Failed to generate token, Email or Password incorrect")
        },
        Err(err) => {
            HttpResponse::InternalServerError().body("Error loading user")
        }
    }
}