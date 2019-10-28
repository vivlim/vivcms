use diesel::{Queryable,Insertable,Identifiable};
use super::schema::*;

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(User, foreign_key="author")]
pub struct Post {
    pub id: i32,
    pub author: i32,
    pub published_content: Option<i32>
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(Post)]
pub struct PostContent {
    pub id: i32,
    pub post_id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Queryable, Identifiable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pass_sha: String,
    pub salt: String
}

pub struct JoinedPost {
    pub post: Post,
    pub contents: Vec<PostContent>,
    pub author: User,
    pub published_content_index: Option<usize>
}

impl JoinedPost {
    pub fn get_published_content(&self) -> Option<&PostContent>{
        match self.published_content_index {
            None => None,
            Some(i) => self.contents.get(i)
        }
    }
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub author: i32
}

pub struct NewPostContents {
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name="post_contents"]
pub struct NewPostContentInsertion {
    pub post_id: i32,
    pub title: String,
    pub body: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub pass_sha: &'a str,
    pub salt: &'a str
}