use diesel::{Queryable,Insertable,Identifiable};
use super::schema::*;

#[derive(Queryable, Identifiable, Associations, Debug)]
pub struct Board {
    pub id: i32,
    pub title: String,
    pub details: String
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(Board)]
pub struct Thread {
    pub id: i32,
    pub board_id: i32
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(Thread)]
pub struct Post {
    pub id: i32,
    pub author: i32,
    pub thread_id: i32,
    pub created: i32
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(Post)]
pub struct PostContent {
    pub id: i32,
    pub post_id: i32,
    pub author_id: i32,
    pub title: String,
    pub body: String,
    pub created: i32,
    pub is_published: i32,
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
#[table_name="boards"]
pub struct NewBoard {
    pub title: String,
    pub details: String,
}

#[derive(Insertable)]
#[table_name="threads"]
pub struct NewThread {
    pub board_id: i32,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub author_id: i32,
    pub thread_id: i32,
    pub created: i32
}

pub struct NewPostContents {
    pub post_id: i32,
    pub author_id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name="post_contents"]
pub struct NewPostContentInsertion {
    pub post_id: i32,
    pub author_id: i32,
    pub title: String,
    pub body: String,
    pub created: i32,
    pub is_published: i32
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub pass_sha: &'a str,
    pub salt: &'a str
}