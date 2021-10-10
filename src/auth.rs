extern crate sha2;
extern crate hex;
use rocket::response::content;
use rocket::http::{Cookie, CookieJar};
use rocket::form::Form;
use rocket::form::FromForm;
use askama::Template;
use sha2::{Sha512, Digest};
use diesel::{self, SqliteConnection};
use thiserror::Error;

use crate::storage::crud;
use crate::storage::db::establish_connection;
use crate::storage::db::models::{NewUser,User};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate { }

#[derive(FromForm)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

pub struct AuthCookie {
    pub userid: u32,
    pub username: String,
    pub display: Option<String>,
}

#[derive(Debug, Error)]
pub struct AuthError {
    pub error_detail: String
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.error_detail)
    }
}

impl std::convert::From<std::num::ParseIntError> for AuthError {
    fn from(e: std::num::ParseIntError) -> AuthError {
        AuthError {
            error_detail: format!("Error parsing integer. {}", e)
        }
    }
}

impl std::convert::From<diesel::result::Error> for AuthError {
    fn from(e: diesel::result::Error) -> AuthError {
        AuthError {
            error_detail: format!("Error querying db: {}", e)
        }
    }
}

#[get("/login")]
pub fn login_page() -> content::Html<std::string::String> {
    let page = LoginTemplate {};
    content::Html(page.render().unwrap())
}

#[post("/login", data = "<input>")]
pub fn login_handle(mut cookies: &CookieJar<'_>, input: Form<LoginForm>) -> String {
    let conn = establish_connection();
    match validate_login_attempt(&conn, &input.username, &input.password) {
        Err(e) => format!("Couldn't log in: {}", e),
        Ok(user) => {
            create_session_cookies(&mut cookies, &user);
            format!("you're logged in now, {}. <a href=\"/\">continue home</a>", input.username)
        }
    }
}

#[get("/createuser/<name>/<pass>")]
pub fn create_user_debug(name: String, pass: String) -> String {
    let salt = String::from("saaaaltpls");

    let hash = create_salted_pwhash(&pass, &salt);

    let new_user = NewUser {
        username: &name,
        pass_sha: &hash,
        salt: &salt
    };

    let conn = establish_connection();
    crud::user::create_user(&conn, new_user);
    
    format!("Added user {}", name)
}

fn create_salted_pwhash(pass: &String, salt: &String) -> String {
    create_hash_string(&format!("{}{}", pass, salt))
}

fn create_hash_string(input: &String) -> String {
    let mut hasher = Sha512::new();
    hasher.input(input);
    hex::encode(hasher.result())
}

fn validate_login_attempt(conn: &SqliteConnection, name: &String, pass: &String) -> Result<User, AuthError> {
    // get the user row
    match crud::user::get_user_from_name(conn, name) {
        Err(e) => Err(AuthError{
            error_detail: format!("Couldn't find user {}", e)
        }),
        Ok(user) => {
            let pass_sha = create_salted_pwhash(&pass, &user.salt);

            if pass_sha.eq(&user.pass_sha){
                return Ok(user);
            }
            return Err(AuthError{
                error_detail: String::from("Incorrect password.")
            });
        }
    }
}

pub fn create_session_cookies(cookies: &CookieJar<'_>, user: &User) {
    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
    cookies.add_private(Cookie::new("pw2hash", create_hash_string(&user.pass_sha)));
}

pub fn remove_session_cookies(cookies: &CookieJar<'_>) {
    cookies.remove_private(Cookie::named("user_id"));
    cookies.remove_private(Cookie::named("pw2hash"));
}

pub fn validate_session_cookies(conn: &SqliteConnection, cookies: &CookieJar<'_>) -> Result<User, AuthError> {
    match (cookies.get_private("user_id"), cookies.get_private("pw2hash")) {
        (Some(cookie_user_id_str), Some(cookie_pw2hash)) => {
            let cookie_user_id = cookie_user_id_str.value().parse::<i32>()?;
            let db_user = crud::user::get_user_from_id(conn, cookie_user_id)?;
            let db_pw2hash = create_hash_string(&db_user.pass_sha);

            if db_pw2hash.eq(&cookie_pw2hash.value()) {
                return Ok(db_user);
            }
            else {
                return Err(AuthError {
                    error_detail: String::from("Password in cookie doesn't match database.")
                })
            }
        }
        _ => Err(AuthError { // either id or hash cookie are missing
            error_detail: "Missing auth cookie.".to_string()
        }),
    }
}