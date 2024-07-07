use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub about: String,
    pub rating: f64,
    pub menu_id: Uuid,
    pub restaurant_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub price: f64,
    pub about: String,
    pub rating: f64,
    pub menu_id: Uuid,
    pub restaurant_id: Uuid,
}
