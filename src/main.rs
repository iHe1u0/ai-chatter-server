#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, FromForm)]
struct User<'a> {
    id: usize,
    email: &'a str,
    password: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse<'a> {
    code: usize,
    msg: Cow<'a, str>,
}

// #[post("/login", data = "<info>")]
// fn login(info: Form<User<'_>>) -> Json<LoginResponse<'_>> {
#[get("/login?<email>&<password>")]
fn login<'a>(email: &'a str, password: &'a str) -> Json<LoginResponse<'a>> {
    let admin = User {
        id: 0,
        email: "admin",
        password: "123",
    };

    let response = if email == admin.email && password == admin.password {
        LoginResponse {
            code: 0,
            msg: Cow::Borrowed("Authenticated"),
        }
    } else {
        LoginResponse {
            code: 401,
            msg: Cow::Borrowed("Authenticate Failed"),
        }
    };

    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login])
}
