
use diesel::prelude::*;
use crate::storage::models::Board;
use crate::storage::models::NewBoard;
use crate::storage::models::NewPost;
use crate::storage::models::NewPostContentInsertion;
use crate::storage::models::NewThread;
use crate::storage::models::Post;
use crate::storage::models::Thread;
use crate::storage::models::User;

use super::models;
use super::schema;








/*
pub fn set_post_published_id(post_id: i32, published_content_id: i32) -> QueryResult<usize> {
    use schema::posts::dsl::*;
    let connection = establish_connection();
    // make sure that published_content_id exists, and that it matches post
    let content_with_id = schema::post_contents::dsl::post_contents
        .filter(schema::post_contents::dsl::id.eq(published_content_id))
        .first::<models::PostContent>(&connection);
    
    match content_with_id {
        Err(e) => Err(e),
        Ok(_) => {
            let target_post = posts.filter(id.eq(post_id)).first::<models::Post>(&connection)?;
            diesel::update(&target_post).set(published_content.eq(published_content_id)).execute(&connection)
        }
    }


} */
