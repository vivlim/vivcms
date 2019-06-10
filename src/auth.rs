extern crate sha2;
extern crate hex;
use rocket::response::content;
use rocket::request::Form;
use rocket::request::FromForm;
use askama::Template;
use sha2::{Sha512, Digest};
use diesel;

use super::db;
use super::models::{NewUser,User};

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

pub struct AuthError {
    pub attempted_username: String,
    pub error_detail: String
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} (when trying to sign in as {})", self.error_detail, self.attempted_username)
    }
}

#[get("/login")]
pub fn login_page() -> content::Html<std::string::String> {
    let page = LoginTemplate {};
    content::Html(page.render().unwrap())
}

#[post("/login", data = "<input>")]
pub fn login_handle(input: Form<LoginForm>) -> String {
    match validate_login_attempt(&input.username, &input.password) {
        Err(e) => format!("Couldn't log in: {}", e),
        Ok(user) => {
            format!("one day you would be logged in, {} {}", input.username, input.password)
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

    db::create_user(new_user);
    
    format!("Added user {}", name)
}

fn create_salted_pwhash(pass: &String, salt: &String) -> String {
    let mut hasher = Sha512::new();
    hasher.input(format!("{}{}", pass, salt));
    hex::encode(hasher.result())
}

fn validate_login_attempt(name: &String, pass: &String) -> Result<User, AuthError> {
    // get the user row
    match db::get_user_from_name(name) {
        Err(e) => Err(AuthError{
            attempted_username: name.clone(),
            error_detail: format!("Couldn't find user {}", e)
        }),
        Ok(user) => {
            let pass_sha = create_salted_pwhash(&pass, &user.salt);

            if pass_sha.eq(&user.pass_sha){
                return Ok(user);
            }
            return Err(AuthError{
                attempted_username: name.clone(),
                error_detail: String::from("Incorrect password.")
            });
        }
    }

}