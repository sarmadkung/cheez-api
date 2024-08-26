use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env;

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }
//
// use diesel::prelude::*;

// Type alias for easier usage
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> DbPool {
    dotenv().ok(); // Load environment variables from .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
