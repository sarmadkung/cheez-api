use crate::models::user::CreateUser;
use crate::{db_conn::establish_connection, models::user::User};
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub async fn create_user(user: CreateUser) -> Result<User, Error> {
    use crate::schema::users;

    let connection = &mut establish_connection();

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
