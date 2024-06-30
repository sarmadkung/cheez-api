use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Queryable, Selectable,Insertable,Identifiable, Debug)]
#[diesel(table_name = crate::schema::restaurants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Restaurant {
    pub id: Uuid,
    pub name: String,
    pub location: String,
    pub rating: f64,
}


#[derive(Debug,Serialize,Deserialize)]
pub struct CreateRestaurant {
    pub name: String,
    pub location: String,
}
