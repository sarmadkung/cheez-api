use diesel::prelude::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable,Insertable,Identifiable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}