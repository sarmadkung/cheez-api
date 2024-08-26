use crate::db_conn::DbPool;
use crate::models::user::CreateUser;
use crate::models::user::UpdateUser;
use crate::models::user::User;
use crate::schema;
use crate::schema::users;
use actix_web::web;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub async fn create_user(pool: web::Data<DbPool>, user: CreateUser) -> Result<User, Error> {
    use crate::schema::users;

    let connection = &mut pool.get().expect("couldn't get db connection from pool");

    let new_user = User {
        id: Uuid::new_v4(),
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
        phone: user.phone,
        role: user.role,
        password: user.password,
    };

    // Insert the new user into the database
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)?;

    // Fetch the newly created user from the database
    let created_user = users::table
        .filter(users::id.eq(new_user.id)) // Assuming `id` is a field in your `User` model
        .first::<User>(connection)?;

    // Return the created user
    Ok(created_user)
}

pub async fn update_user(
    pool: web::Data<DbPool>,
    update: web::Json<UpdateUser>,
    user_id: web::Path<Uuid>,
) -> Result<User, Error> {
    // Get a connection from the pool
    let connection = &mut pool.get().expect("couldn't get db connection from pool");

    let user = schema::users::table
        .filter(schema::users::id.eq(user_id.clone()))
        .first::<User>(connection)?;

    // Update first_name
    if let Some(firstname) = update.first_name.clone() {
        diesel::update(&user)
            .set(users::first_name.eq(firstname))
            .execute(connection)?;
    }
    if let Some(lastname) = update.last_name.clone() {
        diesel::update(&user)
            .set(users::last_name.eq(lastname))
            .execute(connection)?;
    }

    // Return the updated user
    let updated_user = schema::users::table
        .filter(schema::users::id.eq(user_id.clone()))
        .first::<User>(connection)?;

    Ok(updated_user)
}

pub async fn delete_all_users(pool: web::Data<DbPool>) -> Result<(), Error> {
    use crate::schema::users;

    let connection = &mut pool.get().expect("couldn't get db connection from pool");
    println!("Deleting all users from the database");
    let results = schema::users::table.load::<User>(connection)?;
    for user in results {
        println!(
            "ID: {}, Name: {}, Email: {}",
            user.id, user.first_name, user.last_name
        );
    }

    // Delete all users from the database
    diesel::delete(users::table).execute(connection)?;

    Ok(())
}
