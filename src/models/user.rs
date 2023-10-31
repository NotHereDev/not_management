use cfg_if::cfg_if;

#[cfg(feature = "ssr")]
use diesel::prelude::*;
#[cfg(feature = "ssr")]
use diesel::insert_into;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable))]
#[cfg_attr(feature = "ssr", diesel(table_name = super::schema::users))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct User {
    pub id: i32,
    pub pseudo: String,
}

impl User {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            pub fn all(conn: &mut SqliteConnection) -> QueryResult<Vec<Self>> {
                use crate::models::schema::users::dsl::*;

                users.load(conn)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = super::schema::users))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct UserForm {
    pub pseudo: String,
}

impl UserForm {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            pub fn insert(&self, conn: &mut SqliteConnection) -> QueryResult<usize> {
                use crate::models::schema::users::dsl::*;

                insert_into(users).values(self).execute(conn)
            }
        }
    }

}