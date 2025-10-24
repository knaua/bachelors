use rocket::outcome::Outcome;
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use rocket::http::{CookieJar, Status};
use rocket::form::Form;
use rocket_dyn_templates::{Template, context};
use serde::Serialize;
use rocket::response::content::RawHtml;
use rocket_db_pools::sqlx::Row;
use crate::db_manager::{get_login, get_user};

// TODO PASSWORDS ARE STORED IN PLAIN TEXT CURRENTLY! CHANGE TO HASH

#[derive(FromForm)]
struct Login<'r> {
    username: &'r str,
    password: &'r str
}

#[derive(Debug, Serialize)]
struct User {
    id: String,
    password: String,
    is_admin: bool
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user = get_user(request.cookies().get_private("user_id").unwrap().value()).await.expect("error"); // TODO better error handling
        Outcome::Success(User {
            id: user.get("id"),
            password: user.get("password"),
            is_admin: user.get("isAdmin"),
        })
    }
}

#[derive(Debug, Serialize)]
struct Admin(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Admin, Self::Error> {
        let user = request.guard::<User>().await;
        if user.as_ref().unwrap().is_admin {
            Outcome::Success(Admin { 0: user.unwrap() }) // TODO Check unwraps and implement error handlers
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

#[macro_export]
macro_rules! session_uri {
    ($($t:tt)*) => (rocket::uri!("/login", $crate::cred:: $($t)*))
}

pub use session_uri as uri;


#[get("/hi")]
fn index(user: User) -> Template {
    Template::render("hi", context! {
        user_id: user.id,
    })
}

#[get("/", rank = 3)]
fn login(_user: User) -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/")]
fn login_page(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("login", &flash)
}

#[get("/admin")] // TODO currently reachable with any user as a proper admin flag check is missing
fn admin(_admin: Admin)-> RawHtml<&'static str> {
    RawHtml(r#"You're an Admin!"#)
}

#[post("/", data = "<login>")]
async fn post_login(jar: &CookieJar<'_>, login: Form<Login<'_>>) -> Result<Redirect, Flash<Redirect>> {
    if get_login(login.username, login.password).await.is_ok() {
        jar.add_private(("user_id", login.username.to_string()));
        Ok(Redirect::to(uri!(index)))
    } else {
        Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid username/password."))
    }
}

#[post("/logout")]
fn logout(jar: &CookieJar<'_>) -> Flash<Redirect> {
    jar.remove_private("user_id");
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, login, login_page, post_login, logout, admin]
}