#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

extern crate askama; // for the Template trait and custom derive macro
extern crate spongedown;

use askama::Template;
use spongedown::parse;

use rocket::response::content;
use rocket::request::Form;

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
   

#[get("/")]
fn index() -> content::Html<std::string::String> {
    let hello = HelloTemplate { name: "world" }; // instantiate your struct
    let in_html = hello.render().unwrap(); // then render it.
    content::Html(parse(&in_html).unwrap())
}

#[get("/post")]
fn post_page() -> content::Html<std::string::String> {
    let page = auth::LoginTemplate {};
    content::Html(page.render().unwrap())
}

#[post("/post", data = "<input>")]
fn post_handle(input: Form<auth::LoginForm>) -> String {
    return format!("{} {}", input.username, input.password);
}

fn main() {
    rocket::ignite().mount("/", routes![index, auth::login_page, auth::login_handle, auth::create_user_debug]).launch();
}
