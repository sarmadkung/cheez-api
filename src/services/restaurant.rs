use crate::db_conn::establish_connection;
use crate::models::restaurant::{CreateRestaurant, Restaurant};
use crate::schema;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub async fn create_restaurant(
    restaurant: CreateRestaurant,
    user_id: Uuid,
) -> Result<Restaurant, Error> {
    let connection = &mut establish_connection();
    let new_restaurant = Restaurant {
        id: Uuid::new_v4(),
        name: restaurant.name,
        location: restaurant.location,
        user_id,
        rating: None,
    };
    diesel::insert_into(schema::restaurants::table)
        .values(&new_restaurant)
        .execute(connection)?;

    let created_restaurant = schema::restaurants::table
        .filter(schema::restaurants::id.eq(new_restaurant.id))
        .first::<Restaurant>(connection)?;
    Ok(created_restaurant)
}
