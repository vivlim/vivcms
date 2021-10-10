use askama::Template;
use diesel::SqliteConnection;
use rocket::response::content;

use crate::{ForumError, storage::{crud, db::{establish_connection, models::{Board, JoinedPost, Post, Thread}}}};

#[derive(Debug)]
pub struct ThreadRow {
    id: i32,
    thread: Thread,
    first_post: JoinedPost,
}

#[get("/board/<board_id>")]
pub fn view_board_threads(board_id: i32) -> Result<content::Html<std::string::String>, ForumError> {
    let conn = establish_connection();
    let board = crud::boards::get_board_by_id(&conn, board_id).unwrap();


    let threads: Vec<ThreadRow> = crud::threads::get_threads_in_board(&board, &conn)?
        .into_iter().filter_map(|t| load_thread_row(&conn, t))
        .collect();
    let page = ViewBoardThreadsTemplate { board, threads };
    Ok(content::Html(page.render().unwrap()))
}

fn load_thread_row(conn: &SqliteConnection, thread: Thread) -> Option<ThreadRow> {
    let first_post = crud::threads::get_thread_first_post(&conn, &thread).ok()?;
    let first_post = crud::posts::get_post_info(&conn, first_post).ok()?;
    Some(ThreadRow {
        id: thread.id,
        thread,
        first_post,
    })
}


#[derive(Template, Debug)]
#[template(path = "view_board_threads.html")]
pub struct ViewBoardThreadsTemplate {
    pub board: Board,
    pub threads: Vec<ThreadRow>
}