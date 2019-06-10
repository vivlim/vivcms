use diesel::{Queryable,Insertable};
use super::schema::*;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub author: i32
}

#[derive(Queryable)]
pub struct PostContents {
    pub post_id: i32,
    pub revision: i32,
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

pub struct JoinedPost {
    pub post: Post,
    pub latest_contents: Option<PostContents>,
    pub author: User
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub author: i32
}

pub struct NewPostContents {
    pub title: String,
    pub body: String,
    pub published: bool
}

#[derive(Insertable)]
#[table_name="post_contents"]
pub struct NewPostContentsInsertion {
    pub post_id: i32,
    pub revision: i32,
    pub title: String,
    pub body: String,
    pub published: bool
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub pass_sha: &'a str,
    pub salt: &'a str
}