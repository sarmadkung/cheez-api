use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable,Insertable,Identifiable, Debug)]
#[diesel(table_name = crate::schema::restaurants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Restaurant {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub rating: i32,
}
