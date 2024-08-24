#[macro_use]
extern crate rocket;

use rocket::response::content;
use std::borrow::Cow;

use rocket::{response::content::RawJson, serde::json::Json};
use serde::{Deserialize, Serialize};

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

#[get("/")]
fn ping() -> RawJson<String> {
    let time = chrono::Utc::now();

    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    let result_json = format!(
        r#"{{
        "cpu": "{:.2}%",
        "used_memory": "{:.2} MB",
        "total_memory": "{:.2} MB",
        "status": "ok"
        }}"#,
        sys.global_cpu_usage(),
        sys.used_memory() as f32 / 1024.0 / 1024.0,
        sys.total_memory() as f32 / 1024.0 / 1024.0
    );
    content::RawJson(result_json)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ping])
        .mount("/ping", routes![ping])
        .mount("/", routes![login])
}
