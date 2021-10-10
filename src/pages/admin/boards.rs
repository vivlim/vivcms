use rocket::response::content;
use rocket::form::Form;
use rocket::http::{CookieJar};
use crate::storage::db::{self, establish_connection};
use crate::storage::crud;
use askama::Template;
use crate::auth;

#[derive(Template)]
#[template(path = "edit_board.html")]
pub struct EditBoardTemplate {
    pub existing_board: Option<db::models::Board>,
    pub form_action: &'static str
}

#[derive(FromForm, Debug)]
pub struct SaveBoardForm {
    pub title: String,
    pub details: String,
    pub id: Option<u32>
}

#[get("/admin/board/new")]
pub fn new_board_page(mut cookies: &CookieJar<'_>) -> content::Html<std::string::String> {
    let conn = establish_connection();
    match auth::validate_session_cookies(&conn, &mut cookies) {
        Err(e) => content::Html(e.error_detail),
        Ok(_user) => {
            let page = EditBoardTemplate {existing_board: None, form_action: "/admin/board/save"};
            content::Html(page.render().unwrap())
        }
    }
}

#[post("/admin/board/save", data = "<input>")]
pub fn save_board_page(mut cookies: &CookieJar<'_>, input: Form<SaveBoardForm>) -> content::Html<String> {
    let conn = establish_connection();
    match auth::validate_session_cookies(&conn, &mut cookies) {
        Err(e) => content::Html(e.error_detail),
        Ok(_user) => {
            if let Some(id) = input.id {
                return content::Html(format!("editing boards is not implemented. input: {:?}", input))
            }
            else {
                let new_board = crud::boards::create_board(&conn, input.title.clone(), input.details.clone()).unwrap();
                return content::Html(format!("created new board: {:?}", new_board))

            }
        }
    }
}