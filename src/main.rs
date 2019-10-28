#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;

extern crate askama; // for the Template trait and custom derive macro
extern crate spongedown;

use askama::Template;
use spongedown::parse;

use rocket::response::content;
use rocket::request::Form;
use rocket::http::{Cookies};

mod auth;
mod db;
mod models;
mod schema;

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the templates dir in the crate root
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}
   
#[derive(Template)]
#[template(path = "create_post.html")]
pub struct PostTemplate<'a> {
    pub prev_body: &'a String
}

#[derive(Template)]
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
fn index() -> content::Html<std::string::String> {
    let hello = HelloTemplate { name: "world" }; // instantiate your struct
    let in_html = hello.render().unwrap(); // then render it.
    content::Html(parse(&in_html).unwrap())
}

#[get("/admin/post/new")]
fn new_post_page(mut cookies: Cookies) -> content::Html<std::string::String> {
    match auth::validate_session_cookies(&mut cookies) {
        Err(e) => content::Html(e.error_detail),
        Ok(_user) => {
            let page = PostTemplate {prev_body: &"".to_string()};
            content::Html(page.render().unwrap())
        }
    }
}

#[get("/admin/post/edit/<post_id>")]
fn edit_post_page(mut cookies: Cookies, post_id: i32) -> content::Html<std::string::String> {
    match auth::validate_session_cookies(&mut cookies) {
        Err(e) => content::Html(e.error_detail),
        Ok(_user) => {
            match db::get_post_by_id(post_id) {
                Err(e) => content::Html(format!("{}", e)),
                Ok(post) => {
                    match post.get_published_content() {
                        None => content::Html("Can't edit a post with no contents".to_string()),
                        Some(latest_contents) => {
                            let page = PostTemplate {prev_body: &latest_contents.body};
                            content::Html(page.render().unwrap())
                        }
                    }
                }
            }
        }
    }
}

#[post("/admin/post/new", data = "<input>")]
fn post_handle(mut cookies: Cookies, input: Form<PostForm>) -> String {
    match auth::validate_session_cookies(&mut cookies) {
        Err(e) => e.error_detail,
        Ok(user) => {
            match db::create_new_post(models::NewPost {
                author: user.id
            }, models::NewPostContents {
                title: input.title.clone(),
                body: input.body.clone()
            }) {
                Err(e) => format!("{}", e),
                Ok(_) => "okay you made a post, cool".to_string()
            }
        }

    }
}

#[get("/post/<post_id>")]
fn view_post(post_id: i32) -> content::Html<String> {
    match db::get_post_by_id(post_id) {
        Err(e) => content::Html(format!("this page doesn't exist? {}", e)),
        Ok(post) => {
            match post.get_published_content() {
                None => content::Html("This post has no contents".to_string()),
                Some(latest_content) => {
                    let page = ViewPostTemplate {
                        author: &post.author.username,
                        title: &latest_content.title,
                        body: &latest_content.body
                    };

                    content::Html(page.render().unwrap())
                }
            }
        }
    }
}


fn main() {
    rocket::ignite().mount("/", routes![
        index,
        auth::login_page,
        auth::login_handle,
        auth::create_user_debug,
        new_post_page,
        edit_post_page,
        post_handle,
        view_post
    ]).launch();
}
