use diesel::prelude::*;
use diesel::insert_into;
use crate::models::schema::users::dsl::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub pseudo: String,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserForm {
    pub pseudo: String,
}

impl UserForm {
    pub fn insert(&self, conn: &mut SqliteConnection) -> QueryResult<usize> {
        insert_into(users).values(self).execute(conn)
    }
}