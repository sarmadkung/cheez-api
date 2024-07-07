use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug)]
#[diesel(table_name = crate::schema::menu)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Menu {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub restaurant_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMenu {
    pub name: String,
    pub restaurant_id: Uuid,
}
