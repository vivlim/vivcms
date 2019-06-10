use diesel::{Queryable,Insertable};
use super::schema::users;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub author: i32,
    pub title: String,
    pub body: String,
    pub published: bool
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pass_sha: String,
    pub salt: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub pass_sha: &'a str,
    pub salt: &'a str
}