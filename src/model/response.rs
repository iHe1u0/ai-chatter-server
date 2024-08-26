use rocket::serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<'a> {
    pub code: usize,
    pub msg: Cow<'a, str>,
}
