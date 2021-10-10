#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;

extern crate askama; // for the Template trait and custom derive macro
extern crate spongedown;

use std::io::Cursor;

use rocket::Request;
use rocket::Response;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response;
use storage::db::models::User;
use thiserror::Error;
use askama::Template;

use spongedown::parse;

use rocket::response::content;
use rocket::response::Responder;
use rocket::form::Form;
use rocket::http::{CookieJar};
use storage::db::{self, establish_connection};

use crate::storage::crud;

mod auth;
mod storage;
mod pages;

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the templates dir in the crate root
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}
   
// #[derive(Template)]
// #[template(path = "create_post.html")]
// pub struct PostTemplate {
//     pub existing_post: Option<db::models::JoinedPost>,
//     pub form_action: &'static str
// }

#[derive(Error, Debug)]
pub enum ForumError {
    #[error("db error {0}")]
    DbError(#[from] diesel::result::Error),
    #[error("auth error {0}")]
    AuthError(#[from] auth::AuthError),
    #[error("Unexpected issue: {0}")]
    Unexpected(String)

}

impl<'r> Responder<'r, 'static> for ForumError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let error_msg: String = format!("Error: {:?}", self).into();
        Response::build()
            .header(ContentType::Plain)
            .sized_body(error_msg.len(), Cursor::new(error_msg))
            .status(Status {
                code: 500,
            })
            .ok()
    }

}

#[derive(Template)]
#[template(path = "view_boards.html")]
pub struct ViewBoardsTemplate {
    pub boards: Vec<db::models::Board>,
    pub user: Option<User>
}


#[derive(Template, Debug)]
#[template(path = "view_post.html")]
pub struct ViewPostTemplate<'a> {
    pub title: &'a String,
    pub author: &'a String,
    pub body: &'a String
}

#[derive(FromForm)]
pub struct PostForm {
    pub title: String,
    pub body: String,
}

#[get("/")]
fn index(mut cookies: &CookieJar<'_>) -> content::Html<std::string::String> {
    let conn = establish_connection();
    let user = auth::validate_session_cookies(&conn, &mut cookies).ok();
    let boards = crud::boards::get_boards(&conn).unwrap();
    let page = ViewBoardsTemplate { boards, user };
    content::Html(page.render().unwrap())
}

// #[get("/admin/post/new")]
// fn new_post_page(mut cookies: &CookieJar<'_>) -> content::Html<std::string::String> {
//     match auth::validate_session_cookies(&mut cookies) {
//         Err(e) => content::Html(e.error_detail),
//         Ok(_user) => {
//             let page = PostTemplate {existing_post: None, form_action: "/admin/post/new"};
//             content::Html(page.render().unwrap())
//         }
//     }
// }

// #[get("/admin/post/edit/<post_id>")]
// fn edit_post_page(mut cookies: &CookieJar<'_>, post_id: i32) -> content::Html<std::string::String> {
//     match auth::validate_session_cookies(&mut cookies) {
//         Err(e) => content::Html(e.error_detail),
//         Ok(_user) => {
//             match db::get_post_by_id(post_id) {
//                 Err(e) => content::Html(format!("{}", e)),
//                 Ok(post) => {
//                     let page = PostTemplate {existing_post: Some(post), form_action: "/admin/post/edit"};
//                     content::Html(page.render().unwrap())
//                 }
//             }
//         }
//     }
// }

#[post("/admin/post/new", data = "<input>")]
fn post_handle(mut cookies: &CookieJar<'_>, input: Form<PostForm>) -> String {
    "not implemented yet, need to create boards and threads 1st".to_string()
    // let conn = establish_connection();
    // match auth::validate_session_cookies(&conn, &mut cookies) {
    //     Err(e) => e.error_detail,
    //     Ok(user) => {
    //         match crud::posts::create_post(

    //         //     author_id: user.id
    //         // }, models::NewPostContents {
    //         //     title: input.title.clone(),
    //         //     body: input.body.clone()
    //         ) {
    //             Err(e) => format!("{}", e),
    //             Ok(_) => "okay you made a post, cool".to_string()
    //         }
    //     }

    // }
}

#[get("/post/<post_id>")]
fn view_post(post_id: i32) -> content::Html<String> {
    let conn = establish_connection();
    match crud::posts::get_post_by_id(&conn, post_id) {
        Err(e) => content::Html(format!("this page doesn't exist? {}", e)),
        Ok(post) => {
            content::Html(format!("{:?}", post))
            // match post.get_published_content() {
            //     None => content::Html("This post has no contents".to_string()),
            //     Some(latest_content) => {
            //         let page = ViewPostTemplate {
            //             author: &post.author.username,
            //             title: &latest_content.title,
            //             body: &latest_content.body
            //         };

            //         content::Html(page.render().unwrap())
            //     }
            // }
        }
    }
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        auth::login_page,
        auth::login_handle,
        auth::create_user_debug,
        pages::admin::boards::new_board_page,
        pages::admin::boards::save_board_page,
        pages::view_board_threads::view_board_threads,
        pages::post_editor::new_thread,
        pages::post_editor::reply_in_thread,
        pages::post_editor::edit_post,
        pages::post_editor::save_board_page,
        pages::post_editor::save_reply_page,
        pages::view_thread::view_thread,
        post_handle,
        view_post
    ])
}
