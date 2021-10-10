use askama::Template;
use diesel::SqliteConnection;
use rocket::{form::Form, http::CookieJar, response::{self, Redirect, content}};

use crate::{ForumError, auth, storage::{crud, db::{establish_connection, models::{Board, JoinedPost, Post, PostContent, Thread, User}}}};

#[derive(Template, Debug)]
#[template(path = "post_editor.html")]
pub struct PostEditorTemplate<'a> {
    pub mode: Mode,
    pub form_action: &'a str,
}

#[derive(Debug)]
pub enum Mode {
    NewThread(Board),
    Reply(ReplyContext),
    EditPost(JoinedPost)
}

#[derive(Debug)]
pub struct ReplyContext {
    thread: Thread,
    first_post: JoinedPost
}

#[derive(FromForm, Debug)]
pub struct PostEditorForm {
    title: String,
    body: String,
}

pub enum FormMode {
    NewThread(Board),
    Reply(Thread),
    EditPost(Post),
}

#[get("/board/<board_id>/new_thread")]
pub fn new_thread(mut cookies: &CookieJar<'_>, board_id: i32) -> Result<content::Html<std::string::String>, ForumError> {
    let conn = establish_connection();
    let user = auth::validate_session_cookies(&conn, &mut cookies)?;
    let board = crud::boards::get_board_by_id(&conn, board_id)?;
    let page = PostEditorTemplate { mode: Mode::NewThread(board), form_action: &format!("/board/{}/new_thread", board_id)};
    Ok(content::Html(page.render().unwrap()))
}

#[get("/thread/<thread_id>/reply")]
pub fn reply_in_thread(mut cookies: &CookieJar<'_>, thread_id: i32) -> Result<content::Html<std::string::String>, ForumError> {
    let conn = establish_connection();
    let user = auth::validate_session_cookies(&conn, &mut cookies)?;
    let thread = crud::threads::get_thread_by_id(&conn, thread_id)?;
    let first_post = crud::threads::get_thread_first_post(&conn, &thread)?;
    let first_post = crud::posts::get_post_info(&conn, first_post)?;
    let page = PostEditorTemplate {
        mode: Mode::Reply(ReplyContext {
            thread: thread,
            first_post
        }),
        form_action: &format!("/thread/{}/reply", thread_id)
    };
    Ok(content::Html(page.render().unwrap()))
}

#[get("/post/<post_id>/edit")]
pub fn edit_post(mut cookies: &CookieJar<'_>, post_id: i32) -> Result<content::Html<std::string::String>, ForumError> {
    let conn = establish_connection();
    let user = auth::validate_session_cookies(&conn, &mut cookies)?;
    let post = crud::posts::get_post_by_id(&conn, post_id)?;
    let page = PostEditorTemplate { mode: Mode::EditPost(post), form_action: &format!("/post/{}/edit", post_id)};
    Ok(content::Html(page.render().unwrap()))
}

#[post("/board/<board_id>/new_thread", data = "<input>")]
pub fn save_board_page(mut cookies: &CookieJar<'_>, input: Form<PostEditorForm>, board_id: i32) -> Result<Redirect, ForumError> {
    let conn = establish_connection();
    let user = auth::validate_session_cookies(&conn, &mut cookies)?;
    let board = crud::boards::get_board_by_id(&conn, board_id)?;
    save_post(&conn, input, FormMode::NewThread(board), &user)
}

fn save_post(conn: &SqliteConnection, form: Form<PostEditorForm>, mode: FormMode, author: &User) -> Result<Redirect, ForumError> {
    // Save post and get the thread that the post was written to, so we can redirect to it.
    let thread = match mode {
        FormMode::NewThread(board) => {
            crud::threads::create_thread(conn, &board, author, form.title.clone(), form.body.clone())?
        },
        FormMode::Reply(thread) => {
            crud::posts::create_post(conn, &thread, author, form.title.clone(), form.body.clone())?;
            thread
        },
        FormMode::EditPost(_) => todo!(),
    };

    Ok(response::Redirect::temporary(format!("/thread/{}", thread.id)))
}