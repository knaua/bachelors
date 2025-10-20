use rocket::outcome::{IntoOutcome, Outcome};
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use rocket::http::{CookieJar, Status};
use rocket::form::Form;
use rocket_dyn_templates::{Template, context};
use serde::Serialize;
use rocket::response::content::RawHtml;
use rocket_db_pools::sqlx::Row;
use crate::db_manager::{get_login, get_user};
use crate::PasteId;

// TODO PASSWORDS ARE STORED IN PLAIN TEXT CURRENTLY! CHANGE TO HASH VALUE WHEN DONE TESTING

#[derive(FromForm)]
struct Login<'r> {
    username: &'r str,
    password: &'r str
}

#[derive(Debug, Serialize)]
struct User(String);

#[derive(Debug, Serialize)]
struct Users {
    id: String,
    password: String,
    is_admin: bool
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {

        //let user = request.local_cache_async(async { |id| get_user(id)}).await;
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(User)
            .or_forward(Status::Unauthorized)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Users {
    type Error = std::convert::Infallible;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        println!("{:#?}", request.cookies().get_private("user_id").unwrap());
        let u = get_user(request.cookies().get_private("user_id").unwrap().value()).await.expect("error");
        request::Outcome::Success(Users {
            id: u.get("id"),
            password: u.get("password"),
            is_admin: u.get("isAdmin"),
        })
    }
}

fn is_admin(user: Users) -> bool {
    user.is_admin
}

#[derive(Debug, Serialize)]
struct Admin(Users);


#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Admin, Self::Error> {
        let user = request.guard::<Users>().await;
        if user.as_ref().unwrap().is_admin { // Some sort of checking for an admin flag needs to be done here, currently this should just make every user an admin
            Outcome::Success(Admin { 0: user.unwrap() })
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
        user_id: user,
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
        let x = "test".to_string();
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