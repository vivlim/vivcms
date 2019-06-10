
use diesel::prelude::*;
use super::models;
use super::schema;

pub fn establish_connection() -> SqliteConnection {
    let database_url = "data.sqlite";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(user: models::NewUser) {
    let conn = establish_connection();
    diesel::insert_into(schema::users::table)
        .values(&user)
        .execute(&conn)
        .expect("error creating new user");
}

pub fn get_user_from_name(name: &String) -> QueryResult<models::User> {
    use schema::users::dsl::*;
    let conn = establish_connection();
    users.filter(username.eq(name))
        .get_result::<models::User>(&conn)
}

pub fn get_user_from_id(id: i32) -> QueryResult<models::User> {
    use schema::users::dsl::*;
    let conn = establish_connection();
    users.filter(id.eq(id))
        .get_result::<models::User>(&conn)
}