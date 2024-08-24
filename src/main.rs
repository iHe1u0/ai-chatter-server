#[macro_use]
extern crate rocket;

mod user;
mod response;

use std::borrow::Cow;

use crate::response::Response;
use crate::user::User;
use rocket::serde::json::{json, Value};
use rocket::{response::content::RawJson, serde::json::Json};

#[post("/login", format = "json", data = "<user>")]
fn login(user: Json<User>) -> Json<Response<'static>> {
    // 直接使用从表单中提取的 User 实例
    let user = User {
        id: None,
        account: user.account.clone(),
        password: user.password.clone(),
        exist: None,
    };
    let response = if user.validate() {
        Response {
            code: 0,
            msg: Cow::Borrowed("Authenticated"),
        }
    } else {
        Response {
            code: 401,
            msg: Cow::Borrowed("Authentication Failed"),
        }
    };

    Json(response)
}

#[get("/")]
fn ping() -> RawJson<String> {
    let admin = User::new(0, String::from("admin"), String::from("123"), true);
    admin.validate();

    let time = chrono::Utc::now();

    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    let result_json = format!(
        r#"{{
        "time":"{}",
        "cpu": "{:.2}%",
        "used_memory": "{:.2} MB",
        "total_memory": "{:.2} MB",
        "status": "ok"
        }}"#,
        time.to_string(),
        sys.global_cpu_usage(),
        sys.used_memory() as f32 / 1024.0 / 1024.0,
        sys.total_memory() as f32 / 1024.0 / 1024.0
    );
    RawJson(result_json)
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ping,login]).register("/", catchers![not_found])
}
