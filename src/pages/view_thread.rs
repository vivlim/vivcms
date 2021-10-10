use askama::Template;
use diesel::SqliteConnection;
use rocket::response::content;

use crate::{ForumError, storage::{crud, db::{establish_connection, models::{Board, JoinedPost, Post, Thread}}}};

#[derive(Debug)]
pub struct PostRow {
    id: i32,
    post: JoinedPost,
}

#[get("/thread/<thread_id>")]
pub fn view_thread(thread_id: i32) -> Result<content::Html<std::string::String>, ForumError> {
    let conn = establish_connection();
    let thread = crud::threads::get_thread_by_id(&conn, thread_id)?;
    let board = crud::boards::get_board_by_id(&conn, thread.board_id)?;


    let posts: Vec<PostRow> = crud::threads::get_thread_posts(&conn, &thread)?
        .into_iter().filter_map(|p| load_post_row(&conn, p))
        .collect();
    let title = posts.first().ok_or(ForumError::Unexpected("No first post in thread".to_string()))?.post.contents.title.clone();
    let page = ViewThreadTemplate { thread, title, board, posts };
    Ok(content::Html(page.render().unwrap()))
}

fn load_post_row(conn: &SqliteConnection, post: Post) -> Option<PostRow> {
    let id = post.id;
    let post = crud::posts::get_post_info(&conn, post).ok()?;
    Some(PostRow {
        id: post.post.id,
        post,
    })
}


#[derive(Template, Debug)]
#[template(path = "view_thread.html")]
pub struct ViewThreadTemplate {
    pub thread: Thread,
    pub title: String,
    pub board: Board,
    pub posts: Vec<PostRow>
}