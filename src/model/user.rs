pub mod model {

    #[derive(Debug, Serialize, Deserialize, FromForm)]
    pub struct User {
        id: usize,
        account: &'a str,
        password: &'a str,
        exist: bool,
    }
}
