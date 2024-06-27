use diesel::prelude::*;


#[derive(Queryable, Selectable,Insertable,Identifiable, Debug)]
#[diesel(table_name = crate::schema::restaurants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Restaurant {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub rating: i32,
}
