
use diesel::prelude::*;
use crate::models::Board;
use crate::models::NewBoard;
use crate::models::NewPost;
use crate::models::NewPostContentInsertion;
use crate::models::NewPostContents;
use crate::models::NewThread;
use crate::models::Post;
use crate::models::Thread;
use crate::models::User;

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

pub fn get_user_from_id(uid: i32) -> QueryResult<models::User> {
    use schema::users::dsl::*;
    let conn = establish_connection();
    users.filter(id.eq(uid))
        .get_result::<models::User>(&conn)
}

pub fn create_board(conn: &SqliteConnection, title: String, details: String) -> QueryResult<models::Board> {
    let new_board = NewBoard {
        title,
        details
    };
    {
        use schema::boards::dsl::*;
        diesel::insert_into(schema::boards::table)
            .values(&new_board)
            .execute(conn)?;
        boards.order(id.desc())
            .first::<models::Board>(conn)
    }
}

pub fn create_thread(conn: &SqliteConnection, board: &Board, author: &User, title: String, body: String) -> QueryResult<models::Thread> {
    let thread = NewThread {
        board_id: board.id
    };
    let thread = {
        use schema::threads::dsl::*;
        diesel::insert_into(schema::threads::table)
            .values(&thread)
            .execute(conn)?;
        threads.order(id.desc())
            .first::<models::Thread>(conn)?
    };
    let post = create_post(conn, &thread, author, title, body);
}

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
}

pub fn create_post_contents(conn: &SqliteConnection, post: &Post, author: &User, title: String, body: String) -> QueryResult<models::Post> {
    let post_contents = NewPostContentInsertion {
        post_id: post.id,
        author_id: author.id,
        title,
        body,
        created: 0,
        is_published: 1 // no drafts yet
    };
    let post = {
        use schema::posts::dsl::*;
        diesel::insert_into(schema::post_contents::table)
            .values(&post_contents)
            .execute(conn)?;
        posts.order(id.desc())
            .first::<models::PostContent>(conn)?
    };
}


pub fn get_post_by_id<'a>(pid: i32) -> QueryResult<models::JoinedPost> {
    use schema::posts::dsl::*;
    let conn = establish_connection();
    let post = posts.filter(id.eq(pid))
        .first::<models::Post>(&conn)?;
    get_post_info(post, &conn)
}

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


}

pub fn get_posts() -> QueryResult<Vec<models::JoinedPost>> {
    use schema::posts::dsl::*;
    let conn = establish_connection();
    let result = posts.order(id.desc())
        .load::<models::Post>(&conn)?;

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
    use schema::post_contents::dsl as pc;
    let user = users::users.filter(users::id.eq(post.author))
        .first::<models::User>(conn)?;

    let contents = models::PostContent::belonging_to(&post)
        .load::<models::PostContent>(conn)?;

    match post.published_content {
        Some(published_content_id) => {
            Ok(models::JoinedPost {
                post: post,
                published_content_index: contents.iter().position(|c| c.id == published_content_id), // map ID to index in contents vector
                contents: contents,
                author: user
            })
        },
        None => {
            Ok(models::JoinedPost {
                post: post,
                contents: contents,
                published_content_index: None,
                author: user
            })
        }
    }
}

pub fn create_post_contents(post: models::Post, contents: models::NewPostContents) -> QueryResult<models::PostContent> {
    use schema::post_contents::dsl::*;
    let conn = establish_connection();

    let insertion = models::NewPostContentInsertion {
        post_id: post.id,
        title: contents.title,
        body: contents.body
    };

    diesel::insert_into(schema::post_contents::table)
        .values(&insertion)
        .execute(&conn)?;

    post_contents
        .filter(post_id.eq(insertion.post_id))
        .order(id.desc())
        .first::<models::PostContent>(&conn)
}


pub fn create_new_post(new_post: models::NewPost, new_post_content: models::NewPostContents) -> QueryResult<models::JoinedPost> {
    let p = create_post(new_post)?;
    let pid = p.id;
    let c = create_post_contents(p, new_post_content)?;
    Ok(get_post_by_id(pid)?)
}