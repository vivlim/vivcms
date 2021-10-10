use diesel::prelude::*;

use crate::storage::db::models;
use crate::storage::db::schema;

pub fn create_user(conn: &SqliteConnection, user: models::NewUser) {
    diesel::insert_into(schema::users::table)
        .values(&user)
        .execute(conn)
        .expect("error creating new user");
}

pub fn get_user_from_name(conn: &SqliteConnection, name: &String) -> QueryResult<models::User> {
    use schema::users::dsl::*;
    users.filter(username.eq(name))
        .get_result::<models::User>(conn)
}

pub fn get_user_from_id(conn: &SqliteConnection, uid: i32) -> QueryResult<models::User> {
    use schema::users::dsl::*;
    users.filter(id.eq(uid))
        .get_result::<models::User>(conn)
}