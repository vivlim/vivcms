
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

pub fn get_user_from_id(uid: i32) -> QueryResult<models::User> {
    use schema::users::dsl::*;
    let conn = establish_connection();
    users.filter(id.eq(uid))
        .get_result::<models::User>(&conn)
}

pub fn create_post(post: models::NewPost) -> QueryResult<models::Post> {
    use schema::posts::dsl::*;
    let conn = establish_connection();
    diesel::insert_into(schema::posts::table)
        .values(&post)
        .execute(&conn)?;
    posts.order(id.desc())
        .first::<models::Post>(&conn)
}

pub fn get_post_by_id(pid: i32) -> QueryResult<models::JoinedPost> {
    use schema::posts::dsl::*;
    let conn = establish_connection();
    let post = posts.filter(id.eq(pid))
        .first::<models::Post>(&conn)?;
    get_post_info(post, &conn)
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
    let latest_contents = pc::post_contents.filter(pc::post_id.eq(post.id).and(pc::published.eq(true)))
        .order(pc::post_id.desc())
        .first::<models::PostContents>(conn);
    
    Ok(models::JoinedPost {
        post: post,
        latest_contents: match latest_contents {Err(_) => None, Ok(c) => Some(c)},
        author: user
    })
}

pub fn create_post_contents(post: models::Post, contents: models::NewPostContents) -> QueryResult<models::PostContents> {
    use schema::post_contents::dsl::*;
    let conn = establish_connection();

    // Get the current information about this post so we can know what the latest content revision number is
    let post_info = get_post_info(post, &conn)?;
    let next_revision_number = match post_info.latest_contents {
        Some(latest_contents) => latest_contents.revision + 1,
        None => 0 // no previous contents, so just start at 0.
    };

    let insertion = models::NewPostContentsInsertion {
        post_id: post_info.post.id,
        revision: next_revision_number,
        title: contents.title,
        body: contents.body,
        published: contents.published
    };

    diesel::insert_into(schema::post_contents::table)
        .values(&insertion)
        .execute(&conn)?;
    post_contents
        .filter(post_id.eq(insertion.post_id))
        .order(post_id.desc())
        .order(revision.desc())
        .first::<models::PostContents>(&conn)
}