use diesel::prelude::*;
use crate::storage::db::models::NewPost;
use crate::storage::db::models::NewPostContentInsertion;
use crate::storage::db::models::Post;
use crate::storage::db::models::Thread;
use crate::storage::db::models::User;

use crate::storage::db::models;
use crate::storage::db::schema;

/// Create a post with provided content
pub fn create_post(conn: &SqliteConnection, thread: &Thread, author: &User, title: String, body: String) -> QueryResult<models::Post> {
    let post = NewPost {
        author_id: author.id,
        thread_id: thread.id,
        created: 0,
    };
    let post = {
        use schema::posts::dsl::*;
        diesel::insert_into(schema::posts::table)
            .values(&post)
            .execute(conn)?;
        posts.order(id.desc())
            .first::<models::Post>(conn)?
    };
    create_post_contents(conn, &post, author, title, body)?;
    Ok(post)
}

/// Create new post content associated with an existing post
pub fn create_post_contents(conn: &SqliteConnection, post: &Post, author: &User, title: String, body: String) -> QueryResult<models::PostContent> {
    let post_content = NewPostContentInsertion {
        post_id: post.id,
        author_id: author.id,
        title,
        body,
        created: 0,
        is_published: 1 // no drafts yet
    };
    {
        use schema::post_contents::dsl::*;
        diesel::insert_into(schema::post_contents::table)
            .values(&post_contents)
            .execute(conn)?;
        post_contents.order(id.desc())
            .first::<models::PostContent>(conn)
    }
}


pub fn get_post_by_id<'a>(conn: &SqliteConnection, pid: i32) -> QueryResult<models::JoinedPost> {
    use schema::posts::dsl::*;
    let post = posts.filter(id.eq(pid))
        .first::<models::Post>(conn)?;
    get_post_info(post, &conn)
}

pub fn get_posts(conn: &SqliteConnection) -> QueryResult<Vec<models::JoinedPost>> {
    use schema::posts::dsl::*;
    let result = posts.order(id.desc())
        .load::<models::Post>(conn)?;

    Ok(result.into_iter()
        .map(|p| get_post_info(p, &conn)) // Transform all posts into joined posts.
        .filter_map(|p| p.ok()) // Remove any posts that failed to be transformed
        .collect::<Vec<models::JoinedPost>>())
}

fn get_post_info(post: models::Post, conn: &SqliteConnection) -> QueryResult<models::JoinedPost> {
    // Get more information about a post (author, contents)
    // Implemented as multiple queries because it's not immediately obvious how to do this join
    // and this is probably performant enough at the scale it will be used

    use schema::users::dsl as users;
    let user = users::users.filter(users::id.eq(post.author_id))
        .first::<models::User>(conn)?;

    let contents = models::PostContent::belonging_to(&post)
        .load::<models::PostContent>(conn)?;

    Ok(models::JoinedPost {
        post: post,
        contents: contents,
        author: user
    })
}